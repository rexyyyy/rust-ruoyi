//! 任务：发起、办理、待办 / 已办查询、终止、转办。
//!
//! 对标 Java `FlwTaskServiceImpl`。关键约定：
//! - `flowParams.handler` 必须是**用户 id 字符串**（引擎内部按字符串比较）；
//! - `permission_flag` 传当前用户的 id + 角色 + 部门标识，引擎用它和任务办理人求交集；
//! - 超管（role_id = 1）传 `ignore = true`，可以处理任意任务，否则演示时容易卡在
//!   「节点配的角色里没有 admin」上；
//! - 审核动作结束后显式回写业务表状态（见 [`super::biz`]）。

use std::collections::HashMap;

use serde_json::json;
use warm_flow_core::dto::FlowParams;
use warm_flow_core::enums::FlowStatus;
use warm_flow_core::entity::HisTask;
use warm_flow_core::store::{HisTaskQuery, TaskQuery};

use crate::error::{Error, Result};
use crate::modules::workflow::domain::bo::{
    CompleteTaskBo, StartRrocessBo, TaskOperationBo, TaskPageBo, TerminationBo,
};
use crate::modules::workflow::domain::vo::{HisTaskVO, StartReturnVO, TaskVO};
use crate::modules::workflow::domain::Page;
use crate::modules::workflow::service::{self, biz, flow_err};

// ----------------------------------------------------------------------
// 查询
// ----------------------------------------------------------------------

/// 我的待办
pub async fn page_todo(uid: i64, bo: &TaskPageBo) -> Result<Page<TaskVO>> {
    page_task(Some(uid), bo).await
}

/// 全部待办（管理员视角）
pub async fn page_all_todo(bo: &TaskPageBo) -> Result<Page<TaskVO>> {
    page_task(None, bo).await
}

/// 我的已办
pub async fn page_done(uid: i64, bo: &TaskPageBo) -> Result<Page<HisTaskVO>> {
    page_his(Some(uid), bo).await
}

/// 全部已办（管理员视角）
pub async fn page_all_done(bo: &TaskPageBo) -> Result<Page<HisTaskVO>> {
    page_his(None, bo).await
}

/// 实例当前第一条待办 id（业务侧发起后自动办结「申请人」时，用它回塞下一个任务）
pub async fn first_task_id(instance_id: i64) -> Option<i64> {
    let engine = service::engine().ok()?;
    engine
        .list_tasks_by_instance(instance_id)
        .await
        .ok()
        .and_then(|tasks| tasks.first().and_then(|t| t.id))
}

async fn page_task(handler_id: Option<i64>, bo: &TaskPageBo) -> Result<Page<TaskVO>> {
    let engine = service::engine()?;
    let page_no = bo.page_no.unwrap_or(1).max(1);
    let page_size = bo.page_size.unwrap_or(10).clamp(1, 200);
    let (definition_filter, code_ids) = flow_code_filter(bo.flow_code.as_deref()).await;

    let query = TaskQuery {
        instance_id: None,
        handler: handler_id.map(|id| id.to_string()),
        node_code: service::blank_to_none(bo.node_code.as_deref()),
        flow_status: service::blank_to_none(bo.flow_status.as_deref()),
        definition_id: definition_filter,
        page_num: page_no,
        page_size,
    };
    let mut data = engine.list_tasks(&query).await.map_err(flow_err)?;
    if !code_ids.is_empty() {
        data.rows.retain(|t| code_ids.contains(&t.definition_id));
        data.total = data.rows.len() as u64;
    }
    let ctx = instance_context(data.rows.iter().map(|t| (t.instance_id, t.definition_id))).await;
    let mut rows: Vec<TaskVO> = data
        .rows
        .into_iter()
        .map(|task| fill_task(TaskVO::from(task), &ctx))
        .collect();
    if let Some(business_id) = service::blank_to_none(bo.business_id.as_deref()) {
        rows.retain(|r| r.business_id.as_deref() == Some(business_id.as_str()));
    }
    Ok(Page::new(rows, data.total, page_no, page_size))
}

