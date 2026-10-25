//! 流程引擎。对应 Java `FlowEngine` + `service/impl/*`。
//!
//! 按职责拆到多个子模块：
//! - `mod`：引擎装配、公共辅助（历史任务构造、实例状态刷新等）；
//! - [`definition`]：流程定义的保存 / 导入导出 / 发布 / 复制；
//! - [`node`]：节点流转与网关条件判定；
//! - [`flow`]：审批动作（通过、退回、撤销、终止、转办、委派、加减签、拿回、暂存）；
//! - [`query`]：分页查询、流程图、表单读取。

mod definition;
mod flow;
mod node;
mod query;

use std::sync::Arc;

use chrono::NaiveDateTime;
use serde_json::{Map, Value};

use crate::constant::{exception, flow as flow_cons, listener};
use crate::dto::{DefJson, FlowParams};
use crate::entity::{Definition, HisTask, Instance, Node, Task, User};
use crate::enums::{ActivityStatus, CooperateType, FlowStatus, NodeType, SkipType, UserType};
use crate::error::{FlowError, Result};
use crate::handler::{
    DataFillHandler, DefaultDataFillHandler, GlobalListener, ListenerContext, NoopGlobalListener,
    NoopPermissionHandler, NoopTenantHandler, PermissionHandler, TenantHandler, WarmFlowConfig,
};
use crate::store::WarmStore;
use crate::utils::empty_default;

pub use node::{get_ext, FlowCombineExt};

/// 流程引擎
#[derive(Clone)]
pub struct FlowEngine {
    pub(crate) store: Arc<dyn WarmStore>,
    pub(crate) config: WarmFlowConfig,
    pub(crate) fill: Arc<dyn DataFillHandler>,
    pub(crate) permission: Arc<dyn PermissionHandler>,
    pub(crate) tenant: Arc<dyn TenantHandler>,
    pub(crate) global_listener: Arc<dyn GlobalListener>,
}

/// 一次审批动作所需的上下文（对应 Java `TaskServiceImpl.R`）
pub(crate) struct FlowContext {
    pub instance: Instance,
    pub definition: Definition,
    pub now_node: Node,
    pub task: Task,
}

impl FlowEngine {
    /// 使用默认配置与默认处理器创建引擎
    pub fn new(store: Arc<dyn WarmStore>) -> Self {
        Self {
            store,
            config: WarmFlowConfig::default(),
            fill: Arc::new(DefaultDataFillHandler::default()),
            permission: Arc::new(NoopPermissionHandler),
            tenant: Arc::new(NoopTenantHandler),
            global_listener: Arc::new(NoopGlobalListener),
        }
    }

