
use crate::modules::system::domain::bo::operlog::OperLogPageDTO;
use crate::modules::system::domain::{
    Page, PageRequest, SysOperLog, SysOperLogEntity, SysOperLogColumn,
};
use crate::modules::system::domain::vo::SysOperLogVO;
use crate::error::Result;
use crate::pool;
use sea_orm::entity::*;
use sea_orm::query::*;
use crate::modules::system::dao::*;

/// 操作日志服务（对应 Spring SysOperLogServiceImpl）
pub struct SysOperlogService {}

impl SysOperlogService {
    pub async fn page(&self, arg: &OperLogPageDTO) -> Result<Page<SysOperLogVO>> {
        let page_req = &PageRequest::from(arg);
        // 原实现完全忽略 DTO 过滤条件（前端搜索框无效），此处补齐模块/操作人/类型/状态/时间范围
        let mut select = SysOperLogEntity::find();
        if let Some(t) = arg.title.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            select = select.filter(SysOperLogColumn::Title.contains(t));
        }
        if let Some(op) = arg.oper_name.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            select = select.filter(SysOperLogColumn::OperName.contains(op));
        }
        if let Some(bt) = arg.business_type {
            select = select.filter(SysOperLogColumn::BusinessType.eq(bt));
        }
        if let Some(st) = arg.status {
            select = select.filter(SysOperLogColumn::Status.eq(st));
        }
        let (begin, end) = range_bounds(&arg.params);
        if let Some(b) = begin {
            select = select.filter(SysOperLogColumn::OperTime.gte(b));
        }
        if let Some(e) = end {
            select = select.filter(SysOperLogColumn::OperTime.lte(e));
        }
        let select = select.order_by_desc(SysOperLogColumn::OperTime);
        let page = paginate(pool!(), select, page_req).await?;
        Ok(page.convert::<SysOperLogVO>())
    }

    pub async fn remove(&self, oper_id: &str) -> Result<u64> {
        // oper_id 在库中为 bigint，按字符串比较会导致 PostgreSQL 类型不匹配而删不掉
        let r = SysOperLogEntity::delete_many()
            .filter(SysOperLogColumn::OperId.eq(oper_id.parse::<i64>().unwrap_or(0)))
            .exec(pool!())
            .await?
            .rows_affected;
        Ok(r)
    }

    // 清空操作日志（对应Spring DELETE /monitor/operlog/clean）
    pub async fn clean(&self) -> Result<u64> {
        let r = pool!()
            .execute(crate::db::raw_stmt0(
                                "delete from sys_oper_log",
            ))
            .await?;
        Ok(r.rows_affected())
    }

    // 异步写入操作日志：投递到有界队列，由后台协程批量落库（不占请求线程/连接）
    pub async fn add_async(&self, arg: &SysOperLog) -> Result<u64> {
        crate::util::log_sink::push_operlog(arg.clone());
        Ok(1)
    }

    // 便捷记录：业务类型(0其它 1新增 2修改 3删除 5登录 6退出 7授权)
    pub async fn record(&self, title: &str, business_type: i32, method: &str, oper_name: &str, oper_url: &str, status: i32, error_msg: &str) {
        let log = SysOperLog {
            oper_id: crate::util::idseq::next_id() as i64,
            title: Some(title.to_string()),
            business_type: Some(business_type),
            method: Some(method.to_string()),
            request_method: Some(method.split_whitespace().next().unwrap_or("POST").to_string()),
            operator_type: Some(1),
            oper_name: Some(oper_name.to_string()),
            dept_name: None,
            oper_url: Some(oper_url.to_string()),
            oper_ip: None,
            oper_location: None,
            oper_param: None,
            json_result: None,
            status: Some(status),
            error_msg: Some(error_msg.to_string()).filter(|s| !s.is_empty()),
            oper_time: Some(chrono::Local::now().naive_local()),
        };
        let _ = self.add_async(&log).await;
    }
}
