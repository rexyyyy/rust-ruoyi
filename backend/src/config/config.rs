use std::fs;
use std::sync::Arc;
use merge_yaml_hash::{MergeYamlHash};
use once_cell::sync::OnceCell;

/// serde 默认值：未显式配置时默认走进程内 moka 缓存，零外部依赖即可启动。
fn default_cache_type() -> String {
    "moka".to_string()
}
/// moka 默认容量预算 64MB（按 key+value 字节 weigher 计）。
fn default_cache_max_capacity_mb() -> u64 {
    64
}
/// 默认 60 秒做一次缓存维护。
fn default_cache_maintain_secs() -> u64 {
    60
}

/// 开箱即用的默认 JWT 密钥（与 `application.yml` 里 `jwt_secret` 的默认值一致）。
///
/// **定位**：本项目是开源指南/教学项目，目标是「clone 下来 `cargo run` 就能跑」，
/// 因此密钥以明文默认值随仓库分发，避免新人卡在「先配环境变量才能启动」这一步。
/// **生产部署必须替换**：优先用环境变量 `JWT_SECRET` 注入，或直接改 `application-*.yml`；
/// 非环境变量来源在 release 模式下启动时会打印醒目告警。
pub const DEFAULT_JWT_SECRET: &str =
    "d921104b85cf0b33450f00608b499e5e52f8828ec6ebbda7cc5edcdd83d0baea";

/// Config
///
/// 容器级 `#[serde(default = ...)]`：yml 里**缺失**的字段自动取 [`ApplicationConfig::builtin`]
/// 的对应值，不再因为少写一行就整体解析失败（`expect("load config file fail")`）。
/// 注意区分：yml 语法/类型错误仍然会 panic（这类错误必须显式暴露，静默兜底反而坑人）。
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default = "ApplicationConfig::builtin")]
pub struct ApplicationConfig {
    pub debug: bool,
    pub server_url: String,
    pub base_api: String,
    pub web_redis_url: String,
    pub database_url: String,
    pub logic_column: String,
    pub logic_un_deleted: i64,
    pub logic_deleted: i64,
    pub log_dir: String,
    pub log_temp_size: String,
    pub log_pack_compress: String,
    pub log_rolling_type: String,
    pub log_chan_len: Option<usize>,
    pub log_level: String,
    pub sms_cache_send_key_prefix: String,
    pub jwt_secret: String,
    pub white_list_api: Vec<String>,
    /// 缓存后端类型：`moka`（默认，进程内 LRU+TTL）/ `redis` / `mem`（旧 HashMap 实现）。
    /// 配置文件留空或写了未知值时，都回落到 `moka` 并记 warn，不再 panic。
    #[serde(default = "default_cache_type")]
    pub cache_type: String,
    /// moka 加权容量预算（MB）。weigher 按 key+value 字节计，达到上限按 TinyLFU 淘汰。
    #[serde(default = "default_cache_max_capacity_mb")]
    pub cache_max_capacity_mb: u64,
    /// moka 兜底 TTL（秒）：0 = 不设全局上限，每条只按 set_string_ex 传入的过期时间；
    /// >0 时作为「最长存活时间」上限，防止无 TTL 的 key（如字典/菜单）永久驻留。
    #[serde(default)]
    pub cache_ttl_secs: u64,
    /// moka 空闲淘汰时长（秒）：0 = 关闭按空闲淘汰（只按 TTL）。
    #[serde(default)]
    pub cache_time_to_idle_secs: u64,
    /// 缓存周期维护间隔（秒）：定期 run_pending_tasks，使零流量时也能驱逐过期条目、
    /// 刷新 entry_count 计数（moka 计数是最终一致的）。0 = 关闭周期维护。
    #[serde(default = "default_cache_maintain_secs")]
    pub cache_maintain_secs: u64,
    pub login_fail_retry: u64,
    pub login_fail_retry_wait_sec: u64,
    pub captcha_enabled: bool,
    pub captcha_expired_min: u64,
    pub token_expired_min: u64,
    /// CORS 允许的来源白名单。为空时回退为「允许任意来源」（保持开发便利），
    /// 生产环境应在 application-*.yml 显式配置以收敛。
    #[serde(default)]
    pub cors_allowed_origins: Vec<String>
}

