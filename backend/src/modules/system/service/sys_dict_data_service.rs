use std::collections::HashMap;

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
//const DICT_KEY: &'static str = "sys_dict_data:all";

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

    pub async fn get_by_dict_type(&self, dict_type: &String) -> Result<Vec<SysDictDataSimpleVO>> {
        let data = SysDictDataEntity::find()
            .filter(SysDictDataColumn::DictType.eq(dict_type.clone()))
            .order_by_asc(SysDictDataColumn::DictSort)
            .all(pool!())
            .await?;
        let mut res = vec![];
        for d in data {
            res.push(SysDictDataSimpleVO::from(d))
        }
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
        self.update_cache().await?;
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
            self.update_cache().await?;
        }
        return Ok(result?);
    }

    pub async fn remove(&self, dict_code: &str) -> Result<u64> {
        let targets = SysDictDataEntity::find()
            .filter(SysDictDataColumn::DictCode.eq(dict_code.parse::<i64>().unwrap_or(0)))
            .all(pool!())
            .await?;

        let r = SysDictDataEntity::delete_many()
            .filter(SysDictDataColumn::DictCode.eq(dict_code.parse::<i64>().unwrap_or(0)))
            .exec(pool!())
            .await?
            .rows_affected;
        if r > 0 {
            self.update_cache().await?;
            //copy data to trash
            CONTEXT.sys_trash_service.add("sys_dict_data", &targets).await?;
        }
        Ok(r)
    }

    /// update for all cache
    pub async fn update_cache(&self) -> Result<()> {
        let ck = |m: &str| crate::ckpt(&format!("[dict] {}", m));
        ck("a: before find().all()");
        let mut all: Vec<SysDictData> = SysDictDataEntity::find().all(pool!()).await?;
        ck("b: after find().all()");
        all.sort_by(|a, b| a.dict_sort.cmp(&b.dict_sort));
        ck("c: after sort_by");
        let mut dict_data_map: HashMap<&str, Vec<SysDictDataSimpleVO>> = HashMap::new();
        for dict_data in all {
            // 脏数据防护：dict_type 为 NULL 时不能 unwrap，否则启动期加载字典缓存
            // 直接 panic，表现为整个服务起不来（一条坏数据拖垮全站）。
            // 注意：下方 key 仍被硬编码覆盖（原 fixme），缓存写入也已注释禁用，
            // 故此处仅保证不崩溃，不改变现有行为。
            let _key = dict_data.dict_type.clone().as_deref().unwrap_or_default();
            let key = "1233";//fixme
            let data_sim = SysDictDataSimpleVO::from(dict_data);
            if dict_data_map.contains_key(key) {
                dict_data_map.get_mut(key).unwrap().push(data_sim);
            } else {
                dict_data_map.insert(key, vec![data_sim]);
            }
        }
        ck("d: after loop");
        //  CONTEXT.cache_service.set_json(DICT_KEY, &dict_data_map).await?;
        ck("e: return ok");
        return Ok(());
    }
}
