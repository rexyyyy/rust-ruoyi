
use crate::modules::system::domain::bo::logininfor::LogininforPageDTO;
use crate::modules::system::domain::{
    Page, PageRequest, SysLogininfor, SysLogininforEntity, SysLogininforColumn,
};
use crate::modules::system::domain::vo::SysLogininforVO;
use crate::error::Result;
use crate::pool;
use sea_orm::entity::*;
use sea_orm::query::*;
use crate::modules::system::dao::*;

/// dictionary service
pub struct SysLogininforService {}

impl SysLogininforService {
    pub async fn page(&self, arg: &LogininforPageDTO) -> Result<Page<SysLogininforVO>> {
        let page_req = &PageRequest::from(arg);
        // 原实现完全忽略 DTO 过滤条件（前端搜索框无效），此处补齐地址/用户名/状态/时间范围
        let mut select = SysLogininforEntity::find();
        if let Some(ip) = arg.ipaddr.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            select = select.filter(SysLogininforColumn::Ipaddr.contains(ip));
        }
        if let Some(un) = arg.user_name.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            select = select.filter(SysLogininforColumn::UserName.contains(un));
        }
        if let Some(st) = arg.status.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            select = select.filter(SysLogininforColumn::Status.eq(st));
        }
        let (begin, end) = range_bounds(&arg.params);
        if let Some(b) = begin {
            select = select.filter(SysLogininforColumn::LoginTime.gte(b));
        }
        if let Some(e) = end {
            select = select.filter(SysLogininforColumn::LoginTime.lte(e));
        }
        let select = select.order_by_desc(SysLogininforColumn::LoginTime);
        let page = paginate(pool!(), select, page_req).await?;
        Ok(page.convert::<SysLogininforVO>())
    }

    // 异步写入登录日志：投递到有界队列，由后台协程批量落库（不占请求线程/连接）
    pub async fn add_async(&self, arg: &SysLogininfor) -> Result<u64> {
        crate::util::log_sink::push_logininfor(arg.clone());
        Ok(1)
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
