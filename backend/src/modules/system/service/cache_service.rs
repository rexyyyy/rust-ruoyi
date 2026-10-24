use std::format;
use crate::config::config::ApplicationConfig;
use crate::error::{ Result};
use crate::modules::system::service::{MemService, MokaService, RedisService};
use futures_util::future::BoxFuture;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;

pub trait ICacheService: Sync + Send {
    fn set_string(&self, k: &str, v: &str) -> BoxFuture<'_, Result<String>>;

    fn get_string(&self, k: &str) -> BoxFuture<'_, Result<String>>;

    fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> BoxFuture<'_, Result<String>>;

    /// GET + EXPIRE 单次往返（热路径 token 续期用）。
    fn get_string_refresh_ttl(&self, k: &str, time_sec: i32) -> BoxFuture<'_, Result<String>>;

    fn ttl(&self, k: &str) -> BoxFuture<'_, Result<i64>>;

    fn del(&self, k: &str) -> BoxFuture<'_, Result<bool>>;

    fn keys(&self, k: &str) -> BoxFuture<'_, Result<Vec<String>>>;

    /// 批量取值（MGET），避免 N+1 往返。
    fn mget(&self, keys: Vec<String>) -> BoxFuture<'_, Result<Vec<String>>>;

    fn hgetall(&self, k: &str) -> BoxFuture<'_, Result<Vec<String>>>;

    fn expire(&self, k: &str, time_sec: i32) -> BoxFuture<'_, Result<bool>>;
    //从在db插入
    fn hset(&self, k: &str, f: &str, v: &str) -> BoxFuture<'_, Result<u64>>;
    //切换Db
    fn select(&self, db: &str) -> BoxFuture<'_, Result<()>>;

    /// 触发后端自身的异步维护（如 moka 的过期驱逐 / 计数刷新）。
    /// 默认空实现：没有后台维护的后端（Redis / Mem）无需实现。
    fn run_pending_tasks(&self) -> BoxFuture<'_, ()> {
        Box::pin(async move {})
    }

    /// 缓存后端标识，用于启动日志与缓存监控展示。
    fn backend_name(&self) -> &'static str {
        "unknown"
    }

    /// 当前条目数（近似值；moka 读取前会先跑一轮维护）。不支持时返回 0。
    fn entry_count(&self) -> BoxFuture<'_, u64> {
        Box::pin(async move { 0 })
    }
}

/// 已识别的 `cache_type` 取值，供启动日志 / 校验提示复用。
pub const SUPPORTED_CACHE_TYPES: &[&str] = &["moka", "redis", "mem"];

/// 简易通配符匹配：支持任意个 `*`，与 Redis `KEYS` 的日常用法对齐。
/// 缓存监控/在线用户走的是 `prefix*` 形式，这里对无通配符的情况做了全等快速路径。
///
/// 放在门面里供所有进程内后端复用（`moka` / `mem`）：它们没有 Redis 服务端帮忙做匹配，
/// 必须在本地实现，否则 `/monitor/cache` 这类接口在切换后端后行为会不一致。
pub(crate) fn glob_match(pattern: &str, s: &str) -> bool {
    let parts: Vec<&str> = pattern.split('*').collect();
    // 无通配符：全等比较
    if parts.len() == 1 {
        return pattern == s;
    }
    let mut rest = s;
    // 首个分段必须是前缀
    if !parts[0].is_empty() {
        if !rest.starts_with(parts[0]) {
            return false;
        }
        rest = &rest[parts[0].len()..];
    }
    // 中间分段依次顺序出现（find 返回的字节下标必然落在字符边界上，切片安全）
    let last_idx = parts.len() - 1;
    for p in &parts[1..last_idx] {
        match rest.find(p) {
            Some(i) => rest = &rest[i + p.len()..],
            None => return false,
        }
    }
    // 末段必须是后缀（pattern 以 * 结尾时 last 为空，恒真）
    parts[last_idx].is_empty() || rest.ends_with(parts[last_idx])
}

pub struct CacheService {
    pub inner: Box<dyn ICacheService>,
}

impl CacheService {
    /// 按 `cache_type` 选择缓存后端。
    ///
    /// - `moka`（**默认**）：进程内 TinyLFU + TTL + 加权容量预算，零外部依赖；
    /// - `redis`：多实例共享会话时使用，需要配 `web_redis_url`；
    /// - `mem`：旧 HashMap 实现，仅作对照保留，不建议生产使用。
    ///
    /// 配置文件留空或写成未知值时，一律**回落到 moka 并记 warn**，不再 panic——
    /// 避免笔误导致整个服务起不来。
    pub fn new(cfg: &ApplicationConfig) -> crate::error::Result<Self> {
        let inner: Box<dyn ICacheService> =
            match cfg.cache_type.trim().to_ascii_lowercase().as_str() {
                "redis" => {
                    println!("[ruoyi_rust] cache_type: redis");
                    Box::new(RedisService::new(&cfg.web_redis_url)?)
                }
                "mem" => {
                    println!("[ruoyi_rust] cache_type: mem (legacy HashMap, not recommended)");
                    log::warn!(
                        "[ruoyi_rust] cache_type=mem 已不推荐：无容量上限且每次读写全表扫描，建议改用 moka"
                    );
                    Box::new(MemService::default())
                }
                // moka 是默认后端：显式写明或未配置都走这里
                "moka" | "" => Box::new(MokaService::new(cfg)),
                other => {
                    println!(
                        "[ruoyi_rust] unknown cache_type: \"{}\", fallback to 'moka' (support: {})",
                        other,
                        SUPPORTED_CACHE_TYPES.join(" | ")
                    );
                    log::warn!(
                        "[ruoyi_rust] unknown cache_type: \"{}\", fallback to moka",
                        other
                    );
                    Box::new(MokaService::new(cfg))
                }
            };
        Ok(Self { inner })
    }

    pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        self.inner.set_string(k, v).await
    }

    pub async fn get_string(&self, k: &str) -> Result<String> {
        self.inner.get_string(k).await
    }

    pub async fn set_json<T>(&self, k: &str, v: &T) -> Result<String>
        where
            T: Serialize + Sync,
    {
        self.set_json_ex(k, v, None).await
    }

    pub async fn set_json_ex<T>(&self, k: &str, v: &T, ex: Option<Duration>) -> Result<String>
        where
            T: Serialize + Sync,
    {
        let data = serde_json::to_string(v);
        if data.is_err() {
            return Err(crate::error::Error::from(format!(
                "CacheService set_json fail:{}",
                data.err().unwrap()
            )));
        }
        let data = self.set_string_ex(k, data.unwrap().as_str(), ex).await?;
        Ok(data)
    }

    pub async fn get_json<T>(&self, k: &str) -> Result<T>
        where
            T: DeserializeOwned + Sync,
    {
        let mut r = self.get_string(k).await?;
        if r.is_empty() {
            r = "null".to_string();
        }
        let data: serde_json::Result<T> = serde_json::from_str(r.as_str());
        if data.is_err() {
            return Err(crate::error::Error::from(format!(
                "MemCacheService GET fail:{}",
                data.err().unwrap()
            )));
        }
        Ok(data.unwrap())
    }

    /// 取值并刷新过期时间（单次往返），避免「GET 后再 EXPIRE」两条命令。
    pub async fn get_string_refresh_ttl(&self, k: &str, time_sec: i32) -> Result<String> {
        self.inner.get_string_refresh_ttl(k, time_sec).await
    }

    /// 取值并刷新过期时间后反序列化为 T。
    pub async fn get_json_refresh_ttl<T>(&self, k: &str, time_sec: i32) -> Result<T>
        where
            T: DeserializeOwned + Sync,
    {
        let mut r = self.get_string_refresh_ttl(k, time_sec).await?;
        if r.is_empty() {
            r = "null".to_string();
        }
        let data: serde_json::Result<T> = serde_json::from_str(r.as_str());
        if data.is_err() {
            return Err(crate::error::Error::from(format!(
                "MemCacheService GET fail:{}",
                data.err().unwrap()
            )));
        }
        Ok(data.unwrap())
    }

    /// 批量取值（MGET）。
    pub async fn mget(&self, keys: Vec<String>) -> Result<Vec<String>> {
        self.inner.mget(keys).await
    }

    pub async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String> {
        self.inner.set_string_ex(k, v, ex).await
    }

    pub async fn ttl(&self, k: &str) -> Result<i64> {
        self.inner.ttl(k).await
    }
    pub async fn del(&self, k: &str) -> Result<bool> {
        self.inner.del(k).await
    }
    pub async fn keys(&self, k: &str) -> Result<Vec<String>> {
        self.inner.keys(k).await
    }
    pub async  fn expire(&self, k: &str, time_sec: i32) -> Result<bool> {
        self.inner.expire(k,time_sec).await
    }

    /// 当前缓存后端标识（`moka` / `redis` / `mem`），供启动日志与缓存监控页展示。
    pub fn backend(&self) -> &'static str {
        self.inner.backend_name()
    }

    /// 触发后端自身的异步维护（moka 会驱逐过期条目并刷新计数；redis/mem 为空操作）。
    /// 周期任务零流量时调用它，才能保证过期数据被真正清掉。
    pub async fn run_pending_tasks(&self) {
        self.inner.run_pending_tasks().await
    }

    /// 当前条目数（近似值）。redis / mem 后端返回 0。
    pub async fn entry_count(&self) -> u64 {
        self.inner.entry_count().await
    }

    /// 拉起后台周期维护任务（对 moka 有效，redis/mem 为空操作）。
    ///
    /// 为什么需要：moka 的过期驱逐与计数刷新由访问触发的 housekeeping 完成，
    /// 低流量时段过期条目可能长时间驻留。这里按固定间隔主动 `run_pending_tasks`，
    /// 同时输出一条 debug 级统计日志便于观测。
    ///
    /// `secs = 0` 表示关闭。必须在 tokio runtime 上下文内调用（如 `main` 中）。
    pub fn spawn_maintainer(&'static self, secs: u64) {
        if secs == 0 {
            return;
        }
        let backend = self.backend();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(secs));
            // interval 首个 tick 立即就绪，先消费掉，避免刚启动就空跑一次
            ticker.tick().await;
            loop {
                ticker.tick().await;
                self.run_pending_tasks().await;
                log::debug!(
                    "[ruoyi_rust] cache maintain: backend={}, entries={}",
                    backend,
                    self.entry_count().await
                );
            }
        });
    }
}
