//! 进程内缓存后端：`moka` 实现 `ICacheService`（`cache_type = "moka"`，本项目默认）。
//!
//! ## 为什么用它替代原 `MemService`
//! 原 `MemService` 有两个硬伤：
//! 1. 每次读写都先 `recycling()` **全表扫描**判过期：O(n) 且全程抢全局锁，热路径上随缓存变大而变慢；
//! 2. **无容量上限**，只增不减，长时间运行必然内存膨胀。
//!
//! moka 提供 TinyLFU/LRU 淘汰、加权容量预算、以及后台过期维护，上述问题迎刃而解，
//! 且不再需要任何手工回收逻辑。
//!
//! ## per-entry TTL 的实现（本文件关键）
//! `moka::future::Cache` 只有 `insert(k, v)`——**没有** sync 版才有的 `insert_with_ttl`，
//! 所以「每条记录各自不同的过期时间」必须走 `CacheBuilder::expire_after(impl Expiry<K,V>)`。
//!
//! 这里把过期时刻 `expire_at: Option<Instant>` 直接编进 value（`CacheValue`），
//! `Expiry::expire_after_create/update` 从 value 读出来换算成剩余时长返回。
//! 好处：**不需要任何外部过期注册表**，条目被淘汰时其过期元数据自动随之消失，
//! 不会出现「索引里堆积无效 key」的内存泄漏。

use std::time::{Duration, Instant};

use futures_util::future::BoxFuture;
use moka::future::Cache;
use moka::Expiry;

use crate::config::config::ApplicationConfig;
use crate::error::Result;
// 通配符匹配是跨进程内后端（moka / mem）共用的能力，统一放在 CacheService 门面里。
use crate::modules::system::service::cache_service::glob_match;
use crate::modules::system::service::ICacheService;

/// 缓存值：携带自身的过期时刻，供 `Expiry` 决策。
#[derive(Clone)]
pub struct CacheValue {
    pub value: String,
    /// `Some(t)` = 绝对过期时刻；`None` = 无过期时间（仍受 builder 全局 time_to_live 约束）。
    pub expire_at: Option<Instant>,
}

impl CacheValue {
    /// 按调用方给出的存活时长构造（`None` = 永不过期）。
    fn new(value: String, ex: Option<Duration>) -> Self {
        Self {
            value,
            expire_at: ex.map(|d| Instant::now() + d),
        }
    }

    /// 相对 `now` 的剩余存活时长；无过期时间返回 `None`。
    fn remaining(&self, now: Instant) -> Option<Duration> {
        self.expire_at.map(|at| at.saturating_duration_since(now))
    }
}

/// 依据 value 内嵌的 `expire_at`，为**每个条目**推导各自的 TTL。
struct CacheValueExpiry;

impl Expiry<String, CacheValue> for CacheValueExpiry {
    fn expire_after_create(
        &self,
        _key: &String,
        value: &CacheValue,
        current_time: Instant,
    ) -> Option<Duration> {
        value.remaining(current_time)
    }

    fn expire_after_update(
        &self,
        _key: &String,
        value: &CacheValue,
        current_time: Instant,
        _current_duration: Option<Duration>,
    ) -> Option<Duration> {
        // 更新（含 get_string_refresh_ttl 的重插续期）以新的 expire_at 为准。
        value.remaining(current_time)
    }
    // expire_after_read 保持默认实现：读操作**不**改变过期时间，与 Redis 的 TTL 语义一致
    // （区别于 time_to_idle）；需要续期时显式走 get_string_refresh_ttl。
}

/// Redis 剩余 TTL 约定的两个哨兵值。
/// -2 = key 不存在；-1 = key 存在但没有过期时间。
const TTL_KEY_MISSING: i64 = -2;
const TTL_NO_EXPIRE: i64 = -1;

pub struct MokaService {
    cache: Cache<String, CacheValue>,
}

