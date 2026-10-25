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

/// Config
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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
        // 安全：JWT 密钥不再硬编码。优先读环境变量 JWT_SECRET；否则读取/生成
        // 持久化的随机密钥文件（./jwt_secret.key），避免源码内弱密钥被离线爆破。
        result.jwt_secret = resolve_jwt_secret(result.jwt_secret);
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

/// 解析 JWT 密钥：
/// 1) 环境变量 `JWT_SECRET` 非空 -> 直接使用（推荐生产用法）；
/// 2) 配置文件里的值非空且不是历史弱默认值 -> 使用；
/// 3) 否则读取 `./jwt_secret.key`，不存在则生成随机密钥并持久化（重启后 token 仍有效）。
fn resolve_jwt_secret(current: String) -> String {
    if let Ok(s) = std::env::var("JWT_SECRET") {
        if !s.trim().is_empty() {
            return s;
        }
    }
    // 历史提交里泄露的弱默认值，视为未配置
    const WEAK_DEFAULT: &str = "ybgdefghijklmnopqrstuvwxyz";
    let cur = current.trim();
    if !cur.is_empty() && cur != WEAK_DEFAULT {
        return cur.to_string();
    }
    let path = "./jwt_secret.key";
    if let Ok(s) = fs::read_to_string(path) {
        let s = s.trim().to_string();
        if !s.is_empty() {
            return s;
        }
    }
    // 用 uuid v4 拼接生成 256bit 随机十六进制串，无需额外依赖
    let secret = format!(
        "{}{}",
        uuid::Uuid::new_v4().simple(),
        uuid::Uuid::new_v4().simple()
    );
    if let Err(e) = fs::write(path, &secret) {
        println!("[ruoyi_rust] warn: failed to persist jwt secret to {path}: {e}");
    } else {
        println!("[ruoyi_rust] generated random JWT secret at {path}");
    }
    secret
}
