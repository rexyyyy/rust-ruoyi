//! 流程表单接口（`flow_form`）。

use actix_web::web;

use warm_flow_core::entity::Form;
use warm_flow_core::error::FlowError;

use crate::result::WebResult;
use crate::FlowWebState;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/list", web::get().to(list))
        .route("/{id}", web::get().to(get))
        .route("/save", web::post().to(save));
}

/// 表单列表，`isPublish = 1` 表示只取已发布
async fn list(
    state: web::Data<FlowWebState>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> WebResult<Vec<Form>> {
    let is_publish = query.get("isPublish").and_then(|v| v.parse::<i32>().ok());
    WebResult(state.engine().store().list_forms(is_publish).await)
}

async fn get(state: web::Data<FlowWebState>, id: web::Path<i64>) -> WebResult<Form> {
    let result = async {
        state
            .engine()
            .store()
            .get_form(id.into_inner())
            .await?
            .ok_or_else(|| FlowError::not_found("表单不存在"))
    }
    .await;
    WebResult(result)
}

/// 新增或更新：带 id 走更新，否则插入
async fn save(state: web::Data<FlowWebState>, body: web::Json<Form>) -> WebResult<Form> {
    let store = state.engine().store().clone();
    let mut form = body.into_inner();
    let result = async {
        if form.id.is_some() {
            store.update_form(&form).await?;
        } else {
            store.insert_form(&mut form).await?;
        }
        Ok(form)
    }
    .await;
    WebResult(result)
}