/// 进程级唯一的 configs 实例（所有消费方共享同一个 `Arc`）。
///
/// **收敛背景**：历史上存在两条配置源 —— `ServiceContext::default()` 自行
/// `ApplicationConfig::default()` 加载一份，`AppState` 再 `clone()` 一份，
/// 于是同一份配置在进程内存里有两个互不同步的实例（改一边另一边看不到）。
/// 收敛后 `ServiceContext.config` 与 `AppState.config` 指向同一个 `Arc`，
/// 配置数据全进程有且只有一份。
static GLOBAL_CONFIG: OnceCell<Arc<ApplicationConfig>> = OnceCell::new();

/// 取全局配置单例。首次调用时加载 `application.yml` 并叠加 `application-{profile}.yml`。
pub fn global() -> &'static Arc<ApplicationConfig> {
    GLOBAL_CONFIG.get_or_init(|| Arc::new(ApplicationConfig::load()))
}

impl ApplicationConfig {
    /// 内置默认配置（yml 缺字段时的取值，也是「零配置启动」的落点）。
    ///
    /// 与 `application.yml` 保持同值：默认 debug、60800 端口、`/dev-api` 前缀、
    /// MySQL 本机库、moka 进程内缓存（不依赖 Redis 即可启动）。
    fn builtin() -> Self {
        Self {
            debug: true,
            server_url: "0.0.0.0:60800".to_string(),
            base_api: "/dev-api".to_string(),
            web_redis_url: "redis://127.0.0.1:6379".to_string(),
            database_url: "mysql://root:123456@127.0.0.1:3306/ruoyirust".to_string(),
            logic_column: "del".to_string(),
            logic_un_deleted: 0,
            logic_deleted: 1,
            log_dir: "target/logs/".to_string(),
            log_temp_size: "20MB".to_string(),
            log_pack_compress: String::new(),
            log_rolling_type: "KeepNum(20)".to_string(),
            log_chan_len: Some(100_000),
            log_level: "info".to_string(),
            sms_cache_send_key_prefix: "sms:send:".to_string(),
            jwt_secret: DEFAULT_JWT_SECRET.to_string(),
            white_list_api: vec![
                "/".to_string(),
                "/dev-api/auth/login".to_string(),
                "/dev-api/auth/logout".to_string(),
                "/dev-api/auth/code".to_string(),
                "/dev-api/auth/register".to_string(),
                "/dev-api/auth/tenant/list".to_string(),
                "/dev-api/resource/sse".to_string(),
                "/dev-api/resource/sse/close".to_string(),
                "/dev-api/captchaImage".to_string(),
                "/dev-api/login".to_string(),
                "/dev-api/logout".to_string(),
                "/dev-api/register".to_string(),
                "/dev-api/profile/avatar".to_string(),
            ],
            cache_type: "moka".to_string(),
            cache_max_capacity_mb: 64,
            cache_ttl_secs: 0,
            cache_time_to_idle_secs: 0,
            cache_maintain_secs: 60,
            login_fail_retry: 3,
            login_fail_retry_wait_sec: 30,
            captcha_enabled: true,
            captcha_expired_min: 2,
            token_expired_min: 60,
            cors_allowed_origins: Vec::new(),
        }
    }

