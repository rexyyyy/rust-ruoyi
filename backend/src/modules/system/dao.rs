//! sea-orm 通用 DAO 助手，替代 rbatis `crud!` 宏生成的 `insert/update/delete/select_page`。
//!
//! 设计目标：让 service 层从 rbatis 调用切换到 sea-orm 时改动最小、类型安全。
//! - `insert_model` / `update_model` / `delete_model`：按主键值操作单条记录，返回受影响行数（`u64`）。
//! - `insert_batch`：批量插入，返回插入条数。
//! - `paginate`：把 `Select<E>` 包成 `Page<Model>`，自动做 `count` + `LIMIT/OFFSET` 分页。
//!
//! 实体定义见 [`crate::modules::system::domain::tables`]，约定：
//! `SysXxx`（Model）/ `SysXxxEntity`（Entity）/ `SysXxxColumn`（Column）/ `SysXxxActiveModel`（ActiveModel）。

use sea_orm::entity::*;
use sea_orm::query::*;
use sea_orm::*;
use sea_orm::PaginatorTrait;
use crate::modules::system::domain::{Page, PageRequest};
use crate::error::{Error, Result};

/// 单条插入，返回受影响行数（sea-orm 不直接给 rows_affected，单条插入恒为 1）。
pub async fn insert_model<M, A>(db: &DatabaseConnection, m: M) -> Result<u64>
where
    M: IntoActiveModel<A>,
    A: ActiveModelTrait + ActiveModelBehavior + Send,
    A::Entity: EntityTrait,
    <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
{
    m.into_active_model()
        .insert(db)
        .await
        .map(|_| 1)
        .map_err(Error::from)
}

/// 按主键更新（全字段写入），返回受影响行数。
///
/// 注意：sea-orm 的 UPDATE 只会把 `ActiveValue::Set` 写进 SET 子句，
/// `Unchanged` / `NotSet` 一律跳过（见 sea-orm `query/update.rs`）。
/// 而 `Model::into_active_model()` 产出的全是 `Unchanged`——直接传 Model
/// 会得到一条没有 SET 的 UPDATE（执行成功但什么都没改！）。这里统一把
/// `Unchanged` 提升为 `Set`，实现「传完整模型即全字段写入」的语义，
/// 与 rbatis `update_by_map`（传完整实体）对齐。
pub async fn update_model<M, A>(db: &DatabaseConnection, m: M) -> Result<u64>
where
    M: IntoActiveModel<A>,
    A: ActiveModelTrait + ActiveModelBehavior + Send,
    A::Entity: EntityTrait,
    <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
{
    let mut am = m.into_active_model();
    for col in <A::Entity as EntityTrait>::Column::iter() {
        if let ActiveValue::Unchanged(v) = am.get(col) {
            am.set(col, v);
        }
    }
    match A::Entity::update(am).exec(db).await {
        Ok(_) => Ok(1),
        Err(DbErr::RecordNotFound(_)) => Ok(0),
        Err(e) => Err(Error::from(e)),
    }
}

/// 按主键删除，返回受影响行数。
pub async fn delete_model<M, A>(db: &DatabaseConnection, m: M) -> Result<u64>
where
    M: IntoActiveModel<A>,
    A: ActiveModelTrait + ActiveModelBehavior + Send,
    A::Entity: EntityTrait,
{
    let am = m.into_active_model();
    A::Entity::delete(am)
        .exec(db)
        .await
        .map(|r| r.rows_affected)
        .map_err(Error::from)
}

/// 批量插入，返回插入条数。
pub async fn insert_batch<A>(db: &DatabaseConnection, ams: Vec<A>) -> Result<u64>
where
    A: ActiveModelTrait + ActiveModelBehavior + Send,
    A::Entity: EntityTrait,
{
    let n = ams.len() as u64;
    if n == 0 {
        return Ok(0);
    }
    A::Entity::insert_many(ams).exec(db).await.map_err(Error::from)?;
    Ok(n)
}

/// 把 `Select<E>` 包成分页结果：先 `count` 总数，再按 `page_no/page_size` 取值。
///
/// `page_no` 为 1 基（与 rbatis 一致），内部自动转成 0 基 offset。
pub async fn paginate<E>(db: &DatabaseConnection, select: Select<E>, pr: &PageRequest) -> Result<Page<E::Model>>
where
    E: EntityTrait,
    E::Model: FromQueryResult + Send + Sync + 'static,
{
    let total = select
        .clone()
        .count(db)
        .await
        .map_err(Error::from)?;
    let per_page = pr.page_size.max(1);
    let page_index = pr.page_no.max(1).saturating_sub(1);
    let records = select
        .paginate(db, per_page)
        .fetch_page(page_index)
        .await
        .map_err(Error::from)?;
    Ok(Page::new(records, total, pr.page_no, pr.page_size))
}
