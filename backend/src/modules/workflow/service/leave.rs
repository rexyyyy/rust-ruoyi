//! 请假单示例业务（`test_leave`）。
//!
//! 对标 Java `TestLeaveServiceImpl`：业务表本身只管「单据」，审批过程完全交给
//! 流程引擎。两者的连接点是：
//!
//! ```text
//! 请假单 id  ──(business_id)──▶  流程实例
//! 请假单 status ◀──(biz::sync_business_status)── 实例 flow_status
//! ```
//!
//! 因此 `submit` 的顺序很关键：**先落业务表拿到 id，再发起流程**，
//! 否则流程实例的 `business_id` 无处指向。
//!
//! 同一张业务表可以挂多个流程（leave1 线性审批、leave2 带排他网关……），
//! 发起时由入参 `flowCode` 指定，不传则走 [`FLOW_CODE`]。

use std::collections::HashMap;

use sea_orm::ConnectionTrait;
use serde_json::json;
use sea_orm::entity::prelude::*;
use sea_orm::query::*;
use sea_orm::{ActiveModelTrait, PaginatorTrait, Set};

use crate::error::{Error, Result};
use crate::modules::workflow::domain::bo::{
    CompleteTaskBo, LeaveBo, LeavePageBo, StartRrocessBo,
};
use crate::modules::workflow::domain::vo::{LeaveVO, StartReturnVO};
use crate::modules::workflow::domain::{
    Page, TestLeave, TestLeaveActiveModel, TestLeaveColumn, TestLeaveEntity,
};
use crate::modules::workflow::service::{self, biz, task};

/// 默认请假流程编码；`LeaveBo.flowCode` 为空时使用，
/// 与 `demo` 导入的 leave1.json / 设计器里保存的 flowCode 一致
pub const FLOW_CODE: &str = "leave1";

/// 请假单分页
pub async fn page(bo: &LeavePageBo) -> Result<Page<LeaveVO>> {
    let page_no = bo.page_no.unwrap_or(1).max(1);
    let page_size = bo.page_size.unwrap_or(10).clamp(1, 200);
    let mut select = TestLeaveEntity::find();
    if let Some(leave_type) = service::blank_to_none(bo.leave_type.as_deref()) {
        select = select.filter(TestLeaveColumn::LeaveType.eq(leave_type));
    }
    if let Some(status) = service::blank_to_none(bo.status.as_deref()) {
        select = select.filter(TestLeaveColumn::Status.eq(status));
    }
    select = select.order_by_desc(TestLeaveColumn::Id);

    let paginator = select.paginate(crate::pool!(), page_size);
    let total = paginator.num_items().await?;
    let rows = paginator.fetch_page(page_no - 1).await?;
    let mut vos: Vec<LeaveVO> = rows.into_iter().map(LeaveVO::from).collect();
    let ids: Vec<String> = vos.iter().map(|v| v.id.clone()).collect();
    let codes = flow_codes_by_business(&ids).await;
    for vo in vos.iter_mut() {
        vo.flow_code = codes.get(&vo.id).cloned();
    }
    Ok(Page::new(vos, total, page_no, page_size))
}

/// 反查请假单实际走的流程编码（business_id → flow_code）
async fn flow_codes_by_business(business_ids: &[String]) -> HashMap<String, String> {
    if business_ids.is_empty() {
        return HashMap::new();
    }
    let Some(db) = crate::db::db() else {
        return HashMap::new();
    };
    let holders = (1..=business_ids.len()).map(|i| format!("${i}")).collect::<Vec<_>>().join(",");
    let sql = format!(
        "SELECT i.business_id AS business_id, d.flow_code AS flow_code \
         FROM flow_instance i JOIN flow_definition d ON d.id = i.definition_id \
         WHERE i.business_id IN ({holders})"
    );
    let vals = business_ids.iter().cloned().map(sea_orm::Value::from).collect();
    let rows = match db.query_all(crate::db::raw_stmt(sql, vals)).await {
        Ok(rows) => rows,
        Err(e) => {
            log::warn!("[workflow] 反查请假单流程编码失败: {e}");
            return HashMap::new();
        }
    };
    let mut map = HashMap::new();
    for row in rows {
        let bid = row.try_get::<String>("", "business_id");
        let code = row.try_get::<String>("", "flow_code");
        if let (Ok(bid), Ok(code)) = (bid, code) {
            map.insert(bid, code);
        }
    }
    map
}

