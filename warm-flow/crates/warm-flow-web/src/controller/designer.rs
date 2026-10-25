//! 设计器接口，对应 Java `WarmFlowController`（`/warm-flow/**`）与 `WarmFlowUiController`。

use std::collections::HashMap;

use actix_web::{web, HttpRequest};
use serde_json::{Map, Value};

use warm_flow_core::enums::{FormCustomEnum, ModelEnum};
use warm_flow_core::dto::{DefJson, FlowParams};
use warm_flow_core::entity::Definition;
use warm_flow_core::error::{FlowError, Result};
use warm_flow_core::store::PageData;

use crate::result::{WebDone, WebResult};
use crate::vo::*;
use crate::FlowWebState;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/save-json", web::post().to(save_json))
        .route("/query-def", web::get().to(query_def))
        .route("/query-def/{id}", web::get().to(query_def_by_id))
        .route("/query-flow-chart/{id}", web::get().to(query_flow_chart))
        .route("/handler-type", web::get().to(handler_type))
        .route("/handler-result", web::get().to(handler_result))
        .route("/handler-feedback", web::get().to(handler_feedback))
        .route("/handler-dict", web::get().to(handler_dict))
        .route("/published-form", web::get().to(published_form))
        .route("/form-content/{id}", web::get().to(get_form_content))
        .route("/form-content", web::post().to(save_form_content))
        .route("/execute/load/{task_id}", web::get().to(load))
        .route("/execute/hisLoad/{task_id}", web::get().to(his_load))
        .route("/execute/handle", web::post().to(handle))
        .route("/node-ext", web::get().to(node_ext))
        .route("/listener-list", web::get().to(listener_list));
}

/// 保存设计器 JSON。`onlyNodeSkip = true` 时只覆盖节点与连线，保留定义本身的元信息。
async fn save_json(
    state: web::Data<FlowWebState>,
    req: HttpRequest,
    body: web::Json<DefJson>,
) -> WebResult<Definition> {
    let only_node_skip = req
        .headers()
        .get("onlyNodeSkip")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.trim().eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    WebResult(state.engine().save_def(&body.into_inner(), only_node_skip).await)
}

async fn query_def(state: web::Data<FlowWebState>) -> WebResult<DefJson> {
    WebResult(design_shell(&state, None).await)
}

