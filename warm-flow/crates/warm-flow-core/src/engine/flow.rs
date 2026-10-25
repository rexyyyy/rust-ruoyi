//! 审批动作。对应 Java `TaskServiceImpl`。
//!
//! 主流程 `skip` 与 Java 版保持同样的步骤顺序：
//! 校验 → 委派处理 → 权限校验 → 会签票签 → 计算下一节点 → 网关判定 →
//! 并行网关收敛 → 生成待办 → 历史留痕 → 一票否决 → 清理残留待办 → 监听器。

use std::collections::{HashMap, HashSet};

use serde_json::{json, Map, Value};

use super::node::{
    get_next_by_check_gateway, get_next_node, get_next_node_list, prefix_or_suffix_nodes, start_node,
};
use super::FlowEngine;
use crate::chart;
use crate::constant::{exception, flow as flow_cons, listener};
use crate::dto::{FlowParams, PathWayData};
use crate::entity::{HisTask, Instance, Node, Task, User};
use crate::enums::{CooperateType, FlowStatus, NodeType, SkipType, UserType};
use crate::error::{FlowError, Result};
use crate::utils::empty_default;

impl FlowEngine {
    // ------------------------------------------------------------------
    // 审批入口
    // ------------------------------------------------------------------

    /// 通过
    pub async fn pass(&self, task_id: i64, flow_params: FlowParams) -> Result<Instance> {
        self.skip(task_id, flow_params.skip_type(SkipType::PASS)).await
    }

    /// 通过并指定目标节点
    pub async fn pass_at_will(&self, task_id: i64, node_code: &str, flow_params: FlowParams) -> Result<Instance> {
        self.skip(task_id, flow_params.node_code(node_code).skip_type(SkipType::PASS)).await
    }

    /// 退回
    pub async fn reject(&self, task_id: i64, flow_params: FlowParams) -> Result<Instance> {
        self.skip(task_id, flow_params.skip_type(SkipType::REJECT)).await
    }

    /// 退回并指定目标节点
    pub async fn reject_at_will(&self, task_id: i64, node_code: &str, flow_params: FlowParams) -> Result<Instance> {
        self.skip(task_id, flow_params.node_code(node_code).skip_type(SkipType::REJECT)).await
    }

