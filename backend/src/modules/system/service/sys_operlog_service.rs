
use crate::modules::system::domain::bo::operlog::OperLogPageDTO;
use crate::modules::system::domain::{
    Page, PageRequest, SysOperLog, SysOperLogEntity, SysOperLogColumn,
};
use crate::modules::system::domain::vo::SysOperLogVO;
use crate::error::Result;
use crate::pool;
use sea_orm::entity::*;
use sea_orm::query::*;
use sea_orm::*;
use crate::modules::system::dao::*;

/// 操作日志服务（对应 Spring SysOperLogServiceImpl）
pub struct SysOperlogService {}

impl SysOperlogService {
    pub async fn page(&self, arg: &OperLogPageDTO) -> Result<Page<SysOperLogVO>> {
        let page_req = &PageRequest::from(arg);
        let select = SysOperLogEntity::find()
            .order_by_desc(SysOperLogColumn::OperTime);
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

    // 异步写入操作日志
    pub async fn add_async(&self, arg: &SysOperLog) -> Result<u64> {
        let info = arg.to_owned();
        tokio::spawn(async move {
            let _ = insert_model(pool!(), info).await;
        });
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