async fn page_his(approver_id: Option<i64>, bo: &TaskPageBo) -> Result<Page<HisTaskVO>> {
    let engine = service::engine()?;
    let page_no = bo.page_no.unwrap_or(1).max(1);
    let page_size = bo.page_size.unwrap_or(10).clamp(1, 200);
    let (definition_filter, code_ids) = flow_code_filter(bo.flow_code.as_deref()).await;

    let query = HisTaskQuery {
        instance_id: None,
        task_id: None,
        approver: approver_id.map(|id| id.to_string()),
        flow_status: service::blank_to_none(bo.flow_status.as_deref()),
        definition_id: definition_filter,
        page_num: page_no,
        page_size,
    };
    let mut data = engine.list_his_tasks(&query).await.map_err(flow_err)?;
    if !code_ids.is_empty() {
        data.rows.retain(|h| code_ids.contains(&h.definition_id));
        data.total = data.rows.len() as u64;
    }
    let ctx = instance_context(data.rows.iter().map(|h| (h.instance_id, h.definition_id))).await;
    let mut rows: Vec<HisTaskVO> = data
        .rows
        .into_iter()
        .map(|his| fill_his(HisTaskVO::from(his), &ctx))
        .collect();
    if let Some(business_id) = service::blank_to_none(bo.business_id.as_deref()) {
        rows.retain(|r| r.business_id.as_deref() == Some(business_id.as_str()));
    }
    Ok(Page::new(fill_approvers(rows).await, data.total, page_no, page_size))
}

/// 某实例的全部办理历史（详情页时间线）
pub async fn his_by_instance(instance_id: i64) -> Result<Vec<HisTaskVO>> {
    let engine = service::engine()?;
    let list: Vec<HisTask> = engine
        .list_his_tasks_by_instance(instance_id)
        .await
        .map_err(flow_err)?;
    let ctx = instance_context(list.iter().map(|h| (h.instance_id, h.definition_id))).await;
    Ok(fill_approvers(
        list
            .into_iter()
            .map(|his| fill_his(HisTaskVO::from(his), &ctx))
            .collect(),
    )
    .await)
}

/// 任务的办理人（用户 id 列表）
pub async fn task_users(task_id: i64) -> Result<Vec<String>> {
    let engine = service::engine()?;
    let users = engine
        .store()
        .list_users_by_task(task_id)
        .await
        .map_err(flow_err)?;
    Ok(users.into_iter().filter_map(|u| u.processed_by).collect())
}

// ----------------------------------------------------------------------
// 发起
// ----------------------------------------------------------------------

/// 发起流程。
///
/// 幂等：同一 `business_id` 已有实例时，直接返回其现有待办，不重复创建
/// （对应 Java `startWorkFlow` 的「已存在流程」分支）。
pub async fn start(uid: i64, bo: &StartRrocessBo) -> Result<StartReturnVO> {
    let engine = service::engine()?;
    let flow_code = service::blank_to_none(bo.flow_code.as_deref())
        .ok_or_else(|| Error::from("启动工作流时必须包含流程编码"))?;
    let business_id = service::blank_to_none(bo.business_id.as_deref())
        .ok_or_else(|| Error::from("启动工作流时必须包含业务ID"))?;
    if !service::definition_exists(&flow_code).await {
        return Err(Error::from(format!(
            "流程【{flow_code}】未发布，请先在流程设计器中发布流程定义"
        )));
    }

    if let Some(instance) = super::instance::find_by_business(&business_id).await? {
        let instance_id = instance.id.parse::<i64>().unwrap_or_default();
        biz::check_start_status(&instance.flow_status)?;
        let tasks = engine
            .list_tasks_by_instance(instance_id)
            .await
            .map_err(flow_err)?;
        let task_id = tasks.first().and_then(|t| t.id);
        return Ok(StartReturnVO {
            process_instance_id: instance.id,
            task_id: task_id.map(|id| id.to_string()),
            business_code: instance.business_id.into(),
        });
    }

    let mut variable = bo.variables.clone();
    // 与 Java 一致：发起人 / 发起人部门 / 业务主键 写入流程变量，供设计器表达式引用
    variable.insert("initiator".to_string(), json!(uid.to_string()));
    if let Some(dept) = service::user_dept(uid).await {
        variable.insert("initiatorDeptId".to_string(), json!(dept));
    }
    variable.insert("businessId".to_string(), json!(business_id.clone()));

    let params = FlowParams::build()
        .flow_code(&flow_code)
        .handler(uid.to_string())
        // 发起即进入审批中：实例状态从「待提交」直接跳到「审批中」，
        // 业务表也才能同步成 waiting（否则第一条待办的状态与业务状态不一致）
        .flow_status(FlowStatus::APPROVAL)
        .variable(variable)
        // 首个人工节点若未配置办理人（示例流程的「申请人」就是空的），
        // 由发起人自己办理，否则任务没有归属人，待办列表里查不到
        .next_handler(vec![uid.to_string()]);

    let instance = engine.start(&business_id, params).await.map_err(flow_err)?;
    let instance_id = instance.id.unwrap_or_default();
    let tasks = engine
        .list_tasks_by_instance(instance_id)
        .await
        .map_err(flow_err)?;
    let task_id = tasks.first().and_then(|t| t.id);

    biz::sync_business_status(&instance).await;

    Ok(StartReturnVO {
        process_instance_id: instance_id.to_string(),
        task_id: task_id.map(|id| id.to_string()),
        business_code: biz::business_code(&instance),
    })
}

