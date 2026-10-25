
use crate::modules::system::domain::bo::config::ConfigPageDTO;
use crate::modules::system::domain::{
    Page, PageRequest, SysConfig, SysConfigEntity, SysConfigColumn,
};
use crate::modules::system::domain::vo::SysConfigVO;
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::modules::system::service::CONTEXT;
use sea_orm::entity::*;
use sea_orm::query::*;
use sea_orm::*;
use crate::modules::system::dao::*;
const SYS_CONFIG_KEY: &'static str = "sys_config:";

/// dictionary service
pub struct SysConfigService {}

impl SysConfigService {
    pub async fn page(&self, arg: &ConfigPageDTO) -> Result<Page<SysConfigVO>> {
        let page_req = &PageRequest::from(arg);
        let mut select = SysConfigEntity::find();
        if let Some(name) = &arg.config_name {
            if !name.is_empty() {
                select = select.filter(SysConfigColumn::ConfigName.contains(name.as_str()));
            }
        }
        if let Some(key) = &arg.config_key {
            if !key.is_empty() {
                select = select.filter(SysConfigColumn::ConfigKey.contains(key.as_str()));
            }
        }
        if let Some(ct) = &arg.config_type {
            if !ct.is_empty() {
                select = select.filter(SysConfigColumn::ConfigType.eq(ct.to_string()));
            }
        }
        select = select.order_by_asc(SysConfigColumn::CreateTime);
        let page = paginate(pool!(), select, page_req).await?;
        Ok(page.convert::<SysConfigVO>())
    }


    pub async fn detail(&self, config_id: &str) -> Result<SysConfigVO> {
        let config = SysConfigEntity::find()
            .filter(SysConfigColumn::ConfigId.eq(config_id.parse::<i64>().unwrap_or(0)))
            .one(pool!())
            .await?
            .ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", config_id)))?;
        let config_vo = SysConfigVO::from(config);
        return Ok(config_vo);
    }

    pub async fn add(&self, config: &SysConfig) -> Result<u64> {
        if !self.check_config_key_unique("", config.config_key.as_ref().unwrap()).await? { return Err(Error::from("参数键名重复！")); }

        let result = insert_model(pool!(), config.clone()).await?;
        if result == 1 {
            self.add_to_cache(config).await;
        }
        Ok(result)
    }

    pub async fn update(&self, config: SysConfig) -> Result<u64> {
        let old_config = SysConfigEntity::find()
            .filter(SysConfigColumn::ConfigId.eq(config.config_id.clone()))
            .one(pool!())
            .await?;
        let mut merged = match old_config {
            None => { return Err(Error::from("参数错误")); }
            Some(c) => c,
        };
        // 与原 rbatis update_by_map 跳过 null 的语义一致：仅覆盖 DTO 中提供的字段
        let incoming_key = config.config_key.clone();
        merged.config_name = config.config_name.or(merged.config_name);
        merged.config_key = config.config_key.or(merged.config_key);
        merged.config_value = config.config_value.or(merged.config_value);
        merged.config_type = config.config_type.or(merged.config_type);
        merged.update_by = config.update_by.or(merged.update_by);
        merged.remark = config.remark.or(merged.remark);
        if let Some(new_key) = incoming_key.as_ref() {
            if let Some(old_key) = &merged.config_key {
                if new_key != old_key {
                    CONTEXT.cache_service.del(&self.get_cache_key(old_key)).await;
                }
            }
        }
        if !self.check_config_key_unique(&config.config_id.to_string(), merged.config_key.as_ref().unwrap()).await? { return Err(Error::from("参数键名重复！")); }
        let result = update_model(pool!(), merged.clone()).await?;
        if result == 1 {
            self.add_to_cache(&merged).await;
        }
        Ok(result)
    }

