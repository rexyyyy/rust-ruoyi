use std::time::Duration;

use crate::error::{Error, Result};
use crate::modules::system::service::ICacheService;
use futures_util::future::BoxFuture;
use log::error;

use redis::aio::ConnectionManager;
use redis::RedisResult;
use tokio::sync::OnceCell;

///Redis Cache service
///
/// 使用 redis 官方异步 `ConnectionManager`：
/// - 单条连接多路复用（multiplexed），`ConnectionManager` 内部 `Arc`，克隆廉价；
/// - 断线自动重连，避免同步客户端「每条命令新建 TCP 连接」的开销与阻塞；
/// - 所有命令走 `query_async`，不再阻塞 tokio worker 线程。
///
/// 注：不再派生 `Clone`/`Debug`（`OnceCell<ConnectionManager>` 不满足）。
/// 本服务为进程内单例，无需克隆/调试打印。
pub struct RedisService {
    pub client: redis::Client,
    url: String,
    /// 懒初始化 + 全生命周期复用（ConnectionManager 自带重连，无需失效重建）
    conn: OnceCell<ConnectionManager>,
}

impl RedisService {
    pub fn new(url: &str) -> Result<Self> {
        println!("[ruoyi_rust] conncect redis ({})...", url);
        let client = redis::Client::open(url).map_err(|e| {
            let err = format!("RedisService client open fail:{} ({})", e, url);
            log::error!("{}", err);
            Error::from(err)
        })?;
        println!("[ruoyi_rust] conncect redis success!");
        Ok(Self {
            client,
            url: url.to_string(),
            conn: OnceCell::new(),
        })
    }

    /// 取（懒初始化的）多路复用连接。ConnectionManager 克隆廉价。
    pub async fn get_conn(&self) -> Result<ConnectionManager> {
        let mgr = self
            .conn
            .get_or_try_init(|| async { ConnectionManager::new(self.client.clone()).await })
            .await
            .map_err(|e| {
                let err = format!("RedisService connection fail:{}", e);
                error!("{}, {}", self.url, err);
                Error::from(err)
            })?;
        Ok(mgr.clone())
    }
}