// ----------------------------------------------------------------------
// 办理
// ----------------------------------------------------------------------

/// 办理任务（通过 / 退回）
pub async fn complete(uid: i64, bo: &CompleteTaskBo) -> Result<()> {
    let engine = service::engine()?;
    let task_id = bo.task_id.ok_or_else(|| Error::from("任务 id 不能为空"))?;
    let task = engine
        .store()
        .get_task(task_id)
        .await
        .map_err(flow_err)?
        .ok_or_else(|| Error::from("流程任务不存在或已审批"))?;

    let mut params = base_params(uid).await;
    params.skip_type = Some(service::normalize_skip_type(bo.skip_type.as_deref()).to_string());
    if let Some(msg) = service::blank_to_none(bo.message.as_deref()) {
        params.message = Some(msg);
    }
    if let Some(node) = service::blank_to_none(bo.node_code.as_deref()) {
        params.node_code = Some(node);
    }
    if !bo.next_handlers.is_empty() {
        params.next_handler = bo.next_handlers.clone();
        params.next_handler_append = true;
    }
    if !bo.variables.is_empty() {
        params.variable.extend(bo.variables.clone());
    }

    engine.skip(task_id, params).await.map_err(flow_err)?;
    sync_after_action(task.instance_id).await;
    Ok(())
}

/// 终止流程（当前任务直接落到结束节点）
pub async fn termination(uid: i64, bo: &TerminationBo) -> Result<()> {
    let engine = service::engine()?;
    let task_id = bo.task_id.ok_or_else(|| Error::from("任务 id 不能为空"))?;
    let task = engine
        .store()
        .get_task(task_id)
        .await
        .map_err(flow_err)?
        .ok_or_else(|| Error::from("流程任务不存在或已审批"))?;
    let mut params = base_params(uid).await;
    if let Some(msg) = service::blank_to_none(bo.message.as_deref()) {
        params.message = Some(msg);
    }
    let instance_id = task.instance_id;
    engine
        .termination_task(task, params)
        .await
        .map_err(flow_err)?;
    sync_after_action(instance_id).await;
    Ok(())
}

/// 撤销流程（发起人撤回，回到开始节点的下一个节点）
pub async fn revoke(uid: i64, instance_id: i64, message: Option<&str>) -> Result<()> {
    let engine = service::engine()?;
    let mut params = base_params(uid).await;
    if let Some(msg) = service::blank_to_none(message) {
        params.message = Some(msg);
    }
    engine.revoke(instance_id, params).await.map_err(flow_err)?;
    sync_instance(instance_id).await;
    Ok(())
}

/// 转办 / 委派 / 加签 / 减签
pub async fn operate(uid: i64, op: &str, bo: &TaskOperationBo) -> Result<()> {
    let engine = service::engine()?;
    let task_id = bo.task_id.ok_or_else(|| Error::from("任务 id 不能为空"))?;
    let mut params = base_params(uid).await;
    if let Some(msg) = service::blank_to_none(bo.message.as_deref()) {
        params.message = Some(msg);
    }
    match op {
        "transfer" => {
            params.add_handlers = bo.handler_ids.clone();
            engine.transfer(task_id, params).await.map_err(flow_err)?;
        }
        "depute" => {
            params.add_handlers = bo.handler_ids.clone();
            engine.depute(task_id, params).await.map_err(flow_err)?;
        }
        "addSignature" => {
            params.add_handlers = bo.handler_ids.clone();
            engine.add_signature(task_id, params).await.map_err(flow_err)?;
        }
        "reductionSignature" => {
            params.reduction_handlers = bo.handler_ids.clone();
            engine
                .reduction_signature(task_id, params)
                .await
                .map_err(flow_err)?;
        }
        other => return Err(Error::from(format!("不支持的任务操作: {other}"))),
    }
    Ok(())
}

/// 取回（把已办任务拉回自己手上）
pub async fn task_back(uid: i64, task_id: i64) -> Result<()> {
    let engine = service::engine()?;
    let task = engine
        .store()
        .get_task(task_id)
        .await
        .map_err(flow_err)?
        .ok_or_else(|| Error::from("流程任务不存在或已审批"))?;
    let params = base_params(uid).await;
    engine.task_back(task_id, params).await.map_err(flow_err)?;
    sync_after_action(task.instance_id).await;
    Ok(())
}

// ----------------------------------------------------------------------
// 内部辅助
// ----------------------------------------------------------------------

