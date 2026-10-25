//! 待办任务接口：审批、退回、撤销、终止、转办、委派、加签 / 减签、表单加载。
//!
//! 所有动作都接收 `FlowParams`（camelCase），与 Java `FlowParams` 字段一一对应。

use actix_web::web;

use warm_flow_core::dto::{FlowDto, FlowParams};
use warm_flow_core::entity::{HisTask, Instance, Task};

use crate::query::{AtWillQuery, HisTaskPageQuery, TaskPageQuery};
use crate::result::WebResult;
use crate::vo::PageVo;
use crate::FlowWebState;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/list", web::get().to(list))
        .route("/his-list", web::get().to(his_list))
        .route("/instance/{instance_id}", web::get().to(list_by_instance))
        .route("/his-instance/{instance_id}", web::get().to(his_list_by_instance))
        .route("/load/{task_id}", web::get().to(load))
        .route("/his-load/{his_task_id}", web::get().to(his_load))
        // ---- 审批 ----
        .route("/pass/{task_id}", web::post().to(pass))
        .route("/reject/{task_id}", web::post().to(reject))
        .route("/skip/{task_id}", web::post().to(skip))
        .route("/pending/{task_id}", web::post().to(pending))
        .route("/pass-at-will/{task_id}", web::post().to(pass_at_will))
        .route("/reject-at-will/{task_id}", web::post().to(reject_at_will))
        .route("/skip-by-ins/{instance_id}", web::post().to(skip_by_ins))
        // ---- 实例级动作 ----
        .route("/revoke/{instance_id}", web::post().to(revoke))
        .route("/terminate-by-ins/{instance_id}", web::post().to(terminate_by_ins))
        .route("/back-by-ins/{instance_id}", web::post().to(back_by_ins))
        // ---- 任务级动作 ----
        .route("/terminate/{task_id}", web::post().to(terminate))
        .route("/reject-last/{task_id}", web::post().to(reject_last))
        .route("/back/{task_id}", web::post().to(back))
        .route("/transfer/{task_id}", web::post().to(transfer))
        .route("/depute/{task_id}", web::post().to(depute))
        .route("/add-signature/{task_id}", web::post().to(add_signature))
        .route("/reduction-signature/{task_id}", web::post().to(reduction_signature))
        .route("/update-handler/{task_id}", web::post().to(update_handler));
}

async fn list(state: web::Data<FlowWebState>, query: web::Query<TaskPageQuery>) -> WebResult<PageVo<Task>> {
    let q = query.into_inner();
    let condition: warm_flow_core::store::TaskQuery = (&q).into();
    WebResult(state.engine().list_tasks(&condition).await.map(PageVo::from))
}

async fn his_list(
    state: web::Data<FlowWebState>,
    query: web::Query<HisTaskPageQuery>,
) -> WebResult<PageVo<HisTask>> {
    let q = query.into_inner();
    let condition: warm_flow_core::store::HisTaskQuery = (&q).into();
    WebResult(state.engine().list_his_tasks(&condition).await.map(PageVo::from))
}

async fn list_by_instance(state: web::Data<FlowWebState>, instance_id: web::Path<i64>) -> WebResult<Vec<Task>> {
    WebResult(state.engine().list_tasks_by_instance(instance_id.into_inner()).await)
}

async fn his_list_by_instance(
    state: web::Data<FlowWebState>,
    instance_id: web::Path<i64>,
) -> WebResult<Vec<HisTask>> {
    WebResult(state.engine().list_his_tasks_by_instance(instance_id.into_inner()).await)
}

async fn load(state: web::Data<FlowWebState>, task_id: web::Path<i64>) -> WebResult<FlowDto> {
    WebResult(state.engine().load(task_id.into_inner(), &FlowParams::default()).await)
}

async fn his_load(state: web::Data<FlowWebState>, his_task_id: web::Path<i64>) -> WebResult<FlowDto> {
    WebResult(state.engine().his_load(his_task_id.into_inner(), &FlowParams::default()).await)
}

// ----------------------------------------------------------------------
// 审批
// ----------------------------------------------------------------------

/// 审批通过
async fn pass(
    state: web::Data<FlowWebState>,
    task_id: web::Path<i64>,
    body: web::Json<FlowParams>,
) -> WebResult<Instance> {
    WebResult(state.engine().pass(task_id.into_inner(), body.into_inner()).await)
}

/// 退回（退回到目标节点）
async fn reject(
    state: web::Data<FlowWebState>,
    task_id: web::Path<i64>,
    body: web::Json<FlowParams>,
) -> WebResult<Instance> {
    WebResult(state.engine().reject(task_id.into_inner(), body.into_inner()).await)
}

/// 通用跳转，行为由 `skipType` 决定
async fn skip(
    state: web::Data<FlowWebState>,
    task_id: web::Path<i64>,
    body: web::Json<FlowParams>,
) -> WebResult<Instance> {
    WebResult(state.engine().skip(task_id.into_inner(), body.into_inner()).await)
}

