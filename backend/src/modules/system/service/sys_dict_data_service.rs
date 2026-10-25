use std::collections::HashMap;
use std::time::Duration;

use crate::modules::system::domain::bo::dict_data::{DictDataPageDTO};
use crate::modules::system::domain::{
    Page, PageRequest, SysDictData, SysDictDataEntity, SysDictDataColumn,
};
use crate::modules::system::domain::vo::{SysDictDataSimpleVO, SysDictDataVO};
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::modules::system::service::CONTEXT;
use sea_orm::entity::*;
use sea_orm::query::*;
use crate::modules::system::dao::*;

/// 按 dict_type 分组的字典数据缓存 key 前缀（与 RuoYi `sys_dict:{type}` 语义对齐）。
const DICT_DATA_KEY_PREFIX: &'static str = "sys_dict_data:";
/// 兜底 TTL：即使写操作漏了失效，缓存最多 30 分钟后也会自动重建。
const DICT_DATA_TTL_SECS: u64 = 1800;

/// dictionary service
pub struct SysDictDataService {}

impl SysDictDataService {
    pub async fn page(&self, arg: &DictDataPageDTO) -> Result<Page<SysDictDataVO>> {
        let page_req = &PageRequest::from(arg);
        let mut select = SysDictDataEntity::find();
        if let Some(dt) = &arg.dict_type {
            if !dt.is_empty() {
                select = select.filter(SysDictDataColumn::DictType.eq(dt.clone()));
            }
        }
        if let Some(label) = &arg.dict_label {
            if !label.is_empty() {
                select = select.filter(SysDictDataColumn::DictLabel.contains(label.as_str()));
            }
        }
        select = select.order_by_asc(SysDictDataColumn::DictSort);
        let page = paginate(pool!(), select, page_req).await?;
        Ok(page.convert::<SysDictDataVO>())
    }

    /// 按字典类型取下拉项（前端每个使用字典的页面都会调，热路径）。
    ///
    /// 命中 `sys_dict_data:{type}` 直接返回；未命中则回源一次并写缓存。
    pub async fn get_by_dict_type(&self, dict_type: &String) -> Result<Vec<SysDictDataSimpleVO>> {
        let key = Self::cache_key(dict_type);
        if let Ok(Some(cached)) = CONTEXT
            .cache_service
            .get_json::<Option<Vec<SysDictDataSimpleVO>>>(&key)
            .await
        {
            return Ok(cached);
        }
        let data = SysDictDataEntity::find()
            .filter(SysDictDataColumn::DictType.eq(dict_type.clone()))
            .order_by_asc(SysDictDataColumn::DictSort)
            .all(pool!())
            .await?;
        let res: Vec<SysDictDataSimpleVO> = data.into_iter().map(SysDictDataSimpleVO::from).collect();
        // 缓存写入失败不影响本次返回
        let _ = CONTEXT
            .cache_service
            .set_json_ex(&key, &res, Some(Duration::from_secs(DICT_DATA_TTL_SECS)))
            .await;
        Ok(res)
    }

    pub async fn detail(&self, dict_code: &str) -> Result<SysDictDataVO> {
        let dict_data = SysDictDataEntity::find()
            .filter(SysDictDataColumn::DictCode.eq(dict_code.parse::<i64>().unwrap_or(0)))
            .one(pool!())
            .await?
            .ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", dict_code)))?;
        let dict_data_vo = SysDictDataVO::from(dict_data);
        return Ok(dict_data_vo);
    }

    pub async fn add(&self, arg: &SysDictData) -> Result<u64> {
        let result = insert_model(pool!(), arg.clone()).await;
        // 写操作只做「失效」，不整表重载：真正回源交给下一次 get_by_dict_type 惰性完成
        self.clear_cache().await?;
        return result;
    }