/// 构造带办理人权限的公共参数
async fn base_params(uid: i64) -> FlowParams {
    let admin = service::is_admin(uid).await;
    FlowParams::build()
        .handler(uid.to_string())
        .permission_flag(service::permission_flags(uid).await)
        // 超管放行：既不是办理人也不在节点角色里时仍可操作
        .ignore(admin)
}

/// 审核动作后：重新读实例并回写业务状态
async fn sync_after_action(instance_id: i64) {
    sync_instance(instance_id).await;
}

async fn sync_instance(instance_id: i64) {
    let Some(engine) = crate::modules::workflow::state::engine() else {
        return;
    };
    match engine.store().get_instance(instance_id).await {
        // 回写失败不回滚流程：流程已经落库，业务状态最多滞后一次同步
        Ok(Some(instance)) => biz::sync_business_status(&instance).await,
        Ok(None) => log::warn!("[workflow] 实例 {instance_id} 不存在，跳过业务状态回写"),
        Err(e) => log::warn!("[workflow] 回写业务状态失败(实例 {instance_id}): {e}"),
    }
}

/// 列表展示用的实例上下文（业务主键 / 流程名 / 流程编码）
#[derive(Default, Clone)]
struct InstanceCtx {
    business_id: String,
    flow_name: Option<String>,
    flow_code: Option<String>,
}

async fn instance_context(
    items: impl IntoIterator<Item = (i64, i64)>,
) -> HashMap<i64, InstanceCtx> {
    let mut out: HashMap<i64, InstanceCtx> = HashMap::new();
    let Some(engine) = crate::modules::workflow::state::engine() else {
        return out;
    };
    let mut defs: HashMap<i64, (String, String)> = HashMap::new();
    for (instance_id, definition_id) in items {
        if out.contains_key(&instance_id) {
            continue;
        }
        let ok = match engine.store().get_instance(instance_id).await {
            Ok(Some(ins)) => {
                let def = if let Some(cached) = defs.get(&definition_id) {
                    Some(cached.clone())
                } else {
                    match engine.store().get_definition(definition_id).await {
                        Ok(Some(d)) => {
                            let pair = (d.flow_name.clone(), d.flow_code.clone());
                            defs.insert(definition_id, pair.clone());
                            Some(pair)
                        }
                        _ => None,
                    }
                };
                out.insert(
                    instance_id,
                    InstanceCtx {
                        business_id: ins.business_id,
                        flow_name: def.as_ref().map(|d| d.0.clone()),
                        flow_code: def.map(|d| d.1),
                    },
                );
                true
            }
            _ => false,
        };
        if !ok {
            out.insert(instance_id, InstanceCtx::default());
        }
    }
    out
}

fn fill_task(mut vo: TaskVO, ctx: &HashMap<i64, InstanceCtx>) -> TaskVO {
    let key = vo.instance_id.parse::<i64>().unwrap_or_default();
    if let Some(c) = ctx.get(&key) {
        vo.business_id = Some(c.business_id.clone()).filter(|s| !s.is_empty());
        vo.flow_name = c.flow_name.clone();
        vo.flow_code = c.flow_code.clone();
    }
    vo
}

fn fill_his(mut vo: HisTaskVO, ctx: &HashMap<i64, InstanceCtx>) -> HisTaskVO {
    let key = vo.instance_id.parse::<i64>().unwrap_or_default();
    if let Some(c) = ctx.get(&key) {
        vo.business_id = Some(c.business_id.clone()).filter(|s| !s.is_empty());
        vo.flow_name = c.flow_name.clone();
    }
    vo
}

/// 给已办记录补办理人昵称（一次 IN 查询，approver 原始 id 保留不变）
async fn fill_approvers(mut rows: Vec<HisTaskVO>) -> Vec<HisTaskVO> {
    let names = service::user_names(rows.iter().filter_map(|r| r.approver.as_deref())).await;
    if names.is_empty() {
        return rows;
    }
    for r in rows.iter_mut() {
        if let Some(name) = r.approver.as_deref().and_then(|id| names.get(id)) {
            r.approver_name = Some(name.clone());
        }
    }
    rows
}

/// 把 flowCode 翻译成过滤条件：单版本下推到 SQL，多版本返回 id 集合
async fn flow_code_filter(flow_code: Option<&str>) -> (Option<i64>, Vec<i64>) {
    let Some(code) = service::blank_to_none(flow_code) else {
        return (None, Vec::new());
    };
    let ids = service::definition_ids_by_code(&code).await;
    match ids.len() {
        // 编码查无定义时必须让结果为空，不能退化成「不过滤」把整表返回
        0 => (Some(-1), Vec::new()),
        1 => (ids.first().copied(), Vec::new()),
        _ => (None, ids),
    }
}
