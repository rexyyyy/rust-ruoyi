//! 实例开启、分页查询、流程图。对应 Java `InsServiceImpl` + `ChartServiceImpl` + 各 service 的 list。

use serde_json::{Map, Value};

use super::flow::merge_variable;
use super::node::{end_node, get_next_node_list, start_node};
use super::FlowEngine;
use crate::chart;
use crate::constant::{exception, listener};
use crate::dto::{DefJson, FlowParams, PathWayData};
use crate::entity::{Definition, HisTask, Instance, Task};
use crate::enums::{ActivityStatus, FlowStatus, NodeType, SkipType};
use crate::error::{FlowError, Result};
use crate::store::{DefinitionQuery, HisTaskQuery, InstanceQuery, PageData, TaskQuery};
use crate::utils::empty_default;

impl FlowEngine {
    // ------------------------------------------------------------------
    // 开启流程
    // ------------------------------------------------------------------

    /// 开启流程（对应 Java `InsServiceImpl#start`）
    ///
    /// 会依据已发布的最新版流程定义，从开始节点一路走到第一个真实节点，
    /// 生成实例、开始节点的历史记录以及第一批待办任务。
    pub async fn start(&self, business_id: &str, mut flow_params: FlowParams) -> Result<Instance> {
        let flow_code = flow_params
            .flow_code
            .clone()
            .filter(|c| !c.is_empty())
            .ok_or_else(|| FlowError::biz(exception::NULL_FLOW_CODE))?;
        if business_id.is_empty() {
            return Err(FlowError::biz(exception::NULL_BUSINESS_ID));
        }
        let definition = self.get_publish_by_flow_code(&flow_code).await?;
        let combine = self.flow_combine(&definition).await?;
        let start_node = start_node(&combine)?;
        if !definition.is_activity() {
            return Err(FlowError::biz(exception::NOT_DEFINITION_ACTIVITY));
        }
        flow_params.skip_type = Some(SkipType::PASS.to_string());

        self.notify_listener(
            listener::LISTENER_START,
            self.listener_ctx(&definition, None, Some(&start_node), None, &flow_params.variable),
        );

        let mut path_way = PathWayData {
            def_id: definition.id,
            skip_type: Some(SkipType::PASS.to_string()),
            ..Default::default()
        };
        let next_nodes = get_next_node_list(
            &start_node,
            None,
            SkipType::PASS,
            &flow_params.variable,
            &mut path_way,
            &combine,
        )?;
        let first = next_nodes.first().cloned().ok_or_else(|| FlowError::biz(exception::NOT_NODE_DATA))?;

        let now = self.now();
        let mut instance = Instance {
            id: Some(self.next_id()),
            definition_id: first.definition_id,
            business_id: business_id.to_string(),
            node_type: first.node_type,
            node_code: first.node_code.clone(),
            node_name: first.node_name.clone(),
            flow_status: empty_default(flow_params.flow_status.as_deref(), FlowStatus::TO_BE_SUBMIT),
            activity_status: ActivityStatus::ACTIVITY.0,
            variable: Some(flow_params.variable_str()),
            create_time: Some(now),
            update_time: Some(now),
            create_by: Some(flow_params.handler.clone()),
            update_by: Some(flow_params.handler.clone()),
            ext: flow_params.ext.clone(),
            ..Default::default()
        };

        // 开始节点本身不产生待办，只留一条历史
        let start_task = Task {
            id: Some(self.next_id()),
            instance_id: instance.id.unwrap_or_default(),
            definition_id: definition.id.unwrap_or_default(),
            node_code: start_node.node_code.clone(),
            node_name: start_node.node_name.clone(),
            node_type: start_node.node_type,
            flow_status: FlowStatus::TO_BE_SUBMIT.to_string(),
            create_time: Some(now),
            create_by: Some(flow_params.handler.clone()),
            ..Default::default()
        };
        let mut his = self.build_skip_his(&start_task, &next_nodes, &flow_params);
        his.node_code = Some(start_node.node_code.clone());
        his.node_name = start_node.node_name.clone();
        his.node_type = Some(start_node.node_type);

        let mut add_tasks: Vec<Task> = next_nodes
            .iter()
            .map(|node| self.build_task(node, &instance, &definition, &flow_params))
            .collect();
        if !add_tasks.is_empty() {
            self.eval_task_permissions(&mut add_tasks, &flow_params.variable, &flow_params);
        }

        // 流程图元数据：以设计器数据为底，按本次途径打状态
        let mut next_path_way = path_way;
        next_path_way.target_nodes.extend(next_nodes.iter().cloned());
        let mut def_json = self.query_design(definition.id.unwrap_or_default()).await?;
        chart::apply_start_metadata(&mut def_json, &next_path_way);
        instance.def_json = Some(chart::dump_def_json(&def_json));

        self.notify_listener(
            listener::LISTENER_ASSIGNMENT,
            self.listener_ctx(&definition, Some(&instance), Some(&start_node), None, &flow_params.variable),
        );

        // 保存：历史 + 待办 + 办理人 + 实例
        self.store.insert_his_tasks(&mut [his]).await?;
        let users = self.task_add_users(&mut add_tasks);
        self.set_ins_finish_info(&mut instance, &mut add_tasks, &flow_params);
        if !add_tasks.is_empty() {
            self.store.insert_tasks(&mut add_tasks).await?;
        }
        if !users.is_empty() {
            let mut users = users;
            self.store.insert_users(&mut users).await?;
        }
        self.store.insert_instance(&mut instance).await?;

        self.notify_listener(
            listener::LISTENER_FINISH,
            self.listener_ctx(&definition, Some(&instance), Some(&start_node), None, &flow_params.variable),
        );
        self.notify_listener(
            listener::LISTENER_CREATE,
            self.listener_ctx(&definition, Some(&instance), Some(&start_node), None, &flow_params.variable),
        );
        Ok(instance)
    }