async fn query_def_by_id(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<DefJson> {
    WebResult(design_shell(&state, Some(id.into_inner())).await)
}

/// 新建定义时返回默认壳子（经典模式 + 无表单），并带上分类树 / 表单路径树
async fn design_shell(state: &FlowWebState, id: Option<i64>) -> Result<DefJson> {
    let mut def_json = match id {
        Some(id) => state.engine().query_design(id).await?,
        None => DefJson::default(),
    };
    if id.is_none() {
        def_json.model_value = Some(ModelEnum::CLASSICS.to_string());
        def_json.form_custom = Some(FormCustomEnum::N.to_string());
    }

    let categories = state.ui().categories();
    if !categories.is_empty() {
        def_json.category_list = Some(warm_flow_core::dto::Tree::build_tree(categories));
    }
    let form_paths = state.ui().form_paths();
    if !form_paths.is_empty() {
        def_json.form_path_list = Some(warm_flow_core::dto::Tree::build_tree(form_paths));
        if id.is_none() {
            // 配了表单路径，说明走外挂表单
            def_json.form_custom = Some(FormCustomEnum::Y.to_string());
        }
    }
    Ok(def_json)
}

/// 流程图（入参是实例 id）
async fn query_flow_chart(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<DefJson> {
    let engine = state.engine();
    let result = async {
        let mut def_json = engine.query_flow_chart(id.into_inner()).await?;
        def_json.chart_status_color = Some(engine.chart_rgb(&def_json.model()));
        def_json.top_text_show = Some(engine.config().top_text_show);
        state.ui().chart_ext(&mut def_json);
        Ok(def_json)
    }
    .await;
    WebResult(result)
}

async fn handler_type(state: web::Data<FlowWebState>) -> WebResult<Vec<String>> {
    WebResult::ok(state.ui().handler_types())
}

async fn handler_result(
    state: web::Data<FlowWebState>,
    query: web::Query<HandlerQuery>,
) -> WebResult<HandlerSelectVo> {
    WebResult::ok(state.ui().handler_select(&query.into_inner()).await)
}

async fn handler_feedback(
    state: web::Data<FlowWebState>,
    query: web::Query<HashMap<String, String>>,
) -> WebResult<Vec<HandlerFeedBackVo>> {
    // 兼容 storageIds=1,2,3 与 storageIds=1&storageIds=2 两种传法
    let ids: Vec<String> = query
        .get("storageIds")
        .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
        .unwrap_or_default();
    WebResult::ok(state.ui().handler_feedback(&ids).await)
}

async fn handler_dict(state: web::Data<FlowWebState>) -> WebResult<Vec<Dict>> {
    WebResult::ok(state.ui().handler_dict())
}

async fn published_form(state: web::Data<FlowWebState>) -> WebResult<Vec<warm_flow_core::entity::Form>> {
    WebResult(state.engine().store().list_forms(Some(1)).await)
}

async fn get_form_content(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<String> {
    let result = async {
        let form = state
            .engine()
            .store()
            .get_form(id.into_inner())
            .await?
            .ok_or_else(|| FlowError::not_found("表单不存在"))?;
        Ok(form.form_content.unwrap_or_default())
    }
    .await;
    WebResult(result)
}

async fn save_form_content(
    state: web::Data<FlowWebState>,
    body: web::Json<FormContentDto>,
) -> WebResult<()> {
    let body = body.into_inner();
    let result = async {
        let id = body.id.ok_or_else(|| FlowError::biz("表单id不能为空"))?;
        let mut form = state
            .engine()
            .store()
            .get_form(id)
            .await?
            .ok_or_else(|| FlowError::not_found("表单不存在"))?;
        form.form_content = body.form_content;
        state.engine().store().update_form(&form).await
    }
    .await;
    WebResult(result)
}

async fn load(state: web::Data<FlowWebState>, task_id: web::Path<i64>) -> WebResult<warm_flow_core::dto::FlowDto> {
    WebResult(state.engine().load(task_id.into_inner(), &FlowParams::default()).await)
}

async fn his_load(
    state: web::Data<FlowWebState>,
    task_id: web::Path<i64>,
) -> WebResult<warm_flow_core::dto::FlowDto> {
    WebResult(state.engine().his_load(task_id.into_inner(), &FlowParams::default()).await)
}

/// 通用表单审批：表单数据整体写入流程变量 `formData`
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct HandleQuery {
    task_id: i64,
    skip_type: String,
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    node_code: Option<String>,
}

async fn handle(
    state: web::Data<FlowWebState>,
    query: web::Query<HandleQuery>,
    body: web::Json<Map<String, Value>>,
) -> WebResult<warm_flow_core::entity::Instance> {
    let q = query.into_inner();
    let mut params = FlowParams::build().skip_type(q.skip_type);
    if let Some(message) = q.message {
        params = params.message(message);
    }
    if let Some(node_code) = q.node_code.filter(|c| !c.is_empty()) {
        params = params.node_code(node_code);
    }
    params = params.form_data(body.into_inner());
    WebResult(state.engine().skip(q.task_id, params).await)
}

async fn node_ext(state: web::Data<FlowWebState>) -> WebResult<Vec<NodeExt>> {
    WebResult::ok(state.ui().node_ext())
}

async fn listener_list(state: web::Data<FlowWebState>) -> WebResult<Vec<ListenerVo>> {
    WebResult::ok(state.ui().listener_list())
}

/// `/warm-flow-ui/config`：设计器启动时读取 token 头名称与框架标识
pub async fn config(state: web::Data<FlowWebState>) -> WebResult<WarmFlowVo> {
    let engine = state.engine();
    let token_names: Vec<String> = engine
        .config()
        .token_name
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if token_names.is_empty() {
        return WebResult(Err(FlowError::biz("未配置tokenName")));
    }
    WebResult::ok(WarmFlowVo { token_name_list: token_names, framework: state.ui().framework() })
}

/// 供其它 controller 复用的分页包装
#[allow(dead_code)]
pub(crate) fn page_vo<T>(page: PageData<T>) -> PageVo<T> {
    PageVo::from(page)
}

/// 统一处理「无返回数据」的接口
#[allow(dead_code)]
pub(crate) fn done() -> WebDone {
    WebDone
}
