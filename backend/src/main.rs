use backend::modules::system::service::CONTEXT;
use backend::http_server;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_doc_comments)]
/// use tokio,because Rbatis specifies the runtime-tokio
fn main() -> std::io::Result<()> {
    // debug 编译下 sea-orm 大 Model/ActiveModel 的 future 栈帧很大，
    // tokio::main 的 main future 默认 poll 在 1MB 的 OS 主线程上会栈溢出，
    // 因此把整个异步入口放进 64MB 栈的自定义线程。
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(async_main)
        .expect("failed to spawn main thread")
        .join()
        .unwrap_or_else(|_| std::process::exit(1))
}

/// 栈溢出排查用：打点函数已移至 lib（backend::ckpt），bin 内通过 use 引入
use backend::ckpt;

#[tokio::main]
async fn async_main() -> std::io::Result<()> {
    //日志追加器
    backend::config::log::init_log();

    // 建立 sea-orm 连接池（唯一数据访问入口，全进程只建一次），
    // 并把 db + redis + config 一并写入全局 AppState。
    // 注意：CONTEXT.init_pool() 已废弃并删除——它只是转发到 init_db()，
    // 再调一次会白白多建一个池（OnceCell 已占用，新池被静默丢弃）。
    backend::db::init_db()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("db init failed: {}", e)))?;
    // AppState 已就绪：后续 codegen / casbin 复用其连接。
    let app_state = backend::state::state()
        .expect("[ruoyi_rust] app state not initialized after init_db");
    // 代码生成引擎复用主连接池（避免同进程建两个池）
    codegen::db::init_external(&app_state.db);
    // casbin enforcer：从 casbin_rule 表加载策略；失败仅记日志，不阻断启动。
    // 默认 casbin_rule 为空 -> 中间件降级为仅 token 校验（开箱可用，不误杀）。
    // 需要启用细粒度 RBAC 时，先调用一次 backend::casbin::bootstrap_policies()
    // （从现有角色/菜单表播种策略），或手动写入 casbin_rule。
    backend::casbin::init_enforcer(&app_state.db).await;
    // 过渡期策略播种（幂等）：从现有角色/菜单/用户-角色表生成 casbin_rule。
    // 鉴权已改为 fail-closed，若策略为空则非 admin 用户会被拒绝，故此处确保策略存在。
    backend::casbin::bootstrap_policies().await;

    // 定时任务引擎已改为自研「方言无关轮询调度器」：init_engine / start_workers 仅为
    // 历史兼容的 no-op（轮询调度复用共享连接，无需独立连接池 / apalis worker）。
    // 真正的调度由下方 start_scheduler 拉起；失败仅告警不阻断启动。
    if let Err(e) = backend::modules::job::init_engine(&app_state.config.database_url).await {
        eprintln!("[WARN] job engine init failed: {e}");
    } else if let Err(e) = backend::modules::job::start_workers().await {
        eprintln!("[WARN] job workers start failed: {e}");
    }
    // 初始化 sys_job / sys_job_log 表（幂等），并启动对标 RuoYi sys_job 的动态调度器
    if let Err(e) = backend::modules::job::service::ensure_tables(&app_state.db).await {
        eprintln!("[WARN] job tables init failed: {e}");
    }
    backend::modules::job::service::start_scheduler();

    // 工作流引擎（warm-flow）：构造 SeaORM 存储 + RuoYi 权限处理器 + 设计器扩展点，
    // 并在库里没有已发布的 leave1 时自动导入示例流程。
    // 失败仅告警：工作流不可用不影响系统其它功能（`/workflow`、`/warm-flow` 路由不会挂载）。
    if let Err(e) = backend::modules::workflow::init().await {
        eprintln!("[WARN] workflow init failed: {e}");
    }

    // 审计日志（登录/操作日志）批量写入通道：请求侧只做一次 try_send，
    // 后台协程按 200 条 / 500ms 聚合后一条 INSERT 落库，避免每条日志一次事务。
    backend::util::log_sink::init();

    // 缓存周期维护（对 moka 后端有效，redis/mem 为空操作）：定期 run_pending_tasks，
    // 保证低流量时段过期条目也能被真正驱逐、计数被刷新；cache_maintain_secs=0 时关闭。
    CONTEXT
        .cache_service
        .spawn_maintainer(app_state.config.cache_maintain_secs);
    println!(
        "[ruoyi_rust] cache backend: {}",
        CONTEXT.cache_service.backend()
    );

    ckpt("step1: before dict update_cache");
    if let Err(e) = CONTEXT.sys_dict_data_service.update_cache().await {
        eprintln!("[FATAL] dict update_cache failed: {e}");
        return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("dict cache: {e}")));
    }
    ckpt("step2: dict cache ok");
    // dbg!("init sys_dict data ok! 22");
    if let Err(e) = CONTEXT.sys_config_service.loading_config_cache().await {
        eprintln!("[FATAL] config loading_config_cache failed: {e}");
        return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("config cache: {e}")));
    }
    ckpt("step3: config cache ok");
    //dbg!("init sys_config data ok! 33");
    if let Err(e) = CONTEXT.sys_menu_service.update_cache().await {
        eprintln!("[FATAL] menu update_cache failed: {e}");
        return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("menu cache: {e}")));
    }
    ckpt("step4: menu cache ok");
    //dbg!("init sys_menu data ok! 44");
    ckpt("step5: building http server...");
    // 启动 HTTP 服务；这里会一直阻塞，直到进程收到停止信号
    let result = http_server::build_server(app_state).await;
    // 服务停止后再优雅关闭连接池，等待连接归还
    backend::db::close_db().await;
    result
}
