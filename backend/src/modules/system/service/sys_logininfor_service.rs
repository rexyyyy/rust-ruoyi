
use crate::modules::system::domain::bo::logininfor::LogininforPageDTO;
use crate::modules::system::domain::{
    Page, PageRequest, SysLogininfor, SysLogininforEntity, SysLogininforColumn,
};
use crate::modules::system::domain::vo::SysLogininforVO;
use crate::error::Result;
use crate::pool;
use sea_orm::entity::*;
use sea_orm::query::*;
use sea_orm::*;
use crate::modules::system::dao::*;

/// dictionary service
pub struct SysLogininforService {}

impl SysLogininforService {
    pub async fn page(&self, arg: &LogininforPageDTO) -> Result<Page<SysLogininforVO>> {
        let page_req = &PageRequest::from(arg);
        let select = SysLogininforEntity::find()
            .order_by_desc(SysLogininforColumn::LoginTime);
        let page = paginate(pool!(), select, page_req).await?;
        Ok(page.convert::<SysLogininforVO>())
    }

    //异步加入日志
    pub async fn add_async(&self, arg: &SysLogininfor) -> Result<u64> {
        let info = arg.to_owned();
        tokio::spawn(async move {
            let _ = insert_model(pool!(), info).await;
        });
        return Ok(1);
    }


    pub async fn remove(&self, info_id: &str) -> Result<u64> {
        // info_id 在库中为 bigint，按字符串比较会导致 PostgreSQL 类型不匹配而删不掉
        let r = SysLogininforEntity::delete_many()
            .filter(SysLogininforColumn::InfoId.eq(info_id.parse::<i64>().unwrap_or(0)))
            .exec(pool!())
            .await?
            .rows_affected;
        Ok(r)
    }
}

impl SysLogininforService {
    /// 清空登录日志（对应Spring DELETE /monitor/logininfor/clean）
    pub async fn clean(&self) -> Result<u64> {
        let r = pool!()
            .execute(crate::db::raw_stmt0(
                                "delete from sys_logininfor",
            ))
            .await?;
        Ok(r.rows_affected())
    }
}
