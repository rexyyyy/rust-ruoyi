//! 流程分类（flow_category）。对标 Java `FlowCategoryServiceImpl`。
//!
//! 分类是一棵树（OA审批 / 假勤管理 / 请假 ...），设计器的「分类」下拉
//! 与流程定义列表的分类筛选都用它。

use std::collections::HashMap;

use sea_orm::entity::prelude::*;
use sea_orm::query::*;
use sea_orm::{ActiveModelTrait, Set};

use crate::error::{Error, Result};
use crate::modules::workflow::domain::bo::CategoryBo;
use crate::modules::workflow::domain::vo::CategoryVO;
use crate::modules::workflow::domain::{
    FlowCategory, FlowCategoryActiveModel, FlowCategoryColumn, FlowCategoryEntity,
};
use crate::modules::workflow::ui;

/// 分类列表（扁平，按 parent_id + order_num 排序）
pub async fn list(keyword: Option<&str>) -> Result<Vec<CategoryVO>> {
    let mut select = FlowCategoryEntity::find()
        .filter(FlowCategoryColumn::DelFlag.eq("0"))
        .order_by_asc(FlowCategoryColumn::ParentId)
        .order_by_asc(FlowCategoryColumn::OrderNum)
        .order_by_asc(FlowCategoryColumn::CategoryId);
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        select = select.filter(FlowCategoryColumn::CategoryName.contains(kw));
    }
    let rows = select.all(crate::pool!()).await?;
    Ok(rows.into_iter().map(CategoryVO::from).collect())
}

/// 分类树（一次查全量再在内存拼装，分类量级很小）
pub async fn tree() -> Result<Vec<CategoryVO>> {
    Ok(build_tree(list(None).await?, 0))
}

/// 扁平列表 → 树。用 HashMap 分组，避免每个节点都扫一遍全表。
fn build_tree(flat: Vec<CategoryVO>, root_parent: i64) -> Vec<CategoryVO> {
    let mut buckets: HashMap<i64, Vec<CategoryVO>> = HashMap::new();
    for node in flat {
        let pid = node.parent_id.parse::<i64>().unwrap_or(0);
        buckets.entry(pid).or_default().push(node);
    }
    attach(&mut buckets, root_parent)
}

fn attach(buckets: &mut HashMap<i64, Vec<CategoryVO>>, parent_id: i64) -> Vec<CategoryVO> {
    let mut nodes = buckets.remove(&parent_id).unwrap_or_default();
    for node in nodes.iter_mut() {
        let id = node.category_id.parse::<i64>().unwrap_or(0);
        node.children = attach(buckets, id);
    }
    nodes
}

/// 详情
pub async fn detail(category_id: i64) -> Result<CategoryVO> {
    let model = FlowCategoryEntity::find()
        .filter(FlowCategoryColumn::CategoryId.eq(category_id))
        .one(crate::pool!())
        .await?
        .ok_or_else(|| Error::from(format!("流程分类 {category_id} 不存在")))?;
    Ok(CategoryVO::from(model))
}

/// 新增分类
pub async fn add(bo: &CategoryBo, uid: i64) -> Result<i64> {
    let parent_id = bo.parent_id.unwrap_or(0);
    let name = bo
        .category_name
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| Error::from("分类名称不能为空"))?
        .to_string();
    let ancestors = ancestors_of(parent_id).await?;
    let id = crate::util::idseq::next_id() as i64;
    let model = FlowCategory {
        category_id: id,
        parent_id,
        ancestors: Some(ancestors),
        category_name: name,
        order_num: bo.order_num,
        del_flag: Some("0".to_string()),
        create_by: Some(uid),
        create_time: Some(chrono::Local::now().naive_local()),
        update_by: None,
        update_time: None,
    };
    FlowCategoryEntity::insert(FlowCategoryActiveModel::from(model))
        .exec(crate::pool!())
        .await?;
    ui::ui().reload_categories().await.ok();
    Ok(id)
}

/// 修改分类
pub async fn update(bo: &CategoryBo, uid: i64) -> Result<u64> {
    let id = bo.category_id.ok_or_else(|| Error::from("分类 id 不能为空"))?;
    let old = FlowCategoryEntity::find()
        .filter(FlowCategoryColumn::CategoryId.eq(id))
        .one(crate::pool!())
        .await?
        .ok_or_else(|| Error::from(format!("流程分类 {id} 不存在")))?;

    let parent_id = bo.parent_id.unwrap_or(old.parent_id);
    if parent_id == id {
        return Err(Error::from("上级分类不能是自己"));
    }
    let mut active = FlowCategoryActiveModel {
        category_id: Set(old.category_id),
        ..Default::default()
    };
    if let Some(name) = bo.category_name.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        active.category_name = Set(name.to_string());
    }
    if let Some(order_num) = bo.order_num {
        active.order_num = Set(Some(order_num));
    }
    if bo.parent_id.is_some() {
        active.parent_id = Set(parent_id);
        active.ancestors = Set(Some(ancestors_of(parent_id).await?));
    }
    active.update_by = Set(Some(uid));
    active.update_time = Set(Some(chrono::Local::now().naive_local()));
    active.update(crate::pool!()).await?;
    ui::ui().reload_categories().await.ok();
    Ok(1)
}

/// 删除分类：有子分类或被流程定义引用时不允许删除（逻辑删除，del_flag = 2）
pub async fn remove(category_id: i64) -> Result<u64> {
    let children = FlowCategoryEntity::find()
        .filter(FlowCategoryColumn::ParentId.eq(category_id))
        .filter(FlowCategoryColumn::DelFlag.eq("0"))
        .count(crate::pool!())
        .await?;
    if children > 0 {
        return Err(Error::from("存在子分类，不允许删除"));
    }
    let db = crate::db::db().ok_or_else(|| Error::from("数据库未初始化"))?;
    let used = db
        .query_one(crate::db::raw_stmt(
            "SELECT 1 AS hit FROM flow_definition WHERE category = $1 LIMIT 1",
            vec![sea_orm::Value::from(category_id.to_string())],
        ))
        .await?
        .is_some();
    if used {
        return Err(Error::from("分类下存在流程定义，不允许删除"));
    }
    let r = FlowCategoryEntity::update_many()
        .col_expr(FlowCategoryColumn::DelFlag, Expr::value("2"))
        .filter(FlowCategoryColumn::CategoryId.eq(category_id))
        .exec(crate::pool!())
        .await?
        .rows_affected;
    ui::ui().reload_categories().await.ok();
    Ok(r)
}

/// 计算 parent 的祖级串：`0` 表示根，其余为「父的 ancestors + ',' + 父 id」
async fn ancestors_of(parent_id: i64) -> Result<String> {
    if parent_id == 0 {
        return Ok("0".to_string());
    }
    let parent = FlowCategoryEntity::find()
        .filter(FlowCategoryColumn::CategoryId.eq(parent_id))
        .one(crate::pool!())
        .await?
        .ok_or_else(|| Error::from(format!("上级分类 {parent_id} 不存在")))?;
    let base = parent.ancestors.unwrap_or_else(|| "0".to_string());
    Ok(format!("{base},{parent_id}"))
}
