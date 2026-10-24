
use crate::modules::system::domain::bo::post::PostPageDTO;
use crate::modules::system::domain::{
    Page, PageRequest, SysPost, SysPostEntity, SysPostColumn,
};
use crate::modules::system::domain::vo::SysPostVO;
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::modules::system::service::CONTEXT;
use sea_orm::entity::*;
use sea_orm::query::*;
use crate::modules::system::dao::*;
//const DICT_KEY: &'static str = "sys_post:all";

/// dictionary service
pub struct SysPostService {}

impl SysPostService {
    pub async fn page(&self, arg: &PostPageDTO) -> Result<Page<SysPostVO>> {
        let page_req = &PageRequest::from(arg);
        let mut select = SysPostEntity::find();
        if let Some(name) = &arg.post_name {
            if !name.is_empty() {
                select = select.filter(SysPostColumn::PostName.contains(name.as_str()));
            }
        }
        if let Some(code) = &arg.post_code {
            if !code.is_empty() {
                select = select.filter(SysPostColumn::PostCode.contains(code.as_str()));
            }
        }
        if let Some(st) = &arg.status {
            if !st.is_empty() {
                select = select.filter(SysPostColumn::Status.eq(st.to_string()));
            }
        }
        select = select.order_by_asc(SysPostColumn::PostSort);
        let page = paginate(pool!(), select, page_req).await?;
        Ok(page.convert::<SysPostVO>())
    }

    pub async fn finds_all(&self) -> Result<Vec<SysPostVO>> {
        let data = SysPostEntity::find().all(pool!()).await?;
        let mut post_vos = vec![];
        for s in data {
            post_vos.push(SysPostVO::from(s));
        }
        Ok(post_vos)
    }
    pub async fn detail(&self, post_id: &str) -> Result<SysPostVO> {
        // post_id 在库中为 bigint，按字符串比较会导致 PostgreSQL 类型不匹配而查不到数据
        let post = SysPostEntity::find()
            .filter(SysPostColumn::PostId.eq(post_id.parse::<i64>().unwrap_or(0)))
            .one(pool!())
            .await?
            .ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", post_id)))?;
        let post_vo = SysPostVO::from(post);
        return Ok(post_vo);
    }

    pub async fn add(&self, arg: &SysPost) -> Result<u64> {
        let result = insert_model(pool!(), arg.clone()).await;
        return result;
    }

    pub async fn update(&self, data: SysPost) -> Result<u64> {
        let old = SysPostEntity::find()
            .filter(SysPostColumn::PostId.eq(data.post_id.clone()))
            .one(pool!())
            .await?;
        let mut merged = match old {
            None => { return Err(Error::from("参数错误")); }
            Some(c) => c,
        };
        // 与原 rbatis update_by_map 跳过 null 的语义一致：仅覆盖 DTO 中提供的字段
        merged.post_code = data.post_code.or(merged.post_code);
        merged.post_name = data.post_name.or(merged.post_name);
        merged.post_sort = data.post_sort.or(merged.post_sort);
        merged.status = data.status.or(merged.status);
        merged.update_by = data.update_by.or(merged.update_by);
        merged.update_time = data.update_time.or(merged.update_time);
        merged.remark = data.remark.or(merged.remark);
        let result = update_model(pool!(), merged).await;
        return Ok(result?);
    }

    pub async fn remove(&self, post_id: &str) -> Result<u64> {
        let post_id_i64 = post_id.parse::<i64>().unwrap_or(0);
        let targets = SysPostEntity::find()
            .filter(SysPostColumn::PostId.eq(post_id_i64))
            .all(pool!())
            .await?;

        let r = SysPostEntity::delete_many()
            .filter(SysPostColumn::PostId.eq(post_id_i64))
            .exec(pool!())
            .await?
            .rows_affected;
        if r > 0 {
            //copy data to trash
            CONTEXT.sys_trash_service.add("sys_post", &targets).await?;
        }
        Ok(r)
    }
}
