//! DemoCourse 服务层
//! 源表：demo_course　业务：演示课程表
//! 由 codegen 于 2026-10-08 21:03:18 生成。
//!
//! 数据访问统一走 `system::dao` 的助手函数（`paginate` / `insert_model` / `update_model`），
//! 与 backend 既有模块保持同一套写法。

use sea_orm::entity::*;
use sea_orm::query::*;

use crate::error::Error;
use crate::error::Result;
use crate::modules::system::dao::{insert_model, paginate, update_model};
use crate::modules::system::domain::{Page, PageRequest};
use crate::modules::system::service::CONTEXT;
use crate::pool;

use crate::modules::demo::domain::bo::DemoCoursePageDTO;
use crate::modules::demo::domain::entity::{
    DemoCourse, DemoCourseColumn, DemoCourseEntity,
};
use crate::modules::demo::domain::vo::DemoCourseVO;

pub struct DemoCourseService {}

impl DemoCourseService {
    /// 分页列表
    pub async fn page(&self, arg: &DemoCoursePageDTO) -> Result<Page<DemoCourseVO>> {
        let page_req = &PageRequest::from(arg);
        let mut select = DemoCourseEntity::find();

        if let Some(v) = &arg.course_code {
            if !v.is_empty() {
                select = select.filter(DemoCourseColumn::CourseCode.contains(v.as_str()));
            }
        }

        if let Some(v) = &arg.course_name {
            if !v.is_empty() {
                select = select.filter(DemoCourseColumn::CourseName.contains(v.as_str()));
            }
        }

        select = select.order_by_desc(DemoCourseColumn::CourseId);
        let page = paginate(pool!(), select, page_req).await?;
        Ok(page.convert::<DemoCourseVO>())
    }

    /// 全部（不分页）
    pub async fn finds_all(&self) -> Result<Vec<DemoCourseVO>> {
        let data = DemoCourseEntity::find()
            .order_by_desc(DemoCourseColumn::CourseId)
            .all(pool!())
            .await?;
        Ok(data.into_iter().map(DemoCourseVO::from).collect())
    }

    /// 按主键取单条
    pub async fn detail(&self, id: &str) -> Result<DemoCourseVO> {
        let row = DemoCourseEntity::find()
            .filter(DemoCourseColumn::CourseId.eq(id.parse::<i64>().unwrap_or(0)))
            .one(pool!())
            .await?
            .ok_or_else(|| Error::from(format!("不存在:{}", id)))?;
        Ok(DemoCourseVO::from(row))
    }

    /// 新增
    pub async fn add(&self, arg: &DemoCourse) -> Result<u64> {
        Ok(insert_model(pool!(), arg.clone()).await?)
    }

    /// 修改。DTO 未提供到的字段保持原值（`Option::take().or()` 语义），
    /// 避免前端只提交部分表单时把其余列清空。
    pub async fn update(&self, data: DemoCourse) -> Result<u64> {
        let old = DemoCourseEntity::find()
            .filter(DemoCourseColumn::CourseId.eq(data.course_id.clone()))
            .one(pool!())
            .await?
            .ok_or_else(|| Error::from("参数错误"))?;
        let mut merged = old;
        merged.course_code = data.course_code.or(merged.course_code.take());
        merged.course_name = data.course_name.or(merged.course_name.take());
        merged.teacher = data.teacher.or(merged.teacher.take());
        merged.credit = data.credit.or(merged.credit.take());
        merged.open_status = data.open_status.or(merged.open_status.take());
        merged.start_date = data.start_date.or(merged.start_date.take());
        merged.remark = data.remark.or(merged.remark.take());
        merged.create_by = data.create_by.or(merged.create_by.take());
        merged.create_time = data.create_time.or(merged.create_time.take());
        merged.update_by = data.update_by.or(merged.update_by.take());
        merged.update_time = data.update_time.or(merged.update_time.take());

        Ok(update_model(pool!(), merged).await?)
    }

    /// 删除（写回收站）
    pub async fn remove(&self, id: &str) -> Result<u64> {
        use sea_orm::TransactionTrait;
        // 「删课程 + 写回收站」必须原子；快照在事务内读取以保证与删除一致。
        let txn = pool!().begin().await.map_err(Error::from)?;

        let targets = DemoCourseEntity::find()
            .filter(DemoCourseColumn::CourseId.eq(id.parse::<i64>().unwrap_or(0)))
            .all(&txn)
            .await?;
        let rows_affected = DemoCourseEntity::delete_many()
            .filter(DemoCourseColumn::CourseId.eq(id.parse::<i64>().unwrap_or(0)))
            .exec(&txn)
            .await?
            .rows_affected;
        if rows_affected > 0 {
            // copy data to trash
            CONTEXT
                .sys_trash_service
                .add_in_txn(&txn, "demo_course", &targets)
                .await?;
        }
        txn.commit().await.map_err(Error::from)?;
        Ok(rows_affected)
    }
}