impl MokaService {
    /// 按配置构造：加权容量（字节预算）+ 可选全局 TTL/TTI 兜底。
    pub fn new(cfg: &ApplicationConfig) -> Self {
        // weigher 按 key+value 字节计，让 max_capacity 变成「内存预算」而非条目数上限，
        // 避免个别大 value（如整份菜单/字典 JSON）挤爆内存。
        let capacity_bytes = cfg.cache_max_capacity_mb.max(1) as u64 * 1024 * 1024;
        let mut builder = Cache::builder()
            .weigher(|k: &String, v: &CacheValue| -> u32 {
                (k.len() + v.value.len()).try_into().unwrap_or(u32::MAX)
            })
            .max_capacity(capacity_bytes);

        if cfg.cache_ttl_secs > 0 {
            builder = builder.time_to_live(Duration::from_secs(cfg.cache_ttl_secs));
        }
        if cfg.cache_time_to_idle_secs > 0 {
            builder = builder.time_to_idle(Duration::from_secs(cfg.cache_time_to_idle_secs));
        }

        let cache = builder.expire_after(CacheValueExpiry).build();
        println!(
            "[ruoyi_rust] cache_type: moka (capacity={}MB, ttl={}s, tti={}s)",
            cfg.cache_max_capacity_mb, cfg.cache_ttl_secs, cfg.cache_time_to_idle_secs
        );
        Self { cache }
    }

    /// 直接由既有 Cache 构造（单测 / 自定义 builder 用）。
    pub fn from_cache(cache: Cache<String, CacheValue>) -> Self {
        Self { cache }
    }
}