    /// 真正的文件加载逻辑，**只应由 [`global`] 调用**。
    /// 外部需要配置请走 `ApplicationConfig::default()` 或 [`global`]，避免产生第二份实例。
    fn load() -> Self {

        let k = "profile=";
        let profile = std::env::args_os()
            .find(|v| v.to_str().unwrap_or_default().starts_with(k))
            .map(|v| v.to_str().unwrap_or_default().trim_start_matches(k).to_string());
        let profile = match profile{
            Some(o) => Some(o),
            None => { 
                std::env::var("profile").ok()
             }
        };
        //dbg!(format!("profile 2:{:?}",profile  ));
        
        let profile = match profile{
            Some(o) => Some(o),
            None => { 
                "dev".to_string().into()
                //std::env::var_os("profile").map(|x|x.into_string().unwrap_or_default())
             }
        };
        //dbg!(format!("profile 3:{:?}",profile  ));
        
        let profile = profile.unwrap_or_else(|| "prod".to_string());
        
        //dbg!(format!("profile 4:{:?}",profile  ));
        let profile = profile.trim().to_string();
        println!("loading profile: {}", profile);
        let yml_data = include_str!("../../application.yml");

        let mut hash = MergeYamlHash::new();

        hash.merge(yml_data);
        if profile.len() > 0 {
            match fs::read_to_string(format!("application-{profile}.yml")) {
                Ok(s) => { hash.merge(&s); }
                Err(_) => { println!("Can't loading application-{profile}.yml"); }
            }
        }
        //load config
        let mut result: ApplicationConfig =
            serde_yaml::from_str(&*hash.to_string()).expect("load config file fail");
        // JWT 密钥：yml 明文默认值（开箱即用） + 环境变量 JWT_SECRET 最高优先级覆盖。
        // 单一来源、无外部文件依赖，保证 clone 后零配置即可启动。
        let (secret, source) = resolve_jwt_secret(&result.jwt_secret);
        result.jwt_secret = secret;
        println!("[ruoyi_rust] jwt secret source: {source} (len={})", result.jwt_secret.len());
        if source != "env:JWT_SECRET" {
            println!(
                "[ruoyi_rust] warn: JWT 密钥来自 {source}（仓库内置默认值）。\
                 正式部署请设置环境变量 JWT_SECRET 或改写 application-*.yml 的 jwt_secret。"
            );
        }
        if result.debug {
            //println!("[ruoyi_rust] load config:{:?}", result);
            println!("[ruoyi_rust] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            println!("[ruoyi_rust] ///////////////////// Start On Release Mode ////////////////////////////");
        }
        result
    }
}

impl Default for ApplicationConfig {
    /// 返回全局单例的数据副本。
    ///
    /// 底层数据始终来自 [`global`] 这一份，因此进程内任意多处调用
    /// `ApplicationConfig::default()` 都保持一致，不会再出现「两份不同步的配置」。
    fn default() -> Self {
        global().as_ref().clone()
    }
}

/// 历史提交里泄露过的 / 一眼可猜的弱密钥，视同「未配置」，回落内置默认值。
const WEAK_SECRETS: [&str; 4] = [
    "ybgdefghijklmnopqrstuvwxyz",
    "changeme",
    "change-me",
    "secret",
];

/// 解析 JWT 密钥，返回值 + 来源标记（便于启动时在日志里确认用的是哪一份）。
///
/// 优先级：
/// 1) 环境变量 `JWT_SECRET` 非空 —— 最高优先级（生产推荐，不落盘、不入库）；
/// 2) `application*.yml` 的 `jwt_secret` 非空且非弱值 —— 常规用法；
/// 3) 否则取内置默认值 [`DEFAULT_JWT_SECRET`]。
///
/// **不再生成/读写 `jwt_secret.key`**：运行产物进过版本库反而更像「真密钥泄露」，
/// 且多一层文件 IO 只增加部署复杂度。需要随机密钥时请自行 `JWT_SECRET=$(openssl rand -hex 32)`。
fn resolve_jwt_secret(current: &str) -> (String, &'static str) {
    if let Ok(s) = std::env::var("JWT_SECRET") {
        let s = s.trim();
        if !s.is_empty() {
            return (s.to_string(), "env:JWT_SECRET");
        }
    }
    let cur = current.trim();
    if !cur.is_empty() && !WEAK_SECRETS.contains(&cur) {
        return (cur.to_string(), "yaml");
    }
    (DEFAULT_JWT_SECRET.to_string(), "builtin-default")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_defaults_are_runnable() {
        let c = ApplicationConfig::builtin();
        assert_eq!(c.base_api, "/dev-api");
        assert_eq!(c.cache_type, "moka");
        assert_eq!(c.jwt_secret, DEFAULT_JWT_SECRET);
        assert!(c.white_list_api.contains(&"/dev-api/auth/login".to_string()));
    }

    #[test]
    fn yaml_value_wins_over_builtin() {
        let (s, src) = resolve_jwt_secret("abcdef0123456789");
        assert_eq!(s, "abcdef0123456789");
        assert_eq!(src, "yaml");
    }

    #[test]
    fn weak_and_empty_fall_back_to_builtin() {
        for weak in ["", "   ", "changeme", "ybgdefghijklmnopqrstuvwxyz"] {
            let (s, src) = resolve_jwt_secret(weak);
            assert_eq!(s, DEFAULT_JWT_SECRET, "weak={weak:?} 应回落内置默认值");
            assert_eq!(src, "builtin-default");
        }
    }
}
