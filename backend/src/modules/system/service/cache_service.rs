use std::format;
use crate::config::config::ApplicationConfig;
use crate::error::{ Result};
use crate::modules::system::service::{MemService, RedisService};
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
}

pub struct CacheService {
    pub inner: Box<dyn ICacheService>,
}

impl CacheService {
    pub fn new(cfg: &ApplicationConfig) -> crate::error::Result<Self> {
        match cfg.cache_type.as_str() {
            "mem" => {
                println!("[ruoyi_rust] cache_type: mem");
                Ok(Self {
                    inner: Box::new(MemService::default()),
                })
            }
            "redis" => {
                println!("[ruoyi_rust] cache_type: redis");
                Ok(Self {
                    inner: Box::new(RedisService::new(&cfg.web_redis_url)?),
                })
            }
            e => {
                panic!(
                    "[ruoyi_rust] unknown of cache_type: \"{}\",current support 'mem' or 'redis'",
                    e
                );
            }
        }
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
}