impl ICacheService for MokaService {
    fn set_string(&self, k: &str, v: &str) -> BoxFuture<'_, Result<String>> {
        let k = k.to_string();
        let entry = CacheValue::new(v.to_string(), None);
        Box::pin(async move {
            let value = entry.value.clone();
            self.cache.insert(k, entry).await;
            Ok(value)
        })
    }

    fn get_string(&self, k: &str) -> BoxFuture<'_, Result<String>> {
        let k = k.to_string();
        Box::pin(async move {
            Ok(self
                .cache
                .get(&k)
                .await
                .map(|v| v.value)
                .unwrap_or_default())
        })
    }

    /// set_string Automatically expire
    fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> BoxFuture<'_, Result<String>> {
        let k = k.to_string();
        let entry = CacheValue::new(v.to_string(), ex);
        Box::pin(async move {
            let value = entry.value.clone();
            self.cache.insert(k, entry).await;
            Ok(value)
        })
    }

    /// GET + 续期：取到值后必要时按新 TTL 重插（触发 `Expiry::expire_after_update`）。
    ///
    /// **续期降频**：只有剩余寿命不足新 TTL 的一半时才写回。
    /// 鉴权中间件每个请求都会调本方法，若无条件 `insert`，等于每请求一次写：
    /// moka 写路径要走并发 HashMap 的写锁 + 触发 housekeeping，多核下是明确的竞争点；
    /// 而 token 续期本身只需要「别提前过期」，没必要每请求刷新精确到期时刻。
    fn get_string_refresh_ttl(&self, k: &str, time_sec: i32) -> BoxFuture<'_, Result<String>> {
        let k = k.to_string();
        let secs = Duration::from_secs(time_sec.max(1) as u64);
        Box::pin(async move {
            let Some(entry) = self.cache.get(&k).await else {
                return Ok(String::new());
            };
            let value = entry.value.clone();
            let need_refresh = match entry.remaining(Instant::now()) {
                Some(remaining) => remaining < secs / 2,
                // 无过期时间的条目补一次绝对到期时刻，与 Redis EXPIRE 语义对齐
                None => true,
            };
            if need_refresh {
                self.cache
                    .insert(k, CacheValue::new(value.clone(), Some(secs)))
                    .await;
            }
            Ok(value)
        })
    }

    fn ttl(&self, k: &str) -> BoxFuture<'_, Result<i64>> {
        let k = k.to_string();
        Box::pin(async move {
            match self.cache.get(&k).await {
                None => Ok(TTL_KEY_MISSING),
                Some(entry) => match entry.remaining(Instant::now()) {
                    None => Ok(TTL_NO_EXPIRE),
                    Some(d) => Ok(d.as_secs() as i64),
                },
            }
        })
    }

    fn del(&self, k: &str) -> BoxFuture<'_, Result<bool>> {
        let k = k.to_string();
        Box::pin(async move { Ok(self.cache.remove(&k).await.is_some()) })
    }

    /// 通配符取 key（`*` = 任意长度）。缓存监控页与在线用户列表依赖它。
    fn keys(&self, pattern: &str) -> BoxFuture<'_, Result<Vec<String>>> {
        let pattern = pattern.to_string();
        Box::pin(async move {
            // 先维护一轮：让已过期条目真正出局，否则遍历结果里会残留「逻辑已过期」的 key。
            self.cache.run_pending_tasks().await;
            let mut out: Vec<String> = Vec::new();
            for (k, _v) in self.cache.iter() {
                if glob_match(&pattern, k.as_ref()) {
                    out.push(k.as_ref().clone());
                }
            }
            out.sort();
            Ok(out)
        })
    }

    /// 批量取值：moka 无 MGET 语义，逐个 get（本地内存无网络往返，开销可忽略）。
    fn mget(&self, keys: Vec<String>) -> BoxFuture<'_, Result<Vec<String>>> {
        Box::pin(async move {
            if keys.is_empty() {
                return Ok(vec![]);
            }
            let mut out = Vec::with_capacity(keys.len());
            for k in keys {
                out.push(
                    self.cache
                        .get(&k)
                        .await
                        .map(|v| v.value)
                        .unwrap_or_default(),
                );
            }
            Ok(out)
        })
    }

    fn expire(&self, k: &str, time_sec: i32) -> BoxFuture<'_, Result<bool>> {
        let k = k.to_string();
        let secs = Duration::from_secs(time_sec.max(1) as u64);
        Box::pin(async move {
            let Some(entry) = self.cache.get(&k).await else {
                return Ok(false);
            };
            self.cache
                .insert(k, CacheValue::new(entry.value, Some(secs)))
                .await;
            Ok(true)
        })
    }

    // ---- 以下 3 个是 Redis 特有语义（hash / 库选择），本项目无调用点 ----
    fn hgetall(&self, _k: &str) -> BoxFuture<'_, Result<Vec<String>>> {
        Box::pin(async move {
            Err(crate::error::Error::from(
                "moka cache: HGETALL not supported (redis-only)",
            ))
        })
    }

    fn hset(&self, _k: &str, _f: &str, _v: &str) -> BoxFuture<'_, Result<u64>> {
        Box::pin(async move {
            Err(crate::error::Error::from(
                "moka cache: HSET not supported (redis-only)",
            ))
        })
    }

    fn select(&self, _db: &str) -> BoxFuture<'_, Result<()>> {
        Box::pin(async move {
            Err(crate::error::Error::from(
                "moka cache: SELECT not supported (redis-only)",
            ))
        })
    }

    fn run_pending_tasks(&self) -> BoxFuture<'_, ()> {
        Box::pin(async move { self.cache.run_pending_tasks().await })
    }

    fn backend_name(&self) -> &'static str {
        "moka"
    }

    fn entry_count(&self) -> BoxFuture<'_, u64> {
        // moka 的 entry_count 是「最终一致」的（由 housekeeping 异步更新），
        // 读取前先跑一轮维护，否则刚写入的条目可能数不到。
        Box::pin(async move {
            self.cache.run_pending_tasks().await;
            self.cache.entry_count()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn svc() -> MokaService {
        MokaService::from_cache(
            Cache::builder()
                .max_capacity(1024 * 1024)
                .expire_after(CacheValueExpiry)
                .build(),
        )
    }

    #[tokio::test]
    async fn set_get_roundtrip() {
        let s = svc();
        s.set_string("k1", "v1").await.unwrap();
        assert_eq!(s.get_string("k1").await.unwrap(), "v1");
        // 未设置的 key 返回空串（与 Redis/Mem 实现一致）
        assert_eq!(s.get_string("nope").await.unwrap(), "");
    }

    #[tokio::test]
    async fn per_entry_ttl_expires() {
        let s = svc();
        s.set_string_ex("t1", "v", Some(Duration::from_millis(50)))
            .await
            .unwrap();
        assert_eq!(s.get_string("t1").await.unwrap(), "v");
        tokio::time::sleep(Duration::from_millis(90)).await;
        assert_eq!(s.get_string("t1").await.unwrap(), "");
        s.run_pending_tasks().await;
        assert_eq!(s.keys("t1*").await.unwrap(), Vec::<String>::new());
    }

    #[tokio::test]
    async fn ttl_semantics() {
        let s = svc();
        // -2：key 不存在
        assert_eq!(s.ttl("missing").await.unwrap(), -2);
        // -1：存在但无过期时间
        s.set_string("forever", "v").await.unwrap();
        assert_eq!(s.ttl("forever").await.unwrap(), -1);
        // 正数：剩余秒数（向下取整，插入到读取之间已过去若干微秒，故为 29 或 30）
        s.set_string_ex("soon", "v", Some(Duration::from_secs(30)))
            .await
            .unwrap();
        let ttl = s.ttl("soon").await.unwrap();
        assert!(
            (29..=30).contains(&ttl),
            "ttl should round down to 29 or 30, got {ttl}"
        );
    }

    #[tokio::test]
    async fn refresh_ttl_extends_life() {
        let s = svc();
        s.set_string_ex("tok", "v", Some(Duration::from_millis(200)))
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(120)).await;
        // 续到 2s：此刻距首次过期只剩 ~80ms（< 新 TTL 的一半），应触发写回续期
        assert_eq!(s.get_string_refresh_ttl("tok", 2).await.unwrap(), "v");
        tokio::time::sleep(Duration::from_millis(250)).await;
        assert_eq!(s.get_string("tok").await.unwrap(), "v");
        assert_eq!(s.ttl("tok").await.unwrap(), 1);
    }

    #[tokio::test]
    async fn refresh_ttl_throttled_when_remaining_is_large() {
        let s = svc();
        s.set_string_ex("tok2", "v", Some(Duration::from_secs(3600)))
            .await
            .unwrap();
        // 剩余 3600s > 新 TTL(60s) 的一半 -> 不写回，剩余寿命保持不变
        assert_eq!(s.get_string_refresh_ttl("tok2", 60).await.unwrap(), "v");
        assert!(s.ttl("tok2").await.unwrap() > 3000);
    }

    #[tokio::test]
    async fn keys_prefix_filter() {
        let s = svc();
        s.set_string("sys_config:a", "1").await.unwrap();
        s.set_string("sys_config:b", "2").await.unwrap();
        s.set_string("sys_menu:c", "3").await.unwrap();
        let got = s.keys("sys_config:*").await.unwrap();
        assert_eq!(got, vec!["sys_config:a", "sys_config:b"]);
        assert_eq!(s.keys("sys_menu:*").await.unwrap(), vec!["sys_menu:c"]);
    }

    #[tokio::test]
    async fn del_returns_whether_existed() {
        let s = svc();
        s.set_string("d1", "v").await.unwrap();
        assert!(s.del("d1").await.unwrap());
        assert!(!s.del("d1").await.unwrap());
    }

    // 纯同步函数：不能挂 #[tokio::test]（否则宏会断言函数体含 await 且为 async fn）
    #[test]
    fn glob_semantics() {
        assert!(glob_match("sys_config:*", "sys_config:a"));
        assert!(!glob_match("sys_config:*", "sys_menu:a"));
        assert!(glob_match("*a*", "xyzaz"));
        assert!(glob_match("exact", "exact"));
        assert!(!glob_match("exact", "exactly"));
    }
}