/// 详情
pub async fn detail(id: i64) -> Result<LeaveVO> {
    let model = TestLeaveEntity::find()
        .filter(TestLeaveColumn::Id.eq(id))
        .one(crate::pool!())
        .await?
        .ok_or_else(|| Error::from(format!("请假单 {id} 不存在")))?;
    let mut vo = LeaveVO::from(model);
    let codes = flow_codes_by_business(&[vo.id.clone()]).await;
    vo.flow_code = codes.get(&vo.id).cloned();
    Ok(vo)
}

/// 新增草稿
pub async fn add(bo: &LeaveBo, uid: i64) -> Result<i64> {
    let (start, end, days) = normalize(bo)?;
    let id = crate::util::idseq::next_id() as i64;
    let model = TestLeave {
        id,
        apply_code: None,
        leave_type: leave_type_of(bo),
        start_date: start,
        end_date: end,
        leave_days: days,
        remark: service::blank_to_none(bo.remark.as_deref()),
        status: Some(biz::status::DRAFT.to_string()),
        create_by: Some(uid),
        create_time: Some(chrono::Local::now().naive_local()),
        update_by: None,
        update_time: None,
    };
    TestLeaveEntity::insert(TestLeaveActiveModel::from(model))
        .exec(crate::pool!())
        .await?;
    Ok(id)
}

/// 修改（只允许改草稿内容，状态由流程回写）
pub async fn update(bo: &LeaveBo, uid: i64) -> Result<u64> {
    let id = bo.id.ok_or_else(|| Error::from("请假单 id 不能为空"))?;
    let old = TestLeaveEntity::find()
        .filter(TestLeaveColumn::Id.eq(id))
        .one(crate::pool!())
        .await?
        .ok_or_else(|| Error::from(format!("请假单 {id} 不存在")))?;

    let mut active = TestLeaveActiveModel {
        id: Set(old.id),
        ..Default::default()
    };
    if bo.leave_type.is_some() {
        active.leave_type = Set(leave_type_of(bo));
    }
    if bo.start_date.is_some() || bo.end_date.is_some() {
        let (start, end, days) = normalize(bo)?;
        active.start_date = Set(start);
        active.end_date = Set(end);
        active.leave_days = Set(days);
    } else if let Some(days) = bo.leave_days {
        active.leave_days = Set(days);
    }
    if bo.remark.is_some() {
        active.remark = Set(service::blank_to_none(bo.remark.as_deref()));
    }
    active.update_by = Set(Some(uid));
    active.update_time = Set(Some(chrono::Local::now().naive_local()));
    active.update(crate::pool!()).await?;
    Ok(1)
}

/// 删除
pub async fn remove(ids: &[i64]) -> Result<u64> {
    if ids.is_empty() {
        return Err(Error::from("请选择要删除的请假单"));
    }
    let r = TestLeaveEntity::delete_many()
        .filter(TestLeaveColumn::Id.is_in(ids.to_vec()))
        .exec(crate::pool!())
        .await?;
    Ok(r.rows_affected)
}