    /// 按任务 id 审批
    pub async fn skip(&self, task_id: i64, flow_params: FlowParams) -> Result<Instance> {
        let task = self
            .store
            .get_task(task_id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_TASK))?;
        self.skip_task(task, flow_params).await
    }

    /// 按实例 id 审批（要求该实例只有一条待办）
    pub async fn skip_by_ins_id(&self, instance_id: i64, flow_params: FlowParams) -> Result<Instance> {
        let task = self.single_task(instance_id).await?;
        self.skip_task(task, flow_params).await
    }

    /// 审批主流程（对应 Java `TaskServiceImpl#skip`）
    pub async fn skip_task(&self, task: Task, mut flow_params: FlowParams) -> Result<Instance> {
        let mut ctx = self.get_and_check(task).await?;
        flow_params.variable = merge_variable(&ctx.instance, &flow_params.variable);

        // 非开始节点必须显式指定跳转类型
        if !NodeType::is_start(ctx.task.node_type) && flow_params.skip_type_or_empty().is_empty() {
            return Err(FlowError::biz(exception::NULL_CONDITION_VALUE));
        }
        ctx.task.user_list = self
            .store
            .list_users_by_task_and_types(
                ctx.task.id.unwrap_or_default(),
                &[
                    UserType::APPROVAL.to_string(),
                    UserType::TRANSFER.to_string(),
                    UserType::DEPUTE.to_string(),
                ],
            )
            .await?;
        let combine = self.flow_combine_no_def(ctx.instance.definition_id).await?;
        let skip_type = flow_params.skip_type_or_empty().to_string();

        self.notify_listener(
            listener::LISTENER_START,
            self.listener_ctx(&ctx.definition, Some(&ctx.instance), Some(&ctx.now_node), Some(&ctx.task), &flow_params.variable),
        );

        // 受托人处理任务：留一条委派记录并把任务还给委托人
        if !flow_params.ignore_depute && self.handle_depute(&ctx.task, &flow_params).await? {
            return Ok(ctx.instance);
        }

        self.check_auth(&ctx.task, &flow_params)?;

        // 或签 / 会签 / 票签：还没走完就直接返回
        if !flow_params.ignore_cooperate
            && self.cooperate(&ctx.now_node, &ctx.task, &flow_params).await?
        {
            return Ok(ctx.instance);
        }

        let mut path_way = PathWayData {
            def_id: Some(ctx.instance.definition_id),
            ins_id: Some(ctx.instance.id.unwrap_or_default()),
            skip_type: Some(skip_type.clone()),
            ..Default::default()
        };
        let mut next_nodes = get_next_node_list(
            &ctx.now_node,
            flow_params.node_code.as_deref(),
            &skip_type,
            &flow_params.variable,
            &mut path_way,
            &combine,
        )?;

        // 并行 / 包容网关：还有其他前置待办时先不生成新任务
        self.is_generate_new_task(&mut path_way, &ctx.instance, &mut next_nodes, &combine)
            .await?;
        path_way.target_nodes.extend(next_nodes.iter().cloned());
        ctx.instance.def_json = Some(refresh_chart(&ctx.instance, &path_way));

        let mut add_tasks: Vec<Task> = next_nodes
            .iter()
            .map(|node| self.build_task(node, &ctx.instance, &ctx.definition, &flow_params))
            .collect();
        // 办理人变量替换
        self.eval_task_permissions(&mut add_tasks, &flow_params.variable, &flow_params);

        self.notify_listener(
            listener::LISTENER_ASSIGNMENT,
            self.listener_ctx(&ctx.definition, Some(&ctx.instance), Some(&ctx.now_node), Some(&ctx.task), &flow_params.variable),
        );

        self.update_flow_info(&ctx.task, &mut ctx.instance, &mut add_tasks, &flow_params, &next_nodes)
            .await?;

        // 一票否决：退回后清理目标节点之后的待办
        if let Some(first) = next_nodes.first() {
            if SkipType::is_reject(&skip_type) {
                self.one_vote_veto(&ctx.task, &first.node_code, &combine).await?;
            }
        }

        self.hand_undone_task(&ctx.instance).await?;

        self.notify_listener(
            listener::LISTENER_FINISH,
            self.listener_ctx(&ctx.definition, Some(&ctx.instance), Some(&ctx.now_node), Some(&ctx.task), &flow_params.variable),
        );
        self.notify_listener(
            listener::LISTENER_CREATE,
            self.listener_ctx(&ctx.definition, Some(&ctx.instance), Some(&ctx.now_node), Some(&ctx.task), &flow_params.variable),
        );
        Ok(ctx.instance)
    }

    // ------------------------------------------------------------------
    // 撤销 / 终止 / 取回 / 拿回
    // ------------------------------------------------------------------

    /// 撤销流程（发起人撤回）：回到开始节点的下一个节点
    pub async fn revoke(&self, instance_id: i64, mut flow_params: FlowParams) -> Result<Instance> {
        flow_params.skip_type = Some(SkipType::REJECT.to_string());
        if flow_params.flow_status.as_deref().map(|s| s.is_empty()).unwrap_or(true) {
            flow_params.flow_status = Some(FlowStatus::CANCEL.to_string());
        }
        let mut instance = self
            .store
            .get_instance(instance_id)
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
        if NodeType::is_end(instance.node_type) {
            return Err(FlowError::biz(exception::FLOW_FINISH));
        }
        flow_params.variable = merge_variable(&instance, &flow_params.variable);

        let task_list = self.store.list_tasks_by_instance(instance_id).await?;
        let combine = self.flow_combine(&definition).await?;
        let node_map: HashMap<String, Node> =
            combine.all_nodes.iter().map(|n| (n.node_code.clone(), n.clone())).collect();

        for task in &task_list {
            let now_node = node_map.get(&task.node_code).cloned();
            self.notify_listener(
                listener::LISTENER_START,
                self.listener_ctx(&definition, Some(&instance), now_node.as_ref(), Some(task), &flow_params.variable),
            );
        }

        if !flow_params.ignore && instance.create_by.as_deref() != Some(flow_params.handler.as_str()) {
            return Err(FlowError::biz(exception::NOT_DEF_PROMOTER_NOT_CANCEL));
        }

        let start = start_node(&combine)?;
        let mut path_way = PathWayData {
            def_id: Some(instance.definition_id),
            ins_id: Some(instance_id),
            skip_type: Some(SkipType::REJECT.to_string()),
            ..Default::default()
        };
        let next_node = get_next_node(&start, None, SkipType::PASS, &mut path_way, &combine)?;
        let next_nodes =
            get_next_by_check_gateway(&flow_params.variable, &next_node, &mut path_way, &combine)?;
        path_way.target_nodes.extend(next_nodes.iter().cloned());
        instance.def_json = Some(refresh_chart(&instance, &path_way));

        let mut add_tasks: Vec<Task> = next_nodes
            .iter()
            .map(|node| self.build_task(node, &instance, &definition, &flow_params))
            .collect();
        self.eval_task_permissions(&mut add_tasks, &flow_params.variable, &flow_params);

        for task in &task_list {
            self.notify_listener(
                listener::LISTENER_ASSIGNMENT,
                self.listener_ctx(&definition, Some(&instance), node_map.get(&task.node_code), Some(task), &flow_params.variable),
            );
        }

        let mut his_list: Vec<HisTask> = task_list
            .iter()
            .map(|task| self.build_skip_his(task, &next_nodes, &flow_params))
            .collect();
        self.store.insert_his_tasks(&mut his_list).await?;
        self.remove_and_user(&task_list).await?;
        let users = self.task_add_users(&mut add_tasks);

        self.set_ins_finish_info(&mut instance, &mut add_tasks, &flow_params);
        if !add_tasks.is_empty() {
            self.store.insert_tasks(&mut add_tasks).await?;
        }
        self.store.update_instance(&instance).await?;
        if !users.is_empty() {
            let mut users = users;
            self.store.insert_users(&mut users).await?;
        }

        for task in &task_list {
            self.notify_listener(
                listener::LISTENER_FINISH,
                self.listener_ctx(&definition, Some(&instance), node_map.get(&task.node_code), Some(task), &flow_params.variable),
            );
            self.notify_listener(
                listener::LISTENER_CREATE,
                self.listener_ctx(&definition, Some(&instance), node_map.get(&task.node_code), Some(task), &flow_params.variable),
            );
        }
        Ok(instance)
    }

    /// 终止流程（按任务 id）
    pub async fn termination(&self, task_id: i64, flow_params: FlowParams) -> Result<Instance> {
        let task = self
            .store
            .get_task(task_id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_TASK))?;
        self.termination_task(task, flow_params).await
    }

    /// 终止流程（按实例 id，取第一条待办）
    pub async fn termination_by_ins_id(&self, instance_id: i64, flow_params: FlowParams) -> Result<Instance> {
        if instance_id == 0 {
            return Err(FlowError::biz(exception::NULL_INSTANCE_ID));
        }
        let task = self.single_task(instance_id).await?;
        self.termination_task(task, flow_params).await
    }

    /// 终止流程主体：跳过所有中间节点，直接落到结束节点
    pub async fn termination_task(&self, task: Task, mut flow_params: FlowParams) -> Result<Instance> {
        let mut ctx = self.get_and_check(task).await?;
        flow_params.skip_type = Some(SkipType::PASS.to_string());
        flow_params.variable = merge_variable(&ctx.instance, &flow_params.variable);

        ctx.task.user_list = self
            .store
            .list_users_by_task_and_types(
                ctx.task.id.unwrap_or_default(),
                &[
                    UserType::APPROVAL.to_string(),
                    UserType::TRANSFER.to_string(),
                    UserType::DEPUTE.to_string(),
                ],
            )
            .await?;
        self.notify_listener(
            listener::LISTENER_START,
            self.listener_ctx(&ctx.definition, Some(&ctx.instance), Some(&ctx.now_node), Some(&ctx.task), &flow_params.variable),
        );
        self.check_auth(&ctx.task, &flow_params)?;

        let end_node = self.get_end_node(ctx.instance.definition_id).await?;
        let path_way = PathWayData {
            def_id: Some(ctx.instance.definition_id),
            ins_id: Some(ctx.instance.id.unwrap_or_default()),
            skip_type: Some(SkipType::PASS.to_string()),
            target_nodes: vec![end_node.clone()],
            path_way_nodes: vec![ctx.now_node.clone()],
            ..Default::default()
        };
        ctx.instance.def_json = Some(refresh_chart(&ctx.instance, &path_way));

        let flow_status = empty_default(flow_params.flow_status.as_deref(), FlowStatus::TERMINATE);
        ctx.instance.node_type = end_node.node_type;
        ctx.instance.node_code = end_node.node_code.clone();
        ctx.instance.node_name = end_node.node_name.clone();
        ctx.instance.flow_status = flow_status.clone();
        ctx.instance.update_time = Some(self.now());
        flow_params.flow_status = Some(flow_status);

        let mut his = self.build_skip_his(&ctx.task, &[end_node], &flow_params);
        his.flow_status = ctx.instance.flow_status.clone();
        self.store.insert_his_tasks(&mut [his]).await?;
        self.store.update_instance(&ctx.instance).await?;
        self.store
            .delete_users_by_tasks(&[ctx.task.id.unwrap_or_default()])
            .await?;

        self.hand_undone_task(&ctx.instance).await?;
        self.notify_listener(
            listener::LISTENER_FINISH,
            self.listener_ctx(&ctx.definition, Some(&ctx.instance), Some(&ctx.now_node), Some(&ctx.task), &flow_params.variable),
        );
        Ok(ctx.instance)
    }

    /// 退回上一个任务（对应 Java `rejectLast`）
    pub async fn reject_last(&self, task_id: i64, flow_params: FlowParams) -> Result<Instance> {
        let task = self
            .store
            .get_task(task_id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_TASK))?;
        let mut flow_params = flow_params.skip_type(SkipType::REJECT);
        let his_list = self.store.list_his_tasks_by_instance(task.instance_id).await?;
        let target = task.node_code.clone();
        let last = his_list
            .into_iter()
            .filter(|h| h.target_node_code.as_deref().map(|s| !s.is_empty()).unwrap_or(false))
            .filter(|h| SkipType::is_pass(&h.skip_type))
            .filter(|h| {
                h.target_node_code
                    .as_deref()
                    .unwrap_or("")
                    .split(flow_cons::COMMA)
                    .any(|c| c.trim() == target)
            })
            .max_by_key(|h| h.id.unwrap_or_default())
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_LAST_TASK))?;
        flow_params.node_code = last.node_code.clone();
        self.skip_task(task, flow_params).await
    }

    /// 取回：把已办任务重新拉回到自己手上（对应 Java `taskBack`）
    pub async fn task_back(&self, task_id: i64, flow_params: FlowParams) -> Result<Instance> {
        let task = self
            .store
            .get_task(task_id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_TASK))?;
        let last = self.task_back_his(flow_params.clone(), task.instance_id).await?;
        let mut flow_params = flow_params;
        flow_params.node_code = last.node_code.clone();
        flow_params.skip_type = Some(SkipType::REJECT.to_string());
        flow_params.ignore = true;
        flow_params.ignore_depute = true;
        flow_params.ignore_cooperate = true;
        flow_params.flow_status = Some(empty_default(flow_params.flow_status.as_deref(), FlowStatus::TASK_BACK));
        self.skip_task(task, flow_params).await
    }

    /// 取回（按实例 id）
    pub async fn task_back_by_ins_id(&self, instance_id: i64, flow_params: FlowParams) -> Result<Instance> {
        let last = self.task_back_his(flow_params.clone(), instance_id).await?;
        let suffix = self.suffix_node_list(last.definition_id, &last.node_code.clone().unwrap_or_default()).await?;
        let codes: Vec<String> = suffix.iter().map(|n| n.node_code.clone()).collect();
        let tasks = self.store.list_tasks_by_instance_and_nodes(instance_id, &codes).await?;
        let task = tasks
            .into_iter()
            .next()
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_HANDLED_TASK_HANDLER))?;
        self.task_back(task.id.unwrap_or_default(), flow_params).await
    }

    async fn task_back_his(&self, flow_params: FlowParams, instance_id: i64) -> Result<HisTask> {
        let his_list = self.store.list_his_tasks_by_instance(instance_id).await?;
        his_list
            .into_iter()
            .filter(|h| h.approver.as_deref().map(|a| !a.is_empty()).unwrap_or(false))
            .filter(|h| SkipType::is_pass(&h.skip_type))
            .filter(|h| h.approver.as_deref() == Some(flow_params.handler.as_str()))
            .max_by_key(|h| h.id.unwrap_or_default())
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_HANDLED_TASK))
    }

    // ------------------------------------------------------------------
    // 转办 / 委派 / 加减签
    // ------------------------------------------------------------------

    /// 转办
    pub async fn transfer(&self, task_id: i64, mut flow_params: FlowParams) -> Result<bool> {
        if task_id == 0 {
            return Err(FlowError::biz(exception::NULL_TASK_ID));
        }
        if flow_params.handler.is_empty() {
            return Err(FlowError::biz(exception::HANDLER_NOT_EMPTY));
        }
        if flow_params.add_handlers.is_empty() {
            return Err(FlowError::biz(exception::NULL_TRANSFER_HANDLER));
        }
        let exists = self
            .store
            .list_users_by_task_and_types(task_id, &[UserType::TRANSFER.to_string()])
            .await?;
        if exists
            .iter()
            .any(|u| u.processed_by.as_deref().map(|p| flow_params.add_handlers.iter().any(|h| h == p)).unwrap_or(false))
        {
            return Err(FlowError::biz(exception::IS_ALREADY_TRANSFER));
        }
        flow_params.cooperate_type = Some(CooperateType::TRANSFER.0);
        flow_params.reduction_handlers = vec![flow_params.handler.clone()];
        self.update_handler(task_id, flow_params).await
    }

    /// 委派
    pub async fn depute(&self, task_id: i64, mut flow_params: FlowParams) -> Result<bool> {
        if task_id == 0 {
            return Err(FlowError::biz(exception::NULL_TASK_ID));
        }
        if flow_params.handler.is_empty() {
            return Err(FlowError::biz(exception::HANDLER_NOT_EMPTY));
        }
        if flow_params.add_handlers.is_empty() {
            return Err(FlowError::biz(exception::NULL_DEPUTE_HANDLER));
        }
        let exists = self
            .store
            .list_users_by_task_and_types(task_id, &[UserType::DEPUTE.to_string()])
            .await?;
        if exists
            .iter()
            .any(|u| u.processed_by.as_deref().map(|p| flow_params.add_handlers.iter().any(|h| h == p)).unwrap_or(false))
        {
            return Err(FlowError::biz(exception::IS_ALREADY_DEPUTE));
        }
        flow_params.cooperate_type = Some(CooperateType::DEPUTE.0);
        flow_params.reduction_handlers = vec![flow_params.handler.clone()];
        self.update_handler(task_id, flow_params).await
    }

    /// 加签
    pub async fn add_signature(&self, task_id: i64, mut flow_params: FlowParams) -> Result<bool> {
        if task_id == 0 {
            return Err(FlowError::biz(exception::NULL_TASK_ID));
        }
        if flow_params.handler.is_empty() {
            return Err(FlowError::biz(exception::HANDLER_NOT_EMPTY));
        }
        if flow_params.add_handlers.is_empty() {
            return Err(FlowError::biz(exception::NULL_ADD_SIGNATURE_HANDLER));
        }
        let exists = self
            .store
            .list_users_by_task_and_types(task_id, &[UserType::APPROVAL.to_string()])
            .await?;
        if exists
            .iter()
            .any(|u| u.processed_by.as_deref().map(|p| flow_params.add_handlers.iter().any(|h| h == p)).unwrap_or(false))
        {
            return Err(FlowError::biz(exception::IS_ALREADY_SIGN));
        }
        flow_params.cooperate_type = Some(CooperateType::ADD_SIGNATURE.0);
        self.update_handler(task_id, flow_params).await
    }

    /// 减签
    pub async fn reduction_signature(&self, task_id: i64, mut flow_params: FlowParams) -> Result<bool> {
        if task_id == 0 {
            return Err(FlowError::biz(exception::NULL_TASK_ID));
        }
        if flow_params.handler.is_empty() {
            return Err(FlowError::biz(exception::HANDLER_NOT_EMPTY));
        }
        if flow_params.reduction_handlers.is_empty() {
            return Err(FlowError::biz(exception::NULL_REDUCTION_SIGNATURE_HANDLER));
        }
        let users = self
            .store
            .list_users_by_task_and_types(
                task_id,
                &[UserType::APPROVAL.to_string(), UserType::TRANSFER.to_string()],
            )
            .await?;
        if users.len() <= 1 {
            return Err(FlowError::biz(exception::REDUCTION_SIGN_ONE_ERROR));
        }
        flow_params.cooperate_type = Some(CooperateType::REDUCTION_SIGNATURE.0);
        self.update_handler(task_id, flow_params).await
    }

    /// 变更办理人（转办 / 委派 / 加签 / 减签的公共实现）
    pub async fn update_handler(&self, task_id: i64, mut flow_params: FlowParams) -> Result<bool> {
        let task = self
            .store
            .get_task(task_id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_TASK))?;
        let ctx = self.get_and_check(task).await?;
        flow_params.variable = merge_variable(&ctx.instance, &flow_params.variable);
        self.notify_listener(
            listener::LISTENER_START,
            self.listener_ctx(&ctx.definition, Some(&ctx.instance), Some(&ctx.now_node), Some(&ctx.task), &flow_params.variable),
        );

        if !flow_params.ignore {
            let task_permissions: Vec<String> = self
                .store
                .list_users_by_task_and_types(
                    task_id,
                    &[
                        UserType::APPROVAL.to_string(),
                        UserType::TRANSFER.to_string(),
                        UserType::DEPUTE.to_string(),
                    ],
                )
                .await?
                .into_iter()
                .filter_map(|u| u.processed_by)
                .collect();
            if !task_permissions.is_empty()
                && (flow_params.permission_flag.is_empty()
                    || crate::utils::not_contains_any(&flow_params.permission_flag, &task_permissions))
            {
                return Err(FlowError::biz(exception::NOT_AUTHORITY));
            }
        }

        flow_params.skip_type = Some(SkipType::NONE.to_string());
        let mut his: Option<HisTask> = None;

        // 移除原有办理人
        if !flow_params.reduction_handlers.is_empty() {
            let users = self.store.list_users_by_task(task_id).await?;
            for reduction in &flow_params.reduction_handlers {
                for user in users.iter().filter(|u| u.processed_by.as_deref() == Some(reduction.as_str())) {
                    if let Some(id) = user.id {
                        self.store.delete_user(id).await?;
                    }
                }
            }
            his = Some(self.build_cooperate_his(&ctx.task, &flow_params, &flow_params.reduction_handlers));
        }

        // 新增办理人
        if !flow_params.add_handlers.is_empty() {
            let user_type = match flow_params.cooperate_type {
                Some(t) if t == CooperateType::TRANSFER.0 => UserType::TRANSFER,
                Some(t) if t == CooperateType::DEPUTE.0 => UserType::DEPUTE,
                _ => UserType::APPROVAL,
            };
            let mut users: Vec<User> = flow_params
                .add_handlers
                .iter()
                .map(|handler| {
                    self.structure_user_with(
                        task_id,
                        handler,
                        user_type,
                        Some(flow_params.handler.clone()),
                    )
                })
                .collect();
            self.store.insert_users(&mut users).await?;
            his = Some(self.build_cooperate_his(&ctx.task, &flow_params, &flow_params.add_handlers));
        }

        if let Some(his) = his {
            self.store.insert_his_tasks(&mut [his]).await?;
        }
        self.notify_listener(
            listener::LISTENER_FINISH,
            self.listener_ctx(&ctx.definition, Some(&ctx.instance), Some(&ctx.now_node), Some(&ctx.task), &flow_params.variable),
        );
        Ok(true)
    }

    // ------------------------------------------------------------------
    // 内部实现
    // ------------------------------------------------------------------

    /// 取实例下唯一的待办任务
    pub async fn single_task(&self, instance_id: i64) -> Result<Task> {
        let tasks = self.store.list_tasks_by_instance(instance_id).await?;
        if tasks.is_empty() {
            return Err(FlowError::biz(exception::NOT_FOUND_FLOW_TASK));
        }
        if tasks.len() > 1 {
            return Err(FlowError::biz(exception::TASK_NOT_ONE));
        }
        Ok(tasks.into_iter().next().unwrap_or_default())
    }

    /// 构造办理人记录并指定创建人
    pub(crate) fn structure_user_with(
        &self,
        associated: i64,
        processed_by: &str,
        user_type: &str,
        create_by: Option<String>,
    ) -> User {
        let mut user = self.structure_user(associated, processed_by, user_type);
        user.create_by = create_by.or_else(|| self.operator());
        user
    }

    /// 受托人处理任务（对应 Java `handleDepute`）
    async fn handle_depute(&self, task: &Task, flow_params: &FlowParams) -> Result<bool> {
        let entrusted = task
            .user_list
            .iter()
            .find(|u| u.r#type == UserType::DEPUTE && u.processed_by.as_deref() == Some(flow_params.handler.as_str()))
            .cloned();
        let Some(entrusted) = entrusted else {
            return Ok(false);
        };

        let his = self.build_depute_his(task, flow_params, &entrusted);
        self.store.insert_his_tasks(&mut [his]).await?;
        if let Some(id) = entrusted.id {
            self.store.delete_user(id).await?;
        }

        // 委托人若无待办权限则补一条
        let creator = entrusted.create_by.clone().unwrap_or_default();
        let exists = self
            .store
            .find_user(task.id.unwrap_or_default(), &creator, UserType::APPROVAL)
            .await?;
        if exists.is_none() {
            let user = self.structure_user_with(
                entrusted.associated,
                &creator,
                UserType::APPROVAL,
                entrusted.processed_by.clone(),
            );
            self.store.insert_users(&mut [user]).await?;
        }
        Ok(true)
    }

    /// 会签 / 票签 / 或签（对应 Java `cooperate`）
    ///
    /// 返回 true 表示本次审批只做了签署记录，流程还没流转。
    pub(crate) async fn cooperate(
        &self,
        now_node: &Node,
        task: &Task,
        flow_params: &FlowParams,
    ) -> Result<bool> {
        if flow_params.ignore {
            return Ok(false);
        }
        let node_ratio = now_node.ratio();
        if CooperateType::is_or_sign(&node_ratio) {
            return Ok(false);
        }

        let todo_list = self
            .store
            .list_users_by_task_and_types(
                task.id.unwrap_or_default(),
                &[
                    UserType::APPROVAL.to_string(),
                    UserType::TRANSFER.to_string(),
                    UserType::DEPUTE.to_string(),
                ],
            )
            .await?;
        if flow_params.handler.is_empty() {
            return Err(FlowError::biz(exception::SIGN_NULL_HANDLER));
        }
        let todo_user = todo_list
            .iter()
            .find(|u| u.processed_by.as_deref() == Some(flow_params.handler.as_str()))
            .cloned()
            .ok_or_else(|| FlowError::biz(exception::NOT_AUTHORITY))?;
        let rest: Vec<User> = todo_list
            .iter()
            .filter(|u| u.processed_by.as_deref() != Some(flow_params.handler.as_str()))
            .cloned()
            .collect();

        // 会签时有人退回，直接清空其他人
        if CooperateType::is_countersign(&node_ratio) && SkipType::is_reject(flow_params.skip_type_or_empty()) {
            return self.remove_rest_list(&rest).await;
        }

        let done_list = self.store.list_his_tasks_by_task(task.id.unwrap_or_default()).await?;
        let all_num = (todo_list.len() + done_list.len()) as i32;
        let done_pass: Vec<HisTask> = done_list
            .iter()
            .filter(|h| SkipType::is_pass(&h.skip_type))
            .cloned()
            .collect();
        let done_reject: Vec<HisTask> = done_list
            .iter()
            .filter(|h| SkipType::is_reject(&h.skip_type))
            .cloned()
            .collect();
        let is_pass = SkipType::is_pass(flow_params.skip_type_or_empty());

        if CooperateType::is_vote_sign_default(&node_ratio) || CooperateType::is_vote_sign_reject_spel(&node_ratio) {
            let mut variable: Map<String, Value> = flow_params.variable.clone();
            variable.insert("skipType".to_string(), json!(flow_params.skip_type_or_empty()));
            variable.insert("passNum".to_string(), json!(done_pass.len()));
            variable.insert("rejectNum".to_string(), json!(done_reject.len()));
            variable.insert("todoNum".to_string(), json!(todo_list.len()));
            variable.insert("allNum".to_string(), json!(all_num));
            variable.insert("passList".to_string(), serde_json::to_value(&done_pass).unwrap_or(Value::Null));
            variable.insert("rejectList".to_string(), serde_json::to_value(&done_reject).unwrap_or(Value::Null));
            variable.insert("todoList".to_string(), serde_json::to_value(&todo_list).unwrap_or(Value::Null));
            if crate::expression::eval_vote_sign(&node_ratio, &variable)? {
                return self.remove_rest_list(&rest).await;
            }
        } else {
            if all_num <= 0 {
                return self.remove_rest_list(&rest).await;
            }
            let pass_ratio = ((if is_pass { 1.0 } else { 0.0 }) + done_pass.len() as f64) / all_num as f64 * 100.0;
            let reject_ratio = ((if is_pass { 0.0 } else { 1.0 }) + done_reject.len() as f64) / all_num as f64 * 100.0;

            if CooperateType::is_vote_sign_pass_count(&node_ratio) {
                let pass_count = ratio_value(&node_ratio, "=");
                if (is_pass && done_pass.len() as i32 + 1 >= pass_count)
                    || (!is_pass && done_reject.len() as i32 + 1 > all_num - pass_count)
                {
                    return self.remove_rest_list(&rest).await;
                }
            } else if CooperateType::is_vote_sign_reject_count(&node_ratio) {
                let reject_count = ratio_value(&node_ratio, "=");
                if (!is_pass && done_reject.len() as i32 + 1 >= reject_count)
                    || (is_pass && done_pass.len() as i32 + 1 > all_num - reject_count)
                {
                    return self.remove_rest_list(&rest).await;
                }
            } else {
                let ratio: f64 = node_ratio.parse().unwrap_or(100.0);
                if (!is_pass && reject_ratio > 100.0 - ratio) || (is_pass && pass_ratio >= ratio) {
                    return self.remove_rest_list(&rest).await;
                }
            }
        }

        // 只剩自己一个人：由自己决定流向
        if todo_list.len() == 1 {
            return Ok(false);
        }

        let his = self.build_sign_his(task, flow_params, &node_ratio, is_pass);
        self.store.insert_his_tasks(&mut [his]).await?;
        if let Some(id) = todo_user.id {
            self.store.delete_user(id).await?;
        }
        Ok(true)
    }

    /// 删除剩余的办理人，返回 false（流程继续流转）
    async fn remove_rest_list(&self, rest: &[User]) -> Result<bool> {
        for user in rest {
            if let Some(id) = user.id {
                self.store.delete_user(id).await?;
            }
        }
        Ok(false)
    }

    /// 并行 / 包容网关收敛判断（对应 Java `isGenerateNewTask`）
    async fn is_generate_new_task(
        &self,
        path_way: &mut PathWayData,
        instance: &Instance,
        next_nodes: &mut Vec<Node>,
        combine: &crate::dto::FlowCombine,
    ) -> Result<()> {
        if path_way.skip_type.as_deref().map(SkipType::is_reject).unwrap_or(false) {
            return Ok(());
        }
        let gateways: Vec<Node> = path_way
            .path_way_nodes
            .iter()
            .filter(|n| NodeType::is_gateway_parallel(n.node_type) || NodeType::is_gateway_inclusive(n.node_type))
            .cloned()
            .collect();
        let Some(_) = gateways.first() else {
            return Ok(());
        };

        let def_json = chart::parse_def_json(instance.def_json.as_deref());
        let status_map: HashMap<String, i32> = def_json
            .node_list
            .iter()
            .map(|n| (n.node_code.clone(), n.status.unwrap_or(0)))
            .collect();

        let last_gateway = gateways.last().cloned().unwrap_or_default();
        let previous = prefix_or_suffix_nodes(&last_gateway.node_code, flow_cons::PREVIOUS, combine);
        let pending_count = previous
            .iter()
            .filter(|n| status_map.get(&n.node_code).copied().unwrap_or(0) == 1)
            .count();
        if pending_count <= 1 {
            return Ok(());
        }

        // 还有别的分支没办完：本次不生成任务，并把网关之后的部分从流程图元数据里裁掉
        next_nodes.clear();
        let first_code = gateways[0].node_code.clone();
        if let Some(pos) = path_way.path_way_nodes.iter().position(|n| n.node_code == first_code) {
            path_way.path_way_nodes.truncate(pos + 1);
        }
        if let Some(pos) = path_way.path_way_skips.iter().position(|s| s.now_node_code == first_code) {
            path_way.path_way_skips.truncate(pos);
        }
        Ok(())
    }

    /// 一票否决：退回后清掉目标节点之后的待办
    pub(crate) async fn one_vote_veto(
        &self,
        task: &Task,
        next_node_code: &str,
        combine: &crate::dto::FlowCombine,
    ) -> Result<()> {
        let tasks = self.store.list_tasks_by_instance(task.instance_id).await?;
        let suffix = prefix_or_suffix_nodes(next_node_code, flow_cons::SUFFIX, combine);
        let codes: HashSet<String> = suffix.into_iter().map(|n| n.node_code).collect();
        let no_done: Vec<Task> = tasks
            .into_iter()
            .filter(|t| codes.contains(&t.node_code) && t.id != task.id)
            .collect();
        if !no_done.is_empty() {
            self.remove_and_user(&no_done).await?;
        }
        Ok(())
    }

    /// 写入历史任务、删除当前待办、保存新待办（对应 Java `updateFlowInfo`）
    async fn update_flow_info(
        &self,
        task: &Task,
        instance: &mut Instance,
        add_tasks: &mut Vec<Task>,
        flow_params: &FlowParams,
        next_nodes: &[Node],
    ) -> Result<()> {
        let his = self.build_skip_his(task, next_nodes, flow_params);
        self.store.insert_his_tasks(&mut [his]).await?;
        self.remove_and_user(&[task.clone()]).await?;
        let users = self.task_add_users(add_tasks);

        self.set_ins_finish_info(instance, add_tasks, flow_params);
        if !add_tasks.is_empty() {
            self.store.insert_tasks(add_tasks).await?;
        }
        self.store.update_instance(instance).await?;
        if !users.is_empty() {
            let mut users = users;
            self.store.insert_users(&mut users).await?;
        }
        Ok(())
    }
}

/// 合并实例变量与入参变量
pub(crate) fn merge_variable(instance: &Instance, variable: &Map<String, Value>) -> Map<String, Value> {
    let mut merged = instance.variable_map();
    merged.extend(variable.clone());
    merged
}

/// 刷新实例里的流程图元数据
pub(crate) fn refresh_chart(instance: &Instance, path_way: &PathWayData) -> String {
    let mut def_json = chart::parse_def_json(instance.def_json.as_deref());
    chart::apply_skip_metadata(&mut def_json, path_way);
    chart::dump_def_json(&def_json)
}

/// 从 `passCount=3` 这类表达式里取出数字
fn ratio_value(expr: &str, sep: &str) -> i32 {
    expr.split_once(sep)
        .and_then(|(_, v)| v.trim().parse::<i32>().ok())
        .unwrap_or(i32::MAX)
}
