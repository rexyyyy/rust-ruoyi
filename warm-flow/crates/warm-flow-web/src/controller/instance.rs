//! 流程实例接口：开启、查询、激活 / 挂起、删除、变量维护。

use actix_web::web;
use serde_json::{Map, Value};

use warm_flow_core::dto::{DefJson, FlowParams};
use warm_flow_core::entity::{HisTask, Instance, Task};
use warm_flow_core::error::FlowError;

use crate::query::{HisTaskPageQuery, InstancePageQuery, TaskPageQuery};
use crate::result::WebResult;
use crate::vo::{PageVo, StartDto};
use crate::FlowWebState;

pub fn routes(cfg: &mut web::ServiceConfig) {
    // 单段路径的字面量路由必须排在 `/{id}` 之前，避免被动态段吞掉
    cfg.route("/start", web::post().to(start))
        .route("/list", web::get().to(list))
        .route("/task-list", web::get().to(task_page))
        .route("/his-task-list", web::get().to(his_task_page))
        .route("/{id}", web::get().to(get))
        .route("/chart/{id}", web::get().to(flow_chart))
        .route("/tasks/{id}", web::get().to(tasks))
        .route("/his-tasks/{id}", web::get().to(his_tasks))
        .route("/variable/merge/{id}", web::post().to(merge_variable))
        .route("/variable/remove/{id}", web::post().to(remove_variables))
        .route("/active/{id}", web::put().to(active))
        .route("/un-active/{id}", web::put().to(un_active))
        .route("/{id}", web::delete().to(remove));
}

/// 开启流程。`flowCode` 放在 body 或 query 均可，query 优先。
async fn start(
    state: web::Data<FlowWebState>,
    query: web::Query<std::collections::HashMap<String, String>>,
    body: web::Json<StartDto>,
) -> WebResult<Instance> {
    let dto = body.into_inner();
    let mut params = dto.flow_params.unwrap_or_default();
    if params.variable.is_empty() {
        params.variable = dto.variable;
    }
    if let Some(code) = query.get("flowCode").filter(|c| !c.is_empty()) {
        params.flow_code = Some(code.clone());
    }
    WebResult(state.engine().start(&dto.business_id, params).await)
}

async fn list(
    state: web::Data<FlowWebState>,
    query: web::Query<InstancePageQuery>,
) -> WebResult<PageVo<Instance>> {
    let q = query.into_inner();
    let condition: warm_flow_core::store::InstanceQuery = (&q).into();
    WebResult(state.engine().list_instances(&condition).await.map(PageVo::from))
}

async fn get(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<Instance> {
    let result = async {
        state
            .engine()
            .store()
            .get_instance(id.into_inner())
            .await?
            .ok_or_else(|| FlowError::not_found("流程实例不存在"))
    }
    .await;
    WebResult(result)
}

async fn flow_chart(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<DefJson> {
    WebResult(state.engine().query_flow_chart(id.into_inner()).await)
}

async fn tasks(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<Vec<Task>> {
    WebResult(state.engine().list_tasks_by_instance(id.into_inner()).await)
}

async fn his_tasks(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<Vec<HisTask>> {
    WebResult(state.engine().list_his_tasks_by_instance(id.into_inner()).await)
}

async fn task_page(
    state: web::Data<FlowWebState>,
    query: web::Query<TaskPageQuery>,
) -> WebResult<PageVo<Task>> {
    let q = query.into_inner();
    let condition: warm_flow_core::store::TaskQuery = (&q).into();
    WebResult(state.engine().list_tasks(&condition).await.map(PageVo::from))
}

async fn his_task_page(
    state: web::Data<FlowWebState>,
    query: web::Query<HisTaskPageQuery>,
) -> WebResult<PageVo<HisTask>> {
    let q = query.into_inner();
    let condition: warm_flow_core::store::HisTaskQuery = (&q).into();
    WebResult(state.engine().list_his_tasks(&condition).await.map(PageVo::from))
}

/// 合并流程变量（已存在的 key 覆盖）
async fn merge_variable(
    state: web::Data<FlowWebState>,
    id: web::Path<i64>,
    body: web::Json<Map<String, Value>>,
) -> WebResult<Instance> {
    WebResult(state.engine().merge_instance_variable(id.into_inner(), &body.into_inner()).await)
}

/// 删除流程变量
async fn remove_variables(
    state: web::Data<FlowWebState>,
    id: web::Path<i64>,
    body: web::Json<Vec<String>>,
) -> WebResult<()> {
    WebResult(state.engine().remove_variables(id.into_inner(), &body.into_inner()).await)
}

async fn active(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<()> {
    WebResult(state.engine().active_instance(id.into_inner()).await)
}

async fn un_active(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<()> {
    WebResult(state.engine().un_active_instance(id.into_inner()).await)
}

async fn remove(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<()> {
    WebResult(state.engine().remove_instance(&[id.into_inner()]).await)
}

/// 便于业务侧直接构造流程参数的辅助函数
#[allow(dead_code)]
pub(crate) fn params_with_flow_code(flow_code: impl Into<String>, handler: impl Into<String>) -> FlowParams {
    FlowParams::build().flow_code(flow_code).handler(handler)
}