/// 暂存（不流转，仅记录历史）
async fn pending(
    state: web::Data<FlowWebState>,
    task_id: web::Path<i64>,
    body: web::Json<FlowParams>,
) -> WebResult<Instance> {
    WebResult(state.engine().pending(task_id.into_inner(), body.into_inner()).await)
}

/// 任意节点跳转（通过）
async fn pass_at_will(
    state: web::Data<FlowWebState>,
    task_id: web::Path<i64>,
    query: web::Query<AtWillQuery>,
    body: web::Json<FlowParams>,
) -> WebResult<Instance> {
    let q = query.into_inner();
    let params = body.into_inner().ignore(q.ignore);
    WebResult(state.engine().pass_at_will(task_id.into_inner(), &q.node_code, params).await)
}

/// 任意节点跳转（退回）
async fn reject_at_will(
    state: web::Data<FlowWebState>,
    task_id: web::Path<i64>,
    query: web::Query<AtWillQuery>,
    body: web::Json<FlowParams>,
) -> WebResult<Instance> {
    let q = query.into_inner();
    let params = body.into_inner().ignore(q.ignore);
    WebResult(state.engine().reject_at_will(task_id.into_inner(), &q.node_code, params).await)
}

/// 按实例 id 办理当前任务
async fn skip_by_ins(
    state: web::Data<FlowWebState>,
    instance_id: web::Path<i64>,
    body: web::Json<FlowParams>,
) -> WebResult<Instance> {
    WebResult(state.engine().skip_by_ins_id(instance_id.into_inner(), body.into_inner()).await)
}

// ----------------------------------------------------------------------
// 实例级动作
// ----------------------------------------------------------------------

/// 撤销（回到发起人）
async fn revoke(
    state: web::Data<FlowWebState>,
    instance_id: web::Path<i64>,
    body: web::Json<FlowParams>,
) -> WebResult<Instance> {
    WebResult(state.engine().revoke(instance_id.into_inner(), body.into_inner()).await)
}

/// 按实例 id 终止流程
async fn terminate_by_ins(
    state: web::Data<FlowWebState>,
    instance_id: web::Path<i64>,
    body: web::Json<FlowParams>,
) -> WebResult<Instance> {
    WebResult(state.engine().termination_by_ins_id(instance_id.into_inner(), body.into_inner()).await)
}

/// 按实例 id 取回
async fn back_by_ins(
    state: web::Data<FlowWebState>,
    instance_id: web::Path<i64>,
    body: web::Json<FlowParams>,
) -> WebResult<Instance> {
    WebResult(state.engine().task_back_by_ins_id(instance_id.into_inner(), body.into_inner()).await)
}

// ----------------------------------------------------------------------
// 任务级动作
// ----------------------------------------------------------------------

/// 终止流程
async fn terminate(
    state: web::Data<FlowWebState>,
    task_id: web::Path<i64>,
    body: web::Json<FlowParams>,
) -> WebResult<Instance> {
    WebResult(state.engine().termination(task_id.into_inner(), body.into_inner()).await)
}

/// 退回上一个节点
async fn reject_last(
    state: web::Data<FlowWebState>,
    task_id: web::Path<i64>,
    body: web::Json<FlowParams>,
) -> WebResult<Instance> {
    WebResult(state.engine().reject_last(task_id.into_inner(), body.into_inner()).await)
}

/// 取回（发起人把任务拿回来）
async fn back(
    state: web::Data<FlowWebState>,
    task_id: web::Path<i64>,
    body: web::Json<FlowParams>,
) -> WebResult<Instance> {
    WebResult(state.engine().task_back(task_id.into_inner(), body.into_inner()).await)
}

/// 转办
async fn transfer(
    state: web::Data<FlowWebState>,
    task_id: web::Path<i64>,
    body: web::Json<FlowParams>,
) -> WebResult<bool> {
    WebResult(state.engine().transfer(task_id.into_inner(), body.into_inner()).await)
}

/// 委派
async fn depute(
    state: web::Data<FlowWebState>,
    task_id: web::Path<i64>,
    body: web::Json<FlowParams>,
) -> WebResult<bool> {
    WebResult(state.engine().depute(task_id.into_inner(), body.into_inner()).await)
}

/// 加签
async fn add_signature(
    state: web::Data<FlowWebState>,
    task_id: web::Path<i64>,
    body: web::Json<FlowParams>,
) -> WebResult<bool> {
    WebResult(state.engine().add_signature(task_id.into_inner(), body.into_inner()).await)
}

/// 减签
async fn reduction_signature(
    state: web::Data<FlowWebState>,
    task_id: web::Path<i64>,
    body: web::Json<FlowParams>,
) -> WebResult<bool> {
    WebResult(state.engine().reduction_signature(task_id.into_inner(), body.into_inner()).await)
}

/// 更换办理人
async fn update_handler(
    state: web::Data<FlowWebState>,
    task_id: web::Path<i64>,
    body: web::Json<FlowParams>,
) -> WebResult<bool> {
    WebResult(state.engine().update_handler(task_id.into_inner(), body.into_inner()).await)
}