impl ICacheService for RedisService {
    fn set_string(&self, k: &str, v: &str) -> BoxFuture<'_, Result<String>> {
        let k = k.to_string();
        let v = v.to_string();
        Box::pin(async move {
            return self.set_string_ex(&k, &v, None).await;
        })
    }

    fn get_string(&self, k: &str) -> BoxFuture<'_, Result<String>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            let result: RedisResult<Option<String>> =
                redis::cmd("GET").arg(&[&k]).query_async(&mut conn).await;
            return match result {
                Ok(v) => Ok(v.unwrap_or_default()),
                Err(e) => Err(Error::from(format!(
                    "RedisService get_string({}) fail:{}",
                    k,
                    e.to_string()
                ))),
            };
        })
    }

    /// GET + EXPIRE 合并为单次 pipeline 往返，用于 token 续期等热路径。
    fn get_string_refresh_ttl(&self, k: &str, time_sec: i32) -> BoxFuture<'_, Result<String>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            let mut pipe = redis::pipe();
            pipe.cmd("GET").arg(&k);
            pipe.cmd("EXPIRE").arg(&k).arg(time_sec);
            let result: RedisResult<(Option<String>, i64)> = pipe.query_async(&mut conn).await;
            return match result {
                Ok((v, _ttl)) => Ok(v.unwrap_or_default()),
                Err(e) => Err(Error::from(format!(
                    "RedisService get_string_refresh_ttl({}) fail:{}",
                    k,
                    e.to_string()
                ))),
            };
        })
    }

    ///set_string Automatically expire
    fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> BoxFuture<'_, Result<String>> {
        let k = k.to_string();
        let v = v.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return if ex.is_none() {
                match redis::cmd("SET").arg(&[k, v]).query_async(&mut conn).await {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::from(format!(
                        "RedisService set_string_ex fail:{}",
                        e.to_string()
                    ))),
                }
            } else {
                match redis::cmd("SET")
                    .arg(&[&k, &v, "EX", &ex.unwrap().as_secs().to_string()])
                    .query_async(&mut conn)
                    .await
                {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::from(format!(
                        "RedisService set_string_ex fail:{}",
                        e.to_string()
                    ))),
                }
            };
        })
    }

    ///get time to live
    fn ttl(&self, k: &str) -> BoxFuture<'_, Result<i64>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return match redis::cmd("TTL").arg(&[k]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService ttl fail:{}",
                    e.to_string()
                ))),
            };
        })
    }
    fn del(&self, k: &str) -> BoxFuture<'_, Result<bool>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return match redis::cmd("DEL").arg(&[k]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService del fail:{}",
                    e.to_string()
                ))),
            };
        })
    }

    /// 用 SCAN 游标分批遍历，替代阻塞式的 KEYS（O(N) 且会卡住 Redis 单线程）。
    fn keys(&self, k: &str) -> BoxFuture<'_, Result<Vec<String>>> {
        let pattern = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            let mut cursor: u64 = 0;
            let mut out: Vec<String> = Vec::new();
            loop {
                let result: RedisResult<(u64, Vec<String>)> = redis::cmd("SCAN")
                    .arg(cursor)
                    .arg("MATCH")
                    .arg(&pattern)
                    .arg("COUNT")
                    .arg(1000)
                    .query_async(&mut conn)
                    .await;
                match result {
                    Ok((next, batch)) => {
                        out.extend(batch);
                        cursor = next;
                        if cursor == 0 {
                            break;
                        }
                    }
                    Err(e) => {
                        return Err(Error::from(format!(
                            "RedisService scan fail:{}",
                            e.to_string()
                        )))
                    }
                }
            }
            Ok(out)
        })
    }

    /// 批量取值（MGET），替代「逐个 GET」的 N+1 往返。
    fn mget(&self, keys: Vec<String>) -> BoxFuture<'_, Result<Vec<String>>> {
        Box::pin(async move {
            if keys.is_empty() {
                return Ok(vec![]);
            }
            let mut conn = self.get_conn().await?;
            let result: RedisResult<Vec<Option<String>>> =
                redis::cmd("MGET").arg(keys).query_async(&mut conn).await;
            match result {
                Ok(vs) => Ok(vs.into_iter().map(|v| v.unwrap_or_default()).collect()),
                Err(e) => Err(Error::from(format!(
                    "RedisService mget fail:{}",
                    e.to_string()
                ))),
            }
        })
    }

    fn hgetall(&self, key: &str) -> BoxFuture<'_, Result<Vec<String>>> {
        let k = key.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return match redis::cmd("HGETALL").arg(&[k]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService HGETALL fail:{}",
                    e.to_string()
                ))),
            };
        })
    }

    fn expire(&self, k: &str, time_sec: i32) -> BoxFuture<'_, Result<bool>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return match redis::cmd("EXPIRE")
                .arg(k)
                .arg(time_sec)
                .query_async(&mut conn)
                .await
            {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService expire fail:{}",
                    e.to_string()
                ))),
            };
        })
    }

    //从在db插入
    fn hset(&self, k: &str, f: &str, v: &str) -> BoxFuture<'_, Result<u64>> {
        let k = k.to_string();
        let f = f.to_string();
        let v = v.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            let res: Result<()> = match redis::cmd("SELECT").arg("21").query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService hset fail:{}",
                    e.to_string()
                ))),
            };
            res?;
            match redis::cmd("HSET").arg(&[k, f, v]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService hset fail:{}",
                    e.to_string()
                ))),
            }
        })
    }
    fn select(&self, db: &str) -> BoxFuture<'_, Result<()>> {
        let db = db.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            match redis::cmd("SELECT").arg(&[db]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService select fail:{}",
                    e.to_string()
                ))),
            }
        })
    }
}
