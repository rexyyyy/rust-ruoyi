
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;
use actix_web::middleware::Compress;
use actix_web_validator::{Error, JsonConfig};
use actix_web_validator::error::flatten_errors;


use crate::modules::system::controller::system::{sys_config_controller, sys_dept_controller, sys_dict_data_controller, sys_dict_type_controller, sys_menu_controller, sys_notice_controller, sys_post_controller, sys_profile_controller, sys_role_controller, sys_user_controller};
use crate::modules::system::controller::{img_controller, sys_auth_controller, system_sse};
use crate::state::AppState;
use crate::modules::system::controller::sys_file_controller; //文件上传下载控制器
use crate::modules::system::controller::system_websocket;    //websocket控制器
use crate::modules::system::controller::monitor::{cache_controller, server_controller, sys_logininfor_controller, sys_operlog_controller, sys_user_online_controller};
use crate::modules::system::controller::wx::{wx_controller};
use crate::modules::system::domain::vo::RespVO;

pub fn build_server(app_state: &'static AppState) -> Server {
    // 按 CPU 核数显式配置 worker 数量（与默认一致，显式化便于运维预期与调优）。
    let workers = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    // AppState 廉价 Clone（内部均 Arc / 连接池共享），克隆一份供闭包 move 捕获；
    // 每个 worker 再通过 web::Data::new(st.clone()) 各持一份 Arc 包裹的同一实例。
    let st = app_state.clone();
    // CORS 警告只在启动时打印一次（避免每个 worker 各打一条）。
    if st.config.cors_allowed_origins.is_empty() {
        log::warn!("[ruoyi_rust] cors_allowed_origins is empty, allowing any origin");
    }
    HttpServer::new(move || {
        //定义validate 错误的返回json
        //目前只支持Json，form query path需要再写，目前用不上
        // CORS：来源由 config.cors_allowed_origins 白名单控制；
        // 留空时回退「允许任意来源」（便于开发），生产应显式配置。
        let cors_origins = st.config.cors_allowed_origins.clone();
        let mut cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        if cors_origins.is_empty() {
            cors = cors.allow_any_origin();
        } else {
            for origin in &cors_origins {
                cors = cors.allowed_origin(origin);
            }
        }

        let json_config = JsonConfig::default().error_handler(|err, _req| {
            match err {
                Error::Validate(e) => {
                    let err_str = flatten_errors(&e).iter()
                        .map(|(_, field, err)| { format!("{}", err) })
                        .collect::<Vec<_>>()
                        .join("\n");
                    let resp: RespVO<String> = RespVO {
                        code: 500,
                        msg: Some(err_str),
                        data: None,
                    };
                    actix_web::error::InternalError::from_response(e, resp.resp_json()).into()
                }
                _ => {
                    actix_web::error::InternalError::from_response(err, HttpResponse::BadRequest().finish()).into()
                }
            }
        });
        let base_api = app_state.config.base_api.clone();
        App::new()
            .app_data(web::Data::new(st.clone()))
            .app_data(json_config)
            .app_data(web::PayloadConfig::new(50 * 1024 * 1024))
            // 统一鉴权中间件：JWT 校验 + Redis 登录态校验 + casbin 权限校验。
            // 未注册时（历史状态）token 失效会穿透到 handler 的 get_user_cache(...).unwrap()
            // 直接 panic，表现为连接被无响应关闭。白名单见 application-*.yml。
            .wrap(crate::middleware::auth_actix::Auth)
            .wrap(cors)
            // 响应压缩：actix-web 默认 feature 已含 gzip / brotli / zstd，无需额外依赖。
            // 放在最外层，使鉴权中间件产生的 401/403 JSON 也能被压缩；
            // 菜单路由、字典、列表类接口响应常达 100KB+，压缩后传输量可降 70% 以上。
            .wrap(Compress::default())
            .service(web::scope(&base_api).service(
                web::scope("/system") //系统应用（镜像 org.dromara.system.controller.system）
                    // ============================================================
                    // 路由顺序铁律：actix 按注册顺序匹配，同一层级的固定路径
                    // （/user/profile、/role/optionselect、/post/deptTree ...）
                    // 必须全部注册在 /xxx/{id} 动态路由之前，否则会被动态路由吞掉，
                    // 表现为 500（i64 解析失败）或 panic（unwrap on Err）。
                    // ============================================================

                    // 菜单管理 /system/menu（动态 /menu/{menu_id} 放最后）
                    .service(sys_menu_controller::query_menu)
                    .service(sys_menu_controller::routers)
                    .service(sys_menu_controller::treeselect)
                    .service(sys_menu_controller::role_menu_treeselect)
                    .service(sys_menu_controller::add)
                    .service(sys_menu_controller::update)
                    .service(sys_menu_controller::cascade_delete)
                    .service(sys_menu_controller::remove)
                    .service(sys_menu_controller::detail)

                    // 文件上传下载（Rust扩展，保留）
                    .service(sys_file_controller::upload_document)
                    .service(sys_file_controller::upload_form)

                    // 用户管理 /system/user（固定路径全部前置）
                    .service(sys_user_controller::page)
                    .service(sys_user_controller::get_dept_tree)
                    .service(sys_user_controller::get_info)
                    .service(sys_user_controller::optionselect)
                    .service(sys_user_controller::list_by_dept)
                    .service(sys_user_controller::set_auth_roles)
                    .service(sys_user_controller::get_auth_roles)
                    .service(sys_user_controller::change_status)
                    .service(sys_user_controller::reset_pwd)
                    .service(sys_user_controller::export)
                    .service(sys_user_controller::import_template)
                    .service(sys_user_controller::import_data)
                    .service(sys_user_controller::add)
                    .service(sys_user_controller::update)

                    // 个人信息 /system/user/profile —— 必须在 /user/{user_id} 之前
                    .service(sys_profile_controller::profile)
                    .service(sys_profile_controller::update_profile)
                    .service(sys_profile_controller::update_pwd)
                    .service(sys_profile_controller::avatar)

                    // 用户管理：动态路由收尾
                    .service(sys_user_controller::remove)
                    .service(sys_user_controller::detail)
                    .service(sys_user_controller::before_add)

                    // 角色管理 /system/role（optionselect 等固定路径前置）
                    .service(sys_role_controller::page)
                    .service(sys_role_controller::auth_user_list)
                    .service(sys_role_controller::unallocated_user_list)
                    .service(sys_role_controller::dept_tree)
                    .service(sys_role_controller::optionselect)
                    .service(sys_role_controller::change_status)
                    .service(sys_role_controller::cancel_user)
                    .service(sys_role_controller::auth_user_all)
                    .service(sys_role_controller::cancel_user_all)
                    .service(sys_role_controller::data_scope)
                    .service(sys_role_controller::export)
                    .service(sys_role_controller::add)
                    .service(sys_role_controller::update)
                    .service(sys_role_controller::remove)
                    .service(sys_role_controller::detail)

                    // 字典类型 /system/dict/type（refreshCache 为 DELETE 固定路径，须在 remove 前）
                    .service(sys_dict_type_controller::optionselect)
                    .service(sys_dict_type_controller::page)
                    .service(sys_dict_type_controller::refresh_cache)
                    .service(sys_dict_type_controller::export)
                    .service(sys_dict_type_controller::add)
                    .service(sys_dict_type_controller::update)
                    .service(sys_dict_type_controller::remove)
                    .service(sys_dict_type_controller::detail)

                    // 字典数据 /system/dict/data
                    .service(sys_dict_data_controller::page)
                    .service(sys_dict_data_controller::get_by_dict_type)
                    .service(sys_dict_data_controller::export)
                    .service(sys_dict_data_controller::add)
                    .service(sys_dict_data_controller::update)
                    .service(sys_dict_data_controller::remove)
                    .service(sys_dict_data_controller::detail)

                    // 参数配置 /system/config
                    .service(sys_config_controller::page)
                    .service(sys_config_controller::get_config_key)
                    .service(sys_config_controller::update_by_key)
                    .service(sys_config_controller::refresh_cache)
                    .service(sys_config_controller::export)
                    .service(sys_config_controller::add)
                    .service(sys_config_controller::update)
                    .service(sys_config_controller::remove)
                    .service(sys_config_controller::detail)

                    // 部门管理 /system/dept（optionselect 前置）
                    .service(sys_dept_controller::page)
                    .service(sys_dept_controller::optionselect)
                    .service(sys_dept_controller::list_exclude)
                    .service(sys_dept_controller::add)
                    .service(sys_dept_controller::update)
                    .service(sys_dept_controller::remove)
                    .service(sys_dept_controller::detail)

                    // 岗位管理 /system/post（optionselect / deptTree 前置）
                    .service(sys_post_controller::page)
                    .service(sys_post_controller::optionselect)
                    .service(sys_post_controller::dept_tree)
                    .service(sys_post_controller::export)
                    .service(sys_post_controller::add)
                    .service(sys_post_controller::update)
                    .service(sys_post_controller::remove)
                    .service(sys_post_controller::detail)

                    // 通知公告 /system/notice
                    .service(sys_notice_controller::page)
                    .service(sys_notice_controller::add)
                    .service(sys_notice_controller::update)
                    .service(sys_notice_controller::remove)
                    .service(sys_notice_controller::detail)
            ).service(
                web::scope("/monitor") //系统监控（镜像 org.dromara.system.controller.monitor）
                    // 同样是固定路径前置、动态 /{id} 收尾
                    .service(sys_logininfor_controller::page)
                    .service(sys_logininfor_controller::clean)
                    .service(sys_logininfor_controller::export)
                    .service(sys_logininfor_controller::unlock)
                    .service(sys_logininfor_controller::remove)

                    .service(sys_user_online_controller::page)
                    .service(sys_user_online_controller::get_online)
                    .service(sys_user_online_controller::remove_self)
                    .service(sys_user_online_controller::force_logout)

                    .service(server_controller::server_info)

                    .service(cache_controller::get_info)
                    .service(cache_controller::get_names)
                    .service(cache_controller::clear_cache_all)
                    .service(cache_controller::get_keys)
                    .service(cache_controller::get_value)
                    .service(cache_controller::clear_cache_name)
                    .service(cache_controller::clear_cache_key)

                    .service(sys_operlog_controller::page)
                    .service(sys_operlog_controller::export)
                    .service(sys_operlog_controller::clean)
                    .service(sys_operlog_controller::remove)
            ).service(
                web::scope("/wx") //Rust扩展，保留
                    .service(wx_controller::wecallback)
            ).service(
                web::scope("/auth") //认证（镜像 org.dromara.web.controller.AuthController）
                    .route("/login", web::post().to(sys_auth_controller::login))
                    .route("/logout", web::post().to(sys_auth_controller::logout))
                    .route("/code", web::get().to(img_controller::captcha))
                    .route("/register", web::post().to(sys_auth_controller::register))
                    .route("/tenant/list", web::get().to(sys_auth_controller::tenant_list))
                    .route("/social/callback", web::post().to(sys_auth_controller::social_callback))
            ).service(
                web::scope("/resource") //SSE（镜像 org.dromara.common.sse.controller.SseController）
                    .route("/sse", web::get().to(system_sse::sse_connect))
                    .route("/sse/close", web::get().to(system_sse::sse_close))
            ).service(
                web::scope("/demo") //代码生成器产出的演示模块（codegen 生成）
                    .configure(crate::modules::demo::controller::init)
            )
            .service(
                web::scope("/tool") //系统工具（镜像 org.dromara.tool）
                    .service(web::scope("/gen").configure(crate::modules::tool::controller::init))
            ).service(
                web::scope("/job") //定时任务引擎（自研轮询调度，替代 ruoyi-job）
                    .configure(crate::modules::job::controller::init)
            ).service(
                web::scope("/workflow") //工作流业务接口（镜像 org.dromara.workflow）
                    .configure(crate::modules::workflow::controller::init)
            ).configure(|cfg: &mut web::ServiceConfig| {
                // warm-flow 设计器接口（/warm-flow/**、/warm-flow-ui/**）。
                // 引擎未初始化时不挂载，避免整个后台因工作流不可用而启动失败。
                if let Some(engine) = crate::modules::workflow::state::engine() {
                    let ui: std::sync::Arc<dyn warm_flow_web::UiExtension> =
                        crate::modules::workflow::ui::ui().clone();
                    cfg.app_data(web::Data::new(
                        warm_flow_web::FlowWebState::new(engine.clone()).with_ui(ui),
                    ));
                    cfg.configure(warm_flow_web::configure);
                }
            })
            .route("/ws/{user_id}", web::get().to(system_websocket::websocket_handler)) //Rust扩展，保留
            .route("/profile/avatar/{file}", web::get().to(sys_profile_controller::serve_avatar)) //头像静态资源，镜像Spring /profile/**

            // ============================================================
            // RuoYi 标准前端（ruoyi-vue3）顶层路径别名
            // 前端 src/api/login.js 调 /captchaImage、/login、/logout、/register、/getInfo，
            // 菜单调 /getRouters（均不带 /auth、/system 前缀）。这里复用镜像 handler，
            // 后端同时兼容 RuoYi 标准前端，前端零改动。
            // ============================================================
            .route("/captchaImage", web::get().to(img_controller::captcha))
            .route("/login", web::post().to(sys_auth_controller::login))
            .route("/logout", web::post().to(sys_auth_controller::logout))
            .route("/register", web::post().to(sys_auth_controller::register))
            .service(sys_user_controller::get_info_alias)
            .service(sys_menu_controller::routers_alias)
            )
    })
        .workers(workers)
        .bind(&app_state.config.server_url).unwrap()
        .run()
}