    pub fn with_config(mut self, config: WarmFlowConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_fill(mut self, fill: Arc<dyn DataFillHandler>) -> Self {
        self.fill = fill;
        self
    }

    pub fn with_permission(mut self, permission: Arc<dyn PermissionHandler>) -> Self {
        self.permission = permission;
        self
    }

    pub fn with_tenant(mut self, tenant: Arc<dyn TenantHandler>) -> Self {
        self.tenant = tenant;
        self
    }

    pub fn with_global_listener(mut self, listener: Arc<dyn GlobalListener>) -> Self {
        self.global_listener = listener;
        self
    }

    pub fn config(&self) -> &WarmFlowConfig {
        &self.config
    }

    pub fn store(&self) -> &Arc<dyn WarmStore> {
        &self.store
    }

    // ------------------------------------------------------------------
    // 公共辅助
    // ------------------------------------------------------------------

    /// 生成主键
    pub(crate) fn next_id(&self) -> i64 {
        self.fill.id()
    }

    /// 当前时间
    pub(crate) fn now(&self) -> NaiveDateTime {
        self.fill.now()
    }

    /// 当前操作人
    pub(crate) fn operator(&self) -> Option<String> {
        self.fill.operator()
    }

    /// 当前租户
    pub(crate) fn tenant_id(&self) -> Option<String> {
        self.fill.tenant_id().or_else(|| self.tenant.tenant_id())
    }

    /// 合并流程变量：实例变量 + 入参变量
    pub(crate) fn merge_variable(&self, instance: &mut Instance, variable: &Map<String, Value>) {
        if variable.is_empty() {
            return;
        }
        let mut merged = instance.variable_map();
        merged.extend(variable.clone());
        instance.set_variable_map(&merged);
    }

    /// 查询任务、实例、定义、当前节点并做通用校验（对应 Java `getAndCheck`）
    pub(crate) async fn get_and_check(&self, task: Task) -> Result<FlowContext> {
        let instance = self
            .store
            .get_instance(task.instance_id)
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
        let now_node = self
            .store
            .get_node(definition.id.unwrap_or_default(), &task.node_code)
            .await?
            .ok_or_else(|| FlowError::biz(exception::LOST_CUR_NODE))?;
        Ok(FlowContext { instance, definition, now_node, task })
    }

    /// 定义与实例都必须处于激活状态
    pub(crate) fn judge_activity_status(&self, definition: &Definition, instance: &Instance) -> bool {
        ActivityStatus::is_activity(definition.activity_status) && ActivityStatus::is_activity(instance.activity_status)
    }

    /// 构造待办任务（对应 Java `TaskServiceImpl#addTask`）
    pub(crate) fn build_task(
        &self,
        node: &Node,
        instance: &Instance,
        definition: &Definition,
        flow_params: &FlowParams,
    ) -> Task {
        let (form_custom, form_path) = node.form_of(definition);
        let flow_status = empty_default(flow_params.flow_status.as_deref(), &self.default_flow_status(node.node_type, flow_params.skip_type.as_deref().unwrap_or("")));
        Task {
            id: Some(self.next_id()),
            definition_id: instance.definition_id,
            instance_id: instance.id.unwrap_or_default(),
            node_code: node.node_code.clone(),
            node_name: node.node_name.clone(),
            node_type: node.node_type,
            flow_status,
            form_custom,
            form_path,
            create_time: Some(self.now()),
            create_by: self.operator(),
            tenant_id: self.tenant_id(),
            permission_list: crate::utils::str2list(node.permission_flag.as_deref().unwrap_or(""), flow_cons::SPLIT_AT),
            user_list: Vec::new(),
            ..Default::default()
        }
    }

    /// 按节点类型与跳转类型推导默认流程状态
    pub(crate) fn default_flow_status(&self, node_type: i32, skip_type: &str) -> String {
        if NodeType::is_start(node_type) {
            FlowStatus::TO_BE_SUBMIT.to_string()
        } else if NodeType::is_end(node_type) {
            FlowStatus::FINISHED.to_string()
        } else if SkipType::is_reject(skip_type) {
            FlowStatus::REJECT.to_string()
        } else {
            FlowStatus::APPROVAL.to_string()
        }
    }

    /// 构造办理人权限记录
    pub(crate) fn structure_user(&self, associated: i64, processed_by: &str, user_type: &str) -> User {
        User {
            id: Some(self.next_id()),
            r#type: user_type.to_string(),
            processed_by: Some(processed_by.to_string()),
            associated,
            create_time: Some(self.now()),
            create_by: self.operator(),
            tenant_id: self.tenant_id(),
            ..Default::default()
        }
    }

    /// 为一批任务生成办理人权限（对应 Java `UserServiceImpl#taskAddUsers`）
    pub(crate) fn task_add_users(&self, tasks: &mut [Task]) -> Vec<User> {
        let mut users = Vec::new();
        for task in tasks.iter_mut() {
            let task_id = task.id.unwrap_or_default();
            let task_users: Vec<User> = task
                .permission_list
                .iter()
                .map(|p| self.structure_user(task_id, p, UserType::APPROVAL))
                .collect();
            task.user_list = task_users.clone();
            users.extend(task_users);
        }
        users
    }

    /// 删除任务及其办理人
    pub(crate) async fn remove_and_user(&self, tasks: &[Task]) -> Result<()> {
        let ids: Vec<i64> = tasks.iter().filter_map(|t| t.id).collect();
        if ids.is_empty() {
            return Ok(());
        }
        self.store.delete_tasks(&ids).await?;
        self.store.delete_users_by_tasks(&ids).await?;
        Ok(())
    }

    /// 判断当前处理人是否有权办理（对应 Java `checkAuth`）
    pub(crate) fn check_auth(&self, task: &Task, flow_params: &FlowParams) -> Result<()> {
        if flow_params.ignore {
            return Ok(());
        }
        let permissions: Vec<String> = task
            .user_list
            .iter()
            .filter_map(|u| u.processed_by.clone())
            .collect();
        if !permissions.is_empty()
            && (flow_params.permission_flag.is_empty()
                || crate::utils::not_contains_any(&flow_params.permission_flag, &permissions))
        {
            return Err(FlowError::biz(exception::NULL_ROLE_NODE));
        }
        Ok(())
    }

    /// 流程变量合并 + 办理人表达式替换（对应 Java `ExpressionUtil.evalVariable`）
    pub(crate) fn eval_task_permissions(&self, tasks: &mut [Task], variable: &Map<String, Value>, flow_params: &FlowParams) {
        for task in tasks.iter_mut() {
            let mut permissions: Vec<String> = Vec::new();
            for expr in &task.permission_list {
                for p in crate::expression::eval_variable(expr, variable) {
                    if !permissions.contains(&p) {
                        permissions.push(p);
                    }
                }
            }
            if !permissions.is_empty() {
                permissions = self.permission.convert_permissions(permissions);
            }
            // 指定下个任务办理人
            if !flow_params.next_handler.is_empty() {
                if flow_params.next_handler_append {
                    for h in &flow_params.next_handler {
                        if !permissions.contains(h) {
                            permissions.push(h.clone());
                        }
                    }
                } else {
                    permissions = flow_params.next_handler.clone();
                }
            }
            task.permission_list = permissions;
        }
    }

    /// 历史任务：跳转记录（对应 Java `HisTaskServiceImpl#setSkipHis`）
    pub(crate) fn build_skip_his(&self, task: &Task, next_nodes: &[Node], flow_params: &FlowParams) -> HisTask {
        let skip_type = flow_params.skip_type.clone().unwrap_or_else(|| SkipType::NONE.to_string());
        let mut his = self.base_his(task, flow_params, &skip_type);
        his.target_node_code = Some(crate::utils::join(next_nodes.iter().map(|n| n.node_code.as_str()), flow_cons::COMMA));
        his.target_node_name = Some(crate::utils::join(
            next_nodes.iter().map(|n| n.node_name.clone().unwrap_or_default()),
            flow_cons::COMMA,
        ));
        his
    }

    /// 历史任务：会签 / 票签记录
    pub(crate) fn build_sign_his(
        &self,
        task: &Task,
        flow_params: &FlowParams,
        node_ratio: &str,
        is_pass: bool,
    ) -> HisTask {
        let cooperate = if CooperateType::is_countersign(node_ratio) {
            CooperateType::COUNTERSIGN.0
        } else {
            CooperateType::VOTE.0
        };
        let mut his = self.base_his(
            task,
            flow_params,
            if is_pass { SkipType::PASS } else { SkipType::REJECT },
        );
        his.cooperate_type = cooperate;
        his
    }

    /// 历史任务：转办 / 委派 / 加减签记录
    pub(crate) fn build_cooperate_his(
        &self,
        task: &Task,
        flow_params: &FlowParams,
        collaborators: &[String],
    ) -> HisTask {
        let mut his = self.base_his(task, flow_params, SkipType::NONE);
        his.cooperate_type = flow_params.cooperate_type.unwrap_or(CooperateType::APPROVAL.0);
        his.collaborator = Some(collaborators.join(flow_cons::COMMA));
        his.target_node_code = Some(task.node_code.clone());
        his.target_node_name = task.node_name.clone();
        his
    }

    /// 历史任务：委派记录
    pub(crate) fn build_depute_his(&self, task: &Task, flow_params: &FlowParams, entrusted: &User) -> HisTask {
        let skip_type = flow_params.skip_type.clone().unwrap_or_else(|| SkipType::NONE.to_string());
        let mut his = self.base_his(task, flow_params, &skip_type);
        his.cooperate_type = CooperateType::DEPUTE.0;
        his.collaborator = entrusted.create_by.clone();
        his
    }

    /// 历史任务：无跳转（暂存）
    pub(crate) fn build_pending_his(&self, task: &Task, flow_params: &FlowParams) -> HisTask {
        let mut his = self.base_his(task, flow_params, SkipType::NONE);
        his.target_node_code = Some(task.node_code.clone());
        his.target_node_name = task.node_name.clone();
        his
    }

    /// 历史任务状态：优先取入参 hisStatus / flowStatus，其次按跳转类型推导
    pub(crate) fn his_flow_status(&self, flow_params: &FlowParams, skip_type: &str) -> String {
        let explicit = empty_default(
            flow_params.his_status.as_deref().or(flow_params.flow_status.as_deref()),
            "",
        );
        if !explicit.is_empty() {
            return explicit;
        }
        if SkipType::is_reject(skip_type) {
            FlowStatus::REJECT.to_string()
        } else {
            FlowStatus::PASS.to_string()
        }
    }

    fn base_his(&self, task: &Task, flow_params: &FlowParams, skip_type: &str) -> HisTask {
        let flow_status = self.his_flow_status(flow_params, skip_type);
        HisTask {
            id: Some(self.next_id()),
            definition_id: task.definition_id,
            instance_id: task.instance_id,
            task_id: task.id.unwrap_or_default(),
            node_code: Some(task.node_code.clone()),
            node_name: task.node_name.clone(),
            node_type: Some(task.node_type),
            target_node_code: None,
            target_node_name: None,
            approver: Some(flow_params.handler.clone()),
            cooperate_type: flow_params.cooperate_type.unwrap_or(CooperateType::APPROVAL.0),
            collaborator: None,
            skip_type: skip_type.to_string(),
            flow_status,
            form_custom: task.form_custom.clone(),
            form_path: task.form_path.clone(),
            message: flow_params.message.clone(),
            variable: Some(flow_params.variable_str()),
            ext: flow_params.his_task_ext.clone(),
            create_time: task.create_time.or_else(|| Some(self.now())),
            update_time: Some(self.now()),
            del_flag: None,
            tenant_id: self.tenant_id(),
        }
    }

    /// 设置实例完成后的节点信息（对应 Java `setInsFinishInfo`）
    pub(crate) fn set_ins_finish_info(&self, instance: &mut Instance, add_tasks: &mut Vec<Task>, flow_params: &FlowParams) {
        instance.update_time = Some(self.now());
        self.merge_variable(instance, &flow_params.variable);
        if add_tasks.is_empty() {
            return;
        }
        // 结束节点不产生待办，仅用于刷新实例状态
        if let Some(pos) = add_tasks.iter().position(|t| NodeType::is_end(t.node_type)) {
            let end_task = add_tasks.remove(pos);
            instance.node_type = end_task.node_type;
            instance.node_code = end_task.node_code;
            instance.node_name = end_task.node_name;
            instance.flow_status = end_task.flow_status;
            return;
        }
        let finally_task = if add_tasks.len() == 1 {
            add_tasks[0].clone()
        } else if let Some(end) = add_tasks.iter().find(|t| NodeType::is_end(t.node_type)) {
            end.clone()
        } else {
            add_tasks.iter().max_by_key(|t| t.id.unwrap_or_default()).cloned().unwrap_or_default()
        };
        instance.node_type = finally_task.node_type;
        instance.node_code = finally_task.node_code;
        instance.node_name = finally_task.node_name;
        instance.flow_status = finally_task.flow_status;
    }

    /// 流程结束时清理剩余待办（对应 Java `handUndoneTask`）
    pub(crate) async fn hand_undone_task(&self, instance: &Instance) -> Result<()> {
        if NodeType::is_end(instance.node_type) {
            let tasks = self.store.list_tasks_by_instance(instance.id.unwrap_or_default()).await?;
            if !tasks.is_empty() {
                self.remove_and_user(&tasks).await?;
            }
        }
        Ok(())
    }

    /// 触发监听器。
    ///
    /// Java 版支持在节点/定义上配置类路径动态加载，Rust 无运行期反射，
    /// 统一交给 [`GlobalListener`]，由业务方按 `listener_type` 分发。
    pub(crate) fn notify_listener(&self, listener_type: &str, ctx: ListenerContext) {
        self.global_listener.notify(listener_type, &ctx);
    }

    /// 构建监听器上下文
    pub(crate) fn listener_ctx(
        &self,
        definition: &Definition,
        instance: Option<&Instance>,
        node: Option<&Node>,
        task: Option<&Task>,
        variable: &Map<String, Value>,
    ) -> ListenerContext {
        ListenerContext {
            definition: Some(definition.clone()),
            instance: instance.cloned(),
            node: node.cloned(),
            task: task.cloned(),
            next_nodes: Vec::new(),
            next_tasks: Vec::new(),
            variable: variable.clone(),
        }
    }

    /// 加载表单（对应 Java `TaskServiceImpl#load`）
    pub async fn load(&self, task_id: i64, flow_params: &FlowParams) -> Result<crate::dto::FlowDto> {
        let task = self
            .store
            .get_task(task_id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_TASK))?;
        let ctx = self.get_and_check(task).await?;
        self.notify_listener(
            listener::LISTENER_FORM_LOAD,
            self.listener_ctx(&ctx.definition, Some(&ctx.instance), Some(&ctx.now_node), Some(&ctx.task), &flow_params.variable),
        );

        let mut dto = crate::dto::FlowDto::default();
        if flow_cons::FORM_CUSTOM_Y == ctx.now_node.form_custom.as_deref().unwrap_or("") {
            if let Some(form_id) = ctx.now_node.form_path.as_deref().and_then(|p| p.parse::<i64>().ok()) {
                dto.form = self.store.get_form(form_id).await?;
            }
        } else if ctx.now_node.form_custom.is_none()
            && flow_cons::FORM_CUSTOM_Y == ctx.definition.form_custom.as_deref().unwrap_or("")
        {
            if let Some(form_id) = ctx.definition.form_path.as_deref().and_then(|p| p.parse::<i64>().ok()) {
                dto.form = self.store.get_form(form_id).await?;
            }
        }
        dto.data = ctx.instance.variable_map().get(flow_cons::FORM_DATA).cloned();
        Ok(dto)
    }

