
use crate::modules::system::domain::bo::dict_type::{DictTypePageDTO};
use crate::modules::system::domain::{
    Page, PageRequest, SysDictType, SysDictTypeEntity, SysDictTypeColumn,
};
use crate::modules::system::domain::vo::SysDictTypeVO;
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::modules::system::service::CONTEXT;
use sea_orm::entity::*;
use sea_orm::query::*;
use sea_orm::*;
use crate::modules::system::dao::*;
//const DICT_KEY: &'static str = "sys_dict_type:all";

/// dictionary service
pub struct SysDictTypeService {}

impl SysDictTypeService {
    pub async fn page(&self, arg: &DictTypePageDTO) -> Result<Page<SysDictTypeVO>> {
        let page_req = &PageRequest::from(arg);
        let mut select = SysDictTypeEntity::find();
        if let Some(dt) = &arg.dict_type {
            if !dt.is_empty() {
                select = select.filter(SysDictTypeColumn::DictType.contains(dt.as_str()));
            }
        }
        if let Some(name) = &arg.dict_name {
            if !name.is_empty() {
                select = select.filter(SysDictTypeColumn::DictName.contains(name.as_str()));
            }
        }
        select = select.order_by_asc(SysDictTypeColumn::CreateTime);
        let page = paginate(pool!(), select, page_req).await?;
        Ok(page.convert::<SysDictTypeVO>())
    }
    pub async fn finds_all(&self) -> Result<Vec<SysDictTypeVO>> {
        let data = SysDictTypeEntity::find().all(pool!()).await?;
        let mut dict_type_vos = vec![];
        for d in data {
            dict_type_vos.push(SysDictTypeVO::from(d));
        }
        Ok(dict_type_vos)
    }
    pub async fn detail(&self, dict_id: &str) -> Result<SysDictTypeVO> {
        let dict_type = SysDictTypeEntity::find()
            .filter(SysDictTypeColumn::DictId.eq(dict_id.parse::<i64>().unwrap_or(0)))
            .one(pool!())
            .await?
            .ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", dict_id)))?;
        let dict_type_vo = SysDictTypeVO::from(dict_type);
        return Ok(dict_type_vo);
    }
    pub async fn add(&self, arg: &SysDictType) -> Result<u64> {
        let result = insert_model(pool!(), arg.clone()).await;
        CONTEXT.sys_dict_data_service.update_cache().await;
        return result;
    }

    pub async fn update(&self, data: SysDictType) -> Result<u64> {
        let old = SysDictTypeEntity::find()
            .filter(SysDictTypeColumn::DictId.eq(data.dict_id.clone()))
            .one(pool!())
            .await?;
        let mut merged = match old {
            None => { return Err(Error::from("参数错误")); }
            Some(c) => c,
        };
        // 与原 rbatis update_by_map 跳过 null 的语义一致：仅覆盖 DTO 中提供的字段
        merged.dict_name = data.dict_name.or(merged.dict_name);
        merged.dict_type = data.dict_type.or(merged.dict_type);
        merged.update_by = data.update_by.or(merged.update_by);
        merged.update_time = data.update_time.or(merged.update_time);
        merged.remark = data.remark.or(merged.remark);
        let result = update_model(pool!(), merged).await;
        if result.is_ok() {
            //更新dict_data
            CONTEXT.sys_dict_data_service.update_cache().await?;
        }
        return Ok(result?);
    }

    pub async fn remove(&self, dict_id: &str) -> Result<u64> {
        let targets = SysDictTypeEntity::find()
            .filter(SysDictTypeColumn::DictId.eq(dict_id.parse::<i64>().unwrap_or(0)))
            .all(pool!())
            .await?;

        if targets.len() == 1 {
            let dict_type = targets.get(0).unwrap().dict_type.clone().unwrap(); 
            // 存在子项指「字典数据」，不是字典类型表自身（否则统计到当前行，永远删不掉）
            let row = pool!()
                .query_one(crate::db::raw_stmt(
                                        "select count(1) as count from sys_dict_data where dict_type = $1",
                    vec![Value::from(dict_type)],
                ))
                .await?;
            let count: i64 = match row {
                Some(r) => r.try_get::<i64>("", "count").unwrap_or(0),
                None => 0,
            };
            if count > 0 { return Err(Error::from("存在子项,不允许删除！")); }
        } else { return Err(Error::from(format!("字典id{}不存在！", dict_id))); }

        let r = SysDictTypeEntity::delete_many()
            .filter(SysDictTypeColumn::DictId.eq(dict_id.parse::<i64>().unwrap_or(0)))
            .exec(pool!())
            .await?
            .rows_affected;
        if r > 0 {
            CONTEXT.sys_dict_data_service.update_cache().await?;
            //copy data to trash
            CONTEXT.sys_trash_service.add("sys_dict_type", &targets).await?;
        }
        Ok(r)
    }
}