/// 保存并提交审批：落业务表 → 发起流程 → 自动办结「申请人」→ 回填申请编号。
///
/// 对应 Java `TestLeaveServiceImpl.submit` + `FlwTaskServiceImpl.startWorkFlow`
/// 两步合体（Java 侧由 Controller 分两次调用）。
pub async fn submit(bo: &LeaveBo, uid: i64) -> Result<StartReturnVO> {
    let leave_id = match bo.id {
        Some(id) => {
            update(bo, uid).await?;
            id
        }
        None => add(bo, uid).await?,
    };
    let business_id = leave_id.to_string();
    let apply_code = apply_code(leave_id);
    let leave = detail(leave_id).await?;

    let mut variables = serde_json::Map::new();
    // businessCode 会随实例保存，流程图与待办详情里都能看到申请编号
    variables.insert("businessCode".to_string(), json!(apply_code.clone()));
    variables.insert("leaveType".to_string(), json!(leave.leave_type));
    variables.insert("leaveDays".to_string(), json!(leave.leave_days));

    // 流程编码由调用方指定（设计器新建的请假流程也能直接跑），不传则用默认 leave1
    let flow_code = service::blank_to_none(bo.flow_code.as_deref())
        .unwrap_or_else(|| FLOW_CODE.to_string());
    let mut ret = task::start(
        uid,
        &StartRrocessBo {
            flow_code: Some(flow_code),
            business_id: Some(business_id.clone()),
            variables,
        },
    )
    .await?;

    // 编号只回填到业务表：流程实例内已通过变量 `businessCode` 携带
    biz::fill_business_code(&business_id, &apply_code).await;

    // 「申请人」节点只是提交动作在流程里的留痕，不该再要发起人去待办点一次同意：
    // 发起成功后立即由本人办结，流程直接推进到真正的审批节点。
    // 办结失败不影响提交结果（单据已在审批中，发起人仍可在待办里手动办理）。
    if let Some(task_id) = ret.task_id.as_deref().and_then(|s| s.parse::<i64>().ok()) {
        let auto = CompleteTaskBo {
            task_id: Some(task_id),
            skip_type: Some("PASS".to_string()),
            message: Some(format!("提交申请（{apply_code}）")),
            ..Default::default()
        };
        match task::complete(uid, &auto).await {
            Ok(_) => {
                let instance_id = ret.process_instance_id.parse().unwrap_or_default();
                // 回塞下一个真正待办的任务，保持返回值语义
                ret.task_id = task::first_task_id(instance_id).await.map(|id| id.to_string());
            }
            Err(e) => log::warn!("[workflow] 自动办结「申请人」节点失败，可手动办理: {e}"),
        }
    }

    Ok(StartReturnVO {
        business_code: Some(apply_code),
        ..ret
    })
}

/// 按业务主键查请假单（流程详情页反查业务数据）
pub async fn find_by_business(business_id: &str) -> Result<Option<LeaveVO>> {
    let Ok(id) = business_id.trim().parse::<i64>() else {
        return Ok(None);
    };
    let model = TestLeaveEntity::find()
        .filter(TestLeaveColumn::Id.eq(id))
        .one(crate::pool!())
        .await?;
    Ok(model.map(LeaveVO::from))
}

// ----------------------------------------------------------------------
// 内部辅助
// ----------------------------------------------------------------------

fn leave_type_of(bo: &LeaveBo) -> String {
    bo.leave_type
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("1")
        .to_string()
}

/// 解析起止时间并计算天数（含首尾，最小 1 天）
fn normalize(bo: &LeaveBo) -> Result<(chrono::NaiveDateTime, chrono::NaiveDateTime, i32)> {
    let start = bo
        .start_dt()
        .map_err(Error::from)?
        .ok_or_else(|| Error::from("请假开始时间不能为空"))?;
    let end = bo
        .end_dt()
        .map_err(Error::from)?
        .ok_or_else(|| Error::from("请假结束时间不能为空"))?;
    if end < start {
        return Err(Error::from("请假结束时间不能早于开始时间"));
    }
    let days = bo.leave_days.unwrap_or_else(|| {
        let span = (end.date() - start.date()).num_days() + 1;
        span.max(1) as i32
    });
    Ok((start, end, days))
}

/// 申请编号：`LV` + 日期 + 单号后 6 位
fn apply_code(leave_id: i64) -> String {
    format!(
        "LV{}{:06}",
        chrono::Local::now().format("%Y%m%d"),
        leave_id.rem_euclid(1_000_000)
    )
}