    /// 加载历史表单（对应 Java `TaskServiceImpl#hisLoad`）
    pub async fn his_load(&self, his_task_id: i64, _flow_params: &FlowParams) -> Result<crate::dto::FlowDto> {
        let his = self
            .store
            .get_his_task(his_task_id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_FLOW_TASK))?;
        let definition = self
            .store
            .get_definition(his.definition_id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_DEF))?;
        let node_code = his.node_code.clone().unwrap_or_default();
        let now_node = self
            .store
            .get_node(his.definition_id, &node_code)
            .await?
            .ok_or_else(|| FlowError::biz(exception::LOST_CUR_NODE))?;

        let mut dto = crate::dto::FlowDto::default();
        if flow_cons::FORM_CUSTOM_Y == now_node.form_custom.as_deref().unwrap_or("") {
            if let Some(form_id) = his.form_path.as_deref().and_then(|p| p.parse::<i64>().ok()) {
                dto.form = self.store.get_form(form_id).await?;
            }
        } else if now_node.form_custom.is_none()
            && flow_cons::FORM_CUSTOM_Y == definition.form_custom.as_deref().unwrap_or("")
        {
            if let Some(form_id) = definition.form_path.as_deref().and_then(|p| p.parse::<i64>().ok()) {
                dto.form = self.store.get_form(form_id).await?;
            }
        }
        dto.data = his.variable_map().get(flow_cons::FORM_DATA).cloned();
        Ok(dto)
    }

    /// 暂存：不流转，仅记录历史与状态（对应 Java `pending`）
    pub async fn pending(&self, task_id: i64, flow_params: FlowParams) -> Result<Instance> {
        let task = self
            .store
            .get_task(task_id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_TASK))?;
        let mut flow_params = flow_params;
        flow_params.flow_status = Some(empty_default(flow_params.flow_status.as_deref(), FlowStatus::PENDING));
        let mut ctx = self.get_and_check(task).await?;

        self.notify_listener(
            listener::LISTENER_START,
            self.listener_ctx(&ctx.definition, Some(&ctx.instance), Some(&ctx.now_node), Some(&ctx.task), &flow_params.variable),
        );
        // 暂存同样校验办理权限
        ctx.task.user_list = self
            .store
            .list_users_by_task_and_types(
                ctx.task.id.unwrap_or_default(),
                &[UserType::APPROVAL.to_string(), UserType::TRANSFER.to_string(), UserType::DEPUTE.to_string()],
            )
            .await?;
        self.check_auth(&ctx.task, &flow_params)?;

        let mut his = self.build_pending_his(&ctx.task, &flow_params);
        his.flow_status = flow_params.flow_status.clone().unwrap_or_else(|| FlowStatus::PENDING.to_string());
        self.store.insert_his_tasks(&mut [his]).await?;

        ctx.instance.flow_status = flow_params.flow_status.clone().unwrap_or_else(|| FlowStatus::PENDING.to_string());
        self.store.update_instance(&ctx.instance).await?;

        self.notify_listener(
            listener::LISTENER_FINISH,
            self.listener_ctx(&ctx.definition, Some(&ctx.instance), Some(&ctx.now_node), Some(&ctx.task), &flow_params.variable),
        );
        Ok(ctx.instance)
    }
}

/// 便捷扩展：把设计器 JSON 转成 `FlowCombine`
impl FlowCombineExt for DefJson {
    fn to_flow_combine(&self) -> crate::dto::FlowCombine {
        self.to_combine()
    }
}