    pub async fn remove(&self, config_id: &str) -> Result<u64> {
        // 「删配置 + 写回收站」必须原子；快照在事务内读取以保证与删除一致。
        // 任一步 `?` 提前返回时 `txn` 被 drop 自动 rollback。
        let txn = pool!().begin().await.map_err(Error::from)?;

        let targets = SysConfigEntity::find()
            .filter(SysConfigColumn::ConfigId.eq(config_id.parse::<i64>().unwrap_or(0)))
            .all(&txn)
            .await?;
        let r = SysConfigEntity::delete_many()
            .filter(SysConfigColumn::ConfigId.eq(config_id.parse::<i64>().unwrap_or(0)))
            .exec(&txn)
            .await?
            .rows_affected;
        if r > 0 {
            //copy data to trash
            CONTEXT
                .sys_trash_service
                .add_in_txn(&txn, "sys_config", &targets)
                .await?;
            // 缓存清理不在事务内（缓存与库严格一致由上层 TTL 兜底），但避免了
            // 原实现 unwrap() 在 config_key 为空时 panic 的隐患。
            if let Some(key) = targets.first().and_then(|t| t.config_key.as_ref()) {
                CONTEXT.cache_service.del(&self.get_cache_key(key)).await;
            }
        }
        txn.commit().await.map_err(Error::from)?;
        Ok(r)
    }


    /**
     * 获取验证码开关，默认不打开
     *
     * @return true开启，false关闭
     */
    pub async fn select_captcha_enabled(&self) -> Result<bool>
    {
        let captcha_enabled = self.select_config_by_key("sys.account.captcha_enabled").await;
        match captcha_enabled {
            Ok(s) => {
                Ok(s.eq("true"))
            }
            Err(_) => { Ok(false) }
        }
    }

    /**
     * 根据键名查询参数配置信息
     *
     */
    pub async fn select_config_by_key(&self, config_key: &str) -> Result<String>
    {
        let config_value = CONTEXT.cache_service.get_string(&self.get_cache_key(config_key)).await?;
        if !config_value.is_empty() {
            return Ok(config_value);
        }
        let config = SysConfigEntity::find()
            .filter(SysConfigColumn::ConfigKey.eq(config_key.to_string()))
            .all(pool!())
            .await?;
        match config.into_iter().next() {
            None => { Ok("".to_string()) }
            Some(c) => {
                CONTEXT.cache_service.set_string(&self.get_cache_key(c.config_key.as_ref().unwrap()), c.config_value.clone().as_ref().unwrap()).await;
                Ok(c.config_value.unwrap())
            }
        }
    }
    /**
     * 校验参数键名是否唯一
     *
     */
    pub async fn check_config_key_unique(&self, config_id: &str, config_key: &str) -> Result<bool>
    {
        let row = pool!()
            .query_one(crate::db::raw_stmt(
                                "select count(1) as count from sys_config where config_id != $1 and config_key = $2",
                vec![Value::from(config_id.to_string()), Value::from(config_key.to_string())],
            ))
            .await?;
        let count: i64 = match row {
            Some(r) => r.try_get::<i64>("", "count").unwrap_or(0),
            None => 0,
        };
        Ok(count < 1)
    }
    /*
     清空参数缓存数据
    */
    pub async fn clear_config_cache(&self) -> Result<()>
    {
        let keys = CONTEXT.cache_service.keys("sys_config:*").await?;
        for k in keys {
            CONTEXT.cache_service.del(&k).await;
        }
        Ok(())
    }

    /**
     * 重置参数缓存数据
     */

    pub async fn reset_config_cache(&self) -> Result<()>
    {
        self.clear_config_cache();
        self.loading_config_cache();
        Ok(())
    }
    //加载所有的到redis
    pub async fn loading_config_cache(&self) -> Result<u64> {
        let config_list: Vec<SysConfig> = SysConfigEntity::find().all(pool!()).await?;
        for config in config_list {
            self.add_to_cache(&config).await;
        }
        Ok(1)
    }
    async fn add_to_cache(&self, config: &SysConfig) {
        CONTEXT.cache_service.set_string(&self.get_cache_key(config.config_key.as_ref().unwrap()), config.config_value.as_ref().unwrap()).await;
    }
    //对config_key进行处理
    fn get_cache_key(&self, origin: &str) -> String {
        format!("{}{}", SYS_CONFIG_KEY, origin)
    }
}

impl SysConfigService {
    /// 根据参数键名修改参数值（对应Spring PUT /system/config/updateByKey）
    pub async fn update_by_key(&self, config_key: &str, config_value: &str) -> Result<u64> {
        let r = SysConfigEntity::update_many()
            .col_expr(SysConfigColumn::ConfigValue, sea_orm::sea_query::Expr::value(config_value))
            .filter(SysConfigColumn::ConfigKey.eq(config_key.to_string()))
            .exec(pool!())
            .await?;
        if r.rows_affected > 0 {
            CONTEXT.cache_service.set_string(&self.get_cache_key(config_key), config_value).await;
        }
        Ok(r.rows_affected)
    }
}
