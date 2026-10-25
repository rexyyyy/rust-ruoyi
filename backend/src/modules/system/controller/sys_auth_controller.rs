use crate::modules::system::domain::vo::{JWTToken, RespJson, RespVO};
use crate::modules::system::service::CONTEXT;
use actix_web::{HttpRequest, Responder, web};
use crate::config::cache_variables::LOGIN_TOKEN_KEY;
use crate::config::global_variables::LOGIN_SUC;
use crate::modules::system::domain::bo::sign_in::SignInDTO;
use crate::token_auth::{get_token, checked_token};
use crate::modules::system::domain::bo::sign_in::{SignUpDTO, SocialCallbackDTO};
use crate::modules::system::domain::SysUser;
use crate::config::global_variables::{DEL_FLAG_NORMAL, STATUS_NORMAL};

// ///Check whether the token_auth and path are valid and accessible
// pub async fn check(arg: web::Json<SysAuthDTO>) -> impl Responder {
//     let r = CONTEXT.sys_auth_service.check_auth(arg.0).await;
//     RespVO::from_result(&r).resp_json()
// }

pub async fn login(req: HttpRequest, arg: web::Json<SignInDTO>) -> impl Responder {
    let user_name = arg.0.username.clone();
    let token = CONTEXT.sys_user_service.login(&arg.0, &req).await;
    if token.is_err() {
        CONTEXT.sys_operlog_service.record("登录", 5, "POST /auth/login", &user_name, "/auth/login", 1, "登录失败").await;
        return RespVO::from_error_result(500, &token).resp_json();
    }
    let token = token.unwrap();
    CONTEXT.sys_operlog_service.record("登录", 5, "POST /auth/login", &user_name, "/auth/login", 0, "").await;
    // plus-ui 5.x user store 消费 res.data.access_token（RuoYi-Vue-Plus R<LoginVo> 包 data 层）
    let mut res = RespJson::success();
    res.insert("msg".to_string(), "操作成功".into());
    let mut data = serde_json::Map::new();
    data.insert("access_token".to_string(), token.into());
    res.insert("data".to_string(), serde_json::Value::Object(data));
    return res.resp_json();
}

pub async fn logout(req: HttpRequest) -> impl Responder {
    // 注意：/auth/logout 在鉴权白名单内，Auth 中间件不会注入 CurrentUser，
    // 因此这里不能依赖 CurrentUser 提取器，用户名从登录缓存（Redis）中取。
    let token = get_token(&req);
    if !token.is_empty() {
        let claims = JWTToken::verify(&CONTEXT.config.jwt_secret, &token);
        if claims.is_ok() {
            let login_user_key = claims.unwrap().login_user_key;
            let user_name = match checked_token(&token).await {
                Ok(d) => d.user_name,
                Err(_) => String::new(),
            };
            CONTEXT.sys_logininfor_service.add_async(&crate::util::web_utils::build_logininfor(&req, user_name.clone(), LOGIN_SUC, "退出成功".to_string())).await;
            CONTEXT.sys_operlog_service.record("退出登录", 6, "POST /auth/logout", &user_name, "/auth/logout", 0, "").await;
            CONTEXT.cache_service.del(&format!("{}{}", LOGIN_TOKEN_KEY, login_user_key)).await;
        }
    }
    RespVO::<String>::from_success_info("退出成功!").resp_json()
}

pub async fn info(req: HttpRequest) -> impl Responder {
    // match crate::token_auth::check_permit(req, "").await {
    //     None => {}
    //     Some(res) => { return res.resp_json(); }
    // }

    let user_cache = CONTEXT.sys_user_service.get_user_cache(&req).await;
    // let user_data = CONTEXT
    //     .sys_user_service
    //     .get_user_info_by_token(&user_cache)
    //     .await;
    if user_cache.is_err() {
        return RespVO::<u64>::from_error_info(401, "请重新！").resp_json();
    }
    let user_cache = user_cache.unwrap();
    let mut res = RespJson::success();
    res.insert("permissions".to_string(), serde_json::json!(&user_cache.permissions));
    res.insert("user".to_string(), serde_json::json!(&user_cache.user));
    
    res.insert("roles".to_string(), serde_json::json!(&user_cache.roles.iter().map(|r| r.role_key.clone()).collect::<Vec<_>>()));
    res.resp_json()
}

// 用户注册（对应Spring POST /auth/register）
pub async fn register(arg: web::Json<SignUpDTO>) -> impl Responder {
    let username = arg.0.username.clone().unwrap_or_default();
    let password = arg.0.password.clone().unwrap_or_default();
    if username.is_empty() || password.is_empty() {
        return RespVO::<u64>::from_error_info(500, "用户名和密码不能为空").resp_json();
    }
    let exists = CONTEXT.sys_user_service.find_by_account(&username).await.unwrap_or(None);
    if exists.is_some() {
        return RespVO::<u64>::from_error_info(500, "注册用户失败，账号已存在").resp_json();
    }
    let user = SysUser {
        user_id: crate::util::idseq::next_id() as i64,
        dept_id: Some(103),
        user_name: Some(username.clone()),
        nick_name: Some(username.clone()),
        email: None,
        phonenumber: None,
        sex: Some("0".to_string()),
        avatar: None,
        // bcrypt 哈希下沉到阻塞线程池（默认 cost 下单次 200ms+ CPU）
        password: Some(crate::util::password_encoder::PasswordEncoder::encode_async(&password).await),
        status: Some(STATUS_NORMAL.to_string()),
        del_flag: Some(DEL_FLAG_NORMAL.to_string()),
        login_ip: None,
        login_date: None,
        create_by: None,
        create_time: Some(chrono::Local::now().naive_local()),
        update_by: None,
        update_time: None,
        remark: None,
    };
    // 新增用户 + 分配默认角色（普通角色 role_id=2）在同一事务内完成；
    // 旧写法 `add_user_roles(...)` 漏了 `.await`，注册用户实际上没有任何角色。
    let r = CONTEXT
        .sys_user_service
        .add_with_roles(user, &["2".to_string()])
        .await;
    if r.is_err() {
        return RespVO::from_error_result(500, &r).resp_json();
    }
    RespVO::<String>::from_success_info("注册成功").resp_json()
}

// 租户列表（对应Spring GET /auth/tenant/list，单租户体系返回默认租户000000，结构对齐Java版 data.tenantEnabled + data.voList）
pub async fn tenant_list() -> impl Responder {
    let mut json = RespJson::success_info("操作成功");
    json.insert(
        "data".to_string(),
        serde_json::json!({
            "tenantEnabled": true,
            "voList": [{
                "tenantId": "000000",
                "contactUserName": "admin",
                "contactPhone": "",
                "companyName": "默认租户",
                "licenseNumber": "",
                "domain": "",
                "expireTime": null,
                "accountCount": -1,
                "status": "0",
                "delFlag": "0",
                "createTime": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
            }]
        }),
    );
    json.resp_json()
}

// 社交登录回调（对应Spring POST /auth/social/callback）
pub async fn social_callback(arg: web::Json<SocialCallbackDTO>) -> impl Responder {
    let _ = arg;
    RespVO::<String>::from_error_info(500, "未配置社交登录").resp_json()
}