    /// 重新开启一个已终止 / 已撤销的实例摘要（仅刷新变量合并结果）
    pub async fn merge_instance_variable(&self, instance_id: i64, variable: &Map<String, Value>) -> Result<Instance> {
        let mut instance = self
            .store
            .get_instance(instance_id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_INSTANCE))?;
        let merged = merge_variable(&instance, variable);
        instance.set_variable_map(&merged);
        instance.update_time = Some(self.now());
        instance.update_by = self.operator();
        self.store.update_instance(&instance).await?;
        Ok(instance)
    }

    /// 删除流程变量
    pub async fn remove_variables(&self, instance_id: i64, keys: &[String]) -> Result<()> {
        let mut instance = self
            .store
            .get_instance(instance_id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_INSTANCE))?;
        let mut variable = instance.variable_map();
        for key in keys {
            variable.remove(key);
        }
        instance.set_variable_map(&variable);
        self.store.update_instance(&instance).await
    }

    /// 激活流程实例
    pub async fn active_instance(&self, id: i64) -> Result<()> {
        let mut instance = self
            .store
            .get_instance(id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_INSTANCE))?;
        if ActivityStatus::is_activity(instance.activity_status) {
            return Err(FlowError::biz(exception::INSTANCE_ALREADY_ACTIVITY));
        }
        instance.activity_status = ActivityStatus::ACTIVITY.0;
        instance.update_time = Some(self.now());
        self.store.update_instance(&instance).await
    }

    /// 挂起流程实例
    pub async fn un_active_instance(&self, id: i64) -> Result<()> {
        let mut instance = self
            .store
            .get_instance(id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_INSTANCE))?;
        if ActivityStatus::is_suspended(instance.activity_status) {
            return Err(FlowError::biz(exception::INSTANCE_ALREADY_ACTIVITY));
        }
        instance.activity_status = ActivityStatus::SUSPENDED.0;
        instance.update_time = Some(self.now());
        self.store.update_instance(&instance).await
    }

    /// 删除实例及其待办 / 办理人 / 历史
    pub async fn remove_instance(&self, ids: &[i64]) -> Result<()> {
        if ids.is_empty() {
            return Err(FlowError::biz(exception::NULL_INSTANCE_ID));
        }
        for id in ids {
            let instance = self
                .store
                .get_instance(*id)
                .await?
                .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_INSTANCE))?;
            let definition = self
                .store
                .get_definition(instance.definition_id)
                .await?
                .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_DEF))?;
            if !self.judge_activity_status(&definition, &instance) {
                return Err(FlowError::biz(exception::NOT_ACTIVITY));
            }
        }
        for id in ids {
            let tasks = self.store.list_tasks_by_instance(*id).await?;
            if !tasks.is_empty() {
                self.remove_and_user(&tasks).await?;
            }
        }
        self.store.delete_tasks_by_instances(ids).await?;
        self.store.delete_his_tasks_by_instances(ids).await?;
        self.store.delete_instances(ids).await
    }

    // ------------------------------------------------------------------
    // 查询
    // ------------------------------------------------------------------

    /// 流程定义列表
    pub async fn list_definitions(&self, query: &DefinitionQuery) -> Result<PageData<Definition>> {
        self.store.list_definitions(query).await
    }

    /// 流程实例列表
    pub async fn list_instances(&self, query: &InstanceQuery) -> Result<PageData<Instance>> {
        self.store.list_instances(query).await
    }

    /// 待办任务列表
    pub async fn list_tasks(&self, query: &TaskQuery) -> Result<PageData<Task>> {
        self.store.list_tasks(query).await
    }

    /// 历史任务列表
    pub async fn list_his_tasks(&self, query: &HisTaskQuery) -> Result<PageData<HisTask>> {
        self.store.list_his_tasks(query).await
    }

    /// 某实例的全部待办
    pub async fn list_tasks_by_instance(&self, instance_id: i64) -> Result<Vec<Task>> {
        self.store.list_tasks_by_instance(instance_id).await
    }

    /// 某实例的全部历史任务
    pub async fn list_his_tasks_by_instance(&self, instance_id: i64) -> Result<Vec<HisTask>> {
        self.store.list_his_tasks_by_instance(instance_id).await
    }

    /// 流程图（带办理状态）。实例上没存元数据时回退到设计器数据。
    pub async fn query_flow_chart(&self, instance_id: i64) -> Result<DefJson> {
        let instance = self
            .store
            .get_instance(instance_id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_INSTANCE))?;
        let mut def_json = chart::parse_def_json(instance.def_json.as_deref());
        if def_json.node_list.is_empty() {
            def_json = self.query_design(instance.definition_id).await?;
        }
        def_json.instance = Some(instance);
        Ok(def_json)
    }

    /// 流程图三原色，按模型区分经典 / 仿钉钉
    pub fn chart_rgb(&self, model_value: &str) -> Vec<String> {
        chart::chart_rgb(self.config.rgb_of(model_value))
    }

    /// 校验流程定义存在可用的开始 / 结束节点
    pub async fn check_flow_complete(&self, definition_id: i64) -> Result<(crate::entity::Node, crate::entity::Node)> {
        let combine = self.flow_combine_no_def(definition_id).await?;
        Ok((start_node(&combine)?, end_node(&combine)?))
    }

    /// 判断实例是否已结束
    pub fn is_finished(instance: &Instance) -> bool {
        NodeType::is_end(instance.node_type) || FlowStatus::is_finished(&instance.flow_status)
    }
}
