use std::future::{ready, Ready};
use std::rc::Rc;

use actix_http::body::BoxBody;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
    HttpMessage,
};
use futures_util::future::LocalBoxFuture;

use crate::casbin;
use crate::config::global_variables::ADMIN_NAME;
use crate::modules::system::domain::vo::RespVO;
use crate::token_auth::{checked_token, is_white_list_api, get_token};
use crate::web_data::CurrentUser;

/// 是否 warm-flow 设计器接口（见 `http_server.rs` 里的 `warm_flow_web::configure`）
fn is_designer_api(path: &str) -> bool {
    path.contains("/warm-flow")
}

pub struct Auth;

impl<S: 'static> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    #[inline]
    fn poll_ready(
        &self,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(Into::into)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let token = get_token(req.request());
        let path = req.path().to_string();
        let method = req.method().as_str().to_string();
        Box::pin(async move {
            if !is_white_list_api(&path) {
                // 1) token 校验（无效/过期 -> 401，强制前端退出登录）
                let data = match checked_token(&token).await {
                    Ok(d) => d,
                    Err(e) => {
                        // 内部错误只落日志，对外返回泛化提示，避免泄露实现细节
                        log::warn!("[ruoyi_rust] token check failed: {}", e);
                        let resp: RespVO<String> = RespVO {
                            code: 401,
                            msg: Some("登录状态已失效，请重新登录".to_string()),
                            data: None,
                        };
                        return Ok(req.into_response(resp.resp_json()));
                    }
                };

                // 2) 权限判定：管理员直接放行；其余必须通过 casbin。
                // fail-closed：casbin 未就绪 / 执行异常一律拒绝，避免鉴权失效被绕过。
                let allowed = if data.user_name == ADMIN_NAME {
                    true
                } else if is_designer_api(&path) {
                    // 设计器接口（/warm-flow/**、/warm-flow-ui/**）由 warm-flow 内置提供，
                    // 没有对应的 sys_menu 权限标识；按「已登录即可用」放行，
                    // 菜单可见性仍由「工作流」菜单控制。
                    true
                } else if !casbin::enforcer_ready() {
                    log::warn!(
                        "[ruoyi_rust] casbin not ready, deny {} for {}",
                        path,
                        data.user_name
                    );
                    false
                } else {
                    match casbin::enforce(&data.user_name, &path, &method).await {
                        Ok(true) => true,
                        Ok(false) => false,
                        Err(e) => {
                            log::warn!(
                                "[ruoyi_rust] casbin enforce error (fail-closed deny): {}",
                                e
                            );
                            false
                        }
                    }
                };

                if !allowed {
                    let resp: RespVO<String> = RespVO {
                        code: 0,
                        msg: Some(format!("无权限访问:{}", path)),
                        data: None,
                    };
                    return Ok(req.into_response(resp.resp_json()));
                }

                //刷新过期时间，并把当前用户写入请求作用域（替代原先的 thread_local 全局态）
                req.extensions_mut().insert(CurrentUser {
                    id: data.id,
                    name: data.user_name.clone(),
                });
                // 复用：中间件已从 Redis 取到完整 UserCache，注入请求作用域，
                // 处理器 get_user_cache 直接命中，省去再次 JWT 校验 + Redis GET。
                req.extensions_mut().insert(data);
            }
            let res = svc.call(req).await?;
            Ok(res)
        })
    }
}