    pub async fn update(&self, data: SysDictData) -> Result<u64> {
        let old = SysDictDataEntity::find()
            .filter(SysDictDataColumn::DictCode.eq(data.dict_code.clone()))
            .one(pool!())
            .await?;
        let mut merged = match old {
            None => { return Err(Error::from("参数错误")); }
            Some(c) => c,
        };
        // 与原 rbatis update_by_map 跳过 null 的语义一致：仅覆盖 DTO 中提供的字段
        merged.dict_sort = data.dict_sort.or(merged.dict_sort);
        merged.dict_label = data.dict_label.or(merged.dict_label);
        merged.dict_value = data.dict_value.or(merged.dict_value);
        merged.dict_type = data.dict_type.or(merged.dict_type);
        merged.css_class = data.css_class.or(merged.css_class);
        merged.list_class = data.list_class.or(merged.list_class);
        merged.is_default = data.is_default.or(merged.is_default);
        merged.update_by = data.update_by.or(merged.update_by);
        merged.update_time = data.update_time.or(merged.update_time);
        merged.remark = data.remark.or(merged.remark);
        let result = update_model(pool!(), merged).await;
        if result.is_ok() {
            self.clear_cache().await?;
        }
        return Ok(result?);
    }

    pub async fn remove(&self, dict_code: &str) -> Result<u64> {
        use sea_orm::TransactionTrait;
        // 「删字典数据 + 写回收站」必须原子；快照在事务内读取以保证与删除一致。
        let txn = pool!().begin().await.map_err(Error::from)?;

        let targets = SysDictDataEntity::find()
            .filter(SysDictDataColumn::DictCode.eq(dict_code.parse::<i64>().unwrap_or(0)))
            .all(&txn)
            .await?;

        let r = SysDictDataEntity::delete_many()
            .filter(SysDictDataColumn::DictCode.eq(dict_code.parse::<i64>().unwrap_or(0)))
            .exec(&txn)
            .await?
            .rows_affected;
        if r > 0 {
            //copy data to trash（与删除同事务：清理/快照失败则整笔回滚）
            CONTEXT
                .sys_trash_service
                .add_in_txn(&txn, "sys_dict_data", &targets)
                .await?;
        }

        txn.commit().await.map_err(Error::from)?;
        // 缓存在提交成功后清理：提交前清理会让其它请求在回滚期间读到脏缓存
        if r > 0 {
            self.clear_cache().await?;
        }
        Ok(r)
    }

    /// 字典类型 -> 缓存 key
    fn cache_key(dict_type: &str) -> String {
        format!("{}{}", DICT_DATA_KEY_PREFIX, dict_type)
    }

    /// 清空全部字典数据缓存（写操作后调用，惰性重建）。
    pub async fn clear_cache(&self) -> Result<()> {
        let keys = CONTEXT
            .cache_service
            .keys(&format!("{}*", DICT_DATA_KEY_PREFIX))
            .await?;
        for k in keys {
            let _ = CONTEXT.cache_service.del(&k).await;
        }
        Ok(())
    }

    /// 全量预热：一次 SELECT 按 dict_type 分组，写入各自缓存 key。
    ///
    /// 启动时调用（或前端点「刷新缓存」时调用）。
    /// 历史实现把 key 硬编码成 `"1233"` 且写缓存语句被注释，等于每次增删改都白跑
    /// 一遍全表扫描 + 排序，本次一并修正。
    pub async fn update_cache(&self) -> Result<()> {
        let all: Vec<SysDictData> = SysDictDataEntity::find()
            .order_by_asc(SysDictDataColumn::DictSort)
            .all(pool!())
            .await?;
        let mut grouped: HashMap<String, Vec<SysDictDataSimpleVO>> = HashMap::new();
        for dict_data in all {
            // 脏数据防护：dict_type 为 NULL 时跳过（否则会写出一个空前缀 key）
            let dict_type = match dict_data.dict_type.clone() {
                Some(dt) if !dt.is_empty() => dt,
                _ => continue,
            };
            grouped
                .entry(dict_type)
                .or_default()
                .push(SysDictDataSimpleVO::from(dict_data));
        }
        // 先清掉旧值，避免字典类型被删除后仍残留 key
        self.clear_cache().await?;
        let ttl = Some(Duration::from_secs(DICT_DATA_TTL_SECS));
        for (dict_type, list) in grouped {
            let _ = CONTEXT
                .cache_service
                .set_json_ex(&Self::cache_key(&dict_type), &list, ttl)
                .await;
        }
        Ok(())
    }
}
