//! 流程定义维护接口。Java 版这部分由业务项目自行封装，这里按 `DefService` 的能力给出通用实现。

use actix_web::{web, HttpRequest};

use warm_flow_core::dto::{DefJson, FlowCombine};
use warm_flow_core::entity::{Definition, Node};
use warm_flow_core::error::FlowError;

use crate::query::DefinitionPageQuery;
use crate::result::WebResult;
use crate::vo::PageVo;
use crate::FlowWebState;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/list", web::get().to(list))
        .route("/{id}", web::get().to(get))
        .route("/data/{id}", web::get().to(get_with_nodes))
        .route("/design/{id}", web::get().to(query_design))
        .route("/export/{id}", web::get().to(export))
        .route("/save", web::post().to(save))
        .route("/import-json", web::post().to(import_json))
        .route("/import", web::post().to(import_def))
        .route("/publish/{id}", web::post().to(publish))
        .route("/un-publish/{id}", web::post().to(un_publish))
        .route("/copy/{id}", web::post().to(copy))
        .route("/active/{id}", web::put().to(active))
        .route("/un-active/{id}", web::put().to(un_active))
        .route("/{id}", web::delete().to(remove))
        .route("/new-version/{id}", web::get().to(new_version))
        .route("/published/{flow_code}", web::get().to(published_by_code))
        .route("/check/{id}", web::get().to(check))
        .route("/structure", web::post().to(structure));
}

async fn list(
    state: web::Data<FlowWebState>,
    query: web::Query<DefinitionPageQuery>,
) -> WebResult<PageVo<Definition>> {
    let q = query.into_inner();
    let condition: warm_flow_core::store::DefinitionQuery = (&q).into();
    let result = state.engine().list_definitions(&condition).await.map(PageVo::from);
    WebResult(result)
}

async fn get(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<Definition> {
    let result = async {
        state
            .engine()
            .store()
            .get_definition(id.into_inner())
            .await?
            .ok_or_else(|| FlowError::not_found("流程定义不存在"))
    }
    .await;
    WebResult(result)
}

/// 定义 + 全部节点（含连线），对应 Java `getAllDataDefinition`
async fn get_with_nodes(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<Definition> {
    WebResult(state.engine().get_all_data_definition(id.into_inner()).await)
}

async fn query_design(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<DefJson> {
    WebResult(state.engine().query_design(id.into_inner()).await)
}

/// 导出设计器 JSON 文本（`data` 为 JSON 字符串，与 Java `exportJson` 一致）
async fn export(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<String> {
    WebResult(state.engine().export_json(id.into_inner()).await)
}

async fn save(
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

/// 导入 JSON 文本（设计器导出的字符串）
async fn import_json(state: web::Data<FlowWebState>, body: web::Json<serde_json::Value>) -> WebResult<Definition> {
    let text = match body.into_inner() {
        serde_json::Value::String(s) => s,
        other => other.to_string(),
    };
    WebResult(state.engine().import_json(&text).await)
}

async fn import_def(state: web::Data<FlowWebState>, body: web::Json<DefJson>) -> WebResult<Definition> {
    WebResult(state.engine().import_def(&body.into_inner()).await)
}

async fn publish(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<()> {
    WebResult(state.engine().publish(id.into_inner()).await)
}

async fn un_publish(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<()> {
    WebResult(state.engine().un_publish(id.into_inner()).await)
}

async fn copy(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<Definition> {
    WebResult(state.engine().copy_def(id.into_inner()).await)
}

async fn active(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<()> {
    WebResult(state.engine().active_def(id.into_inner()).await)
}

async fn un_active(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<()> {
    WebResult(state.engine().un_active_def(id.into_inner()).await)
}

async fn remove(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<()> {
    WebResult(state.engine().remove_def(&[id.into_inner()]).await)
}

/// 计算下一个可用版本号
async fn new_version(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<String> {
    let result = async {
        let definition = state
            .engine()
            .store()
            .get_definition(id.into_inner())
            .await?
            .ok_or_else(|| FlowError::not_found("流程定义不存在"))?;
        state.engine().get_new_version(&definition).await
    }
    .await;
    WebResult(result)
}

/// 按流程编码取最新已发布版本
async fn published_by_code(state: web::Data<FlowWebState>, flow_code: web::Path<String>) -> WebResult<Definition> {
    WebResult(state.engine().get_publish_by_flow_code(&flow_code.into_inner()).await)
}

/// 校验开始 / 结束节点是否齐备
async fn check(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<Vec<Node>> {
    let result = state
        .engine()
        .check_flow_complete(id.into_inner())
        .await
        .map(|(start, end)| vec![start, end]);
    WebResult(result)
}

/// 只做结构化 + 合法性校验，不落库
async fn structure(state: web::Data<FlowWebState>, body: web::Json<DefJson>) -> WebResult<FlowCombine> {
    let engine = state.engine();
    let result = async {
        let mut combine = engine.structure_flow(&body.into_inner())?;
        engine.check_flow_legal(&mut combine)?;
        Ok(combine)
    }
    .await;
    WebResult(result)
}
