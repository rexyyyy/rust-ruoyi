//! 代码生成服务：gen_table 配置管理 + 调用 codegen 引擎渲染。

use std::collections::HashMap;

use sea_orm::entity::*;
use sea_orm::query::*;

use codegen::meta::{ColumnMeta, TableMeta};
use codegen::{GenColumnOverride, GenOptions};

use crate::error::{Error, Result};
use crate::modules::system::dao::paginate;
use crate::modules::system::domain::{Page, PageRequest};
use crate::modules::tool::domain::gen_table_column::{self};
use crate::modules::tool::domain::{GenQuery, GenTable, GenTableColumnEnum, GenTableEntity, GenTableInfo, GenTableVO};
use crate::pool;

fn now() -> chrono::NaiveDateTime {
    chrono::Local::now().naive_local()
}

fn next_id() -> i64 {
    crate::util::idseq::next_id()
}

/// '1'/'0' 字符位转 bool（数据库与前端约定的 char 位）
fn flag(v: &Option<String>) -> bool {
    v.as_deref() == Some("1")
}

fn yn(v: bool) -> String {
    if v { "1".to_string() } else { "0".to_string() }
}

/// codegen::Error -> backend Error（统一错误出口）
fn cg_err<T>(r: std::result::Result<T, codegen::Error>) -> Result<T> {
    r.map_err(|e| Error::from(e.to_string()))
}

pub struct GenService {}

impl GenService {
    // ---------------- 列表 ----------------

    /// 生成配置分页
    pub async fn page_list(&self, q: &GenQuery) -> Result<Page<GenTable>> {
        let pr = PageRequest::new(q.page_no.unwrap_or(1), q.page_size.unwrap_or(10));
        let mut select = GenTableEntity::find();
        if let Some(v) = q.table_name.as_deref().filter(|s| !s.is_empty()) {
            select = select.filter(GenTableColumnEnum::TableName.contains(v));
        }
        if let Some(v) = q.table_comment.as_deref().filter(|s| !s.is_empty()) {
            select = select.filter(GenTableColumnEnum::TableComment.contains(v));
        }
        if let Some(v) = q.data_name.as_deref().filter(|s| !s.is_empty()) {
            select = select.filter(GenTableColumnEnum::DataName.eq(v));
        }
        let page = paginate(pool!(), select, &pr).await?;
        Ok(page)
    }

    /// 可导入的物理表（排除已导入），内存过滤 + 分页
    pub async fn db_list(&self, q: &GenQuery) -> Result<Page<codegen::DbTableRow>> {
        let rows = cg_err(
            codegen::CodegenEngine::db_tables(codegen::meta::reader::DEFAULT_SCHEMA).await,
        )?;
        let imported: Vec<String> = GenTableEntity::find()
            .all(pool!())
            .await?
            .into_iter()
            .filter_map(|t| t.table_name)
            .collect();

        let name_kw = q.table_name.as_deref().unwrap_or("").to_lowercase();
        let comment_kw = q.table_comment.as_deref().unwrap_or("").to_lowercase();
        let mut filtered: Vec<codegen::DbTableRow> = rows
            .into_iter()
            .filter(|r| !imported.contains(&r.table_name))
            .filter(|r| name_kw.is_empty() || r.table_name.to_lowercase().contains(&name_kw))
            .filter(|r| comment_kw.is_empty() || r.table_comment.to_lowercase().contains(&comment_kw))
            .collect();
        filtered.sort_by(|a, b| a.table_name.cmp(&b.table_name));

        let total = filtered.len() as u64;
        let page_no = q.page_no.unwrap_or(1).max(1);
        let page_size = q.page_size.unwrap_or(10).max(1);
        let start = ((page_no - 1) * page_size) as usize;
        let records: Vec<codegen::DbTableRow> = filtered
            .into_iter()
            .skip(start)
            .take(page_size as usize)
            .collect();
        Ok(Page::new(records, total, page_no, page_size))
    }

    // ---------------- 详情 / 配置 ----------------

    /// 详情：info + rows(columns) + tables
    pub async fn get_info(&self, table_id: i64) -> Result<GenTableVO> {
        let info = GenTableEntity::find()
            .filter(GenTableColumnEnum::TableId.eq(table_id))
            .one(pool!())
            .await?
            .ok_or_else(|| Error::from(format!("生成配置不存在: {}", table_id)))?;
        let rows = gen_table_column::GenTableColumnEntity::find()
            .filter(gen_table_column::GenTableColumnEnum::TableId.eq(table_id))
            .order_by_asc(gen_table_column::GenTableColumnEnum::Sort)
            .all(pool!())
            .await?;
        Ok(GenTableVO {
            info: GenTableInfo::from(info),
            rows,
            tables: Vec::new(),
        })
    }

    /// 修改配置（表信息 + 列配置）
    pub async fn update(
        &self,
        table_id: i64,
        table: crate::modules::tool::domain::GenTableForm,
    ) -> Result<u64> {
        let old = GenTableEntity::find()
            .filter(GenTableColumnEnum::TableId.eq(table_id))
            .one(pool!())
            .await?
            .ok_or_else(|| Error::from(format!("生成配置不存在: {}", table_id)))?;
        let mut merged = old;
        merged.table_name = table.table_name.or(merged.table_name.take());
        merged.table_comment = table.table_comment.or(merged.table_comment.take());
        merged.class_name = table.class_name.or(merged.class_name.take());
        merged.module_name = table.module_name.or(merged.module_name.take());
        merged.business_name = table.business_name.or(merged.business_name.take());
        merged.function_name = table.function_name.or(merged.function_name.take());
        merged.function_author = table.function_author.or(merged.function_author.take());
        merged.remark = table.remark.or(merged.remark.take());
        // parent_menu_id 为瞬态字段（官方 exist=false），源自 options JSON，
        // 不直接落库为列；如请求携带则并入 options。
        let eff_pid = table
            .parent_menu_id
            .or_else(|| {
                table
                    .params
                    .as_ref()
                    .and_then(|p| p.parent_menu_id.as_deref())
                    .and_then(|s| s.parse().ok())
            });
        merged.options = Self::with_parent_menu_id(merged.options.take(), eff_pid);
        merged.update_time = Some(now());

        let affected = crate::modules::system::dao::update_model(pool!(), merged).await?;

        for c in &table.columns {
            let Some(column_id) = c.column_id else { continue };
            let Some(old_col) = gen_table_column::GenTableColumnEntity::find()
                .filter(gen_table_column::GenTableColumnEnum::ColumnId.eq(column_id))
                .filter(gen_table_column::GenTableColumnEnum::TableId.eq(table_id))
                .one(pool!())
                .await?
            else {
                continue;
            };
            let mut cm = old_col;
            cm.column_comment = c.column_comment.clone().or(cm.column_comment.take());
            cm.java_field = c.java_field.clone().or(cm.java_field.take());
            cm.is_pk = c.is_pk.clone().or(cm.is_pk.take());
            cm.is_increment = c.is_increment.clone().or(cm.is_increment.take());
            cm.is_required = c.is_required.clone().or(cm.is_required.take());
            cm.is_insert = c.is_insert.clone().or(cm.is_insert.take());
            cm.is_edit = c.is_edit.clone().or(cm.is_edit.take());
            cm.is_list = c.is_list.clone().or(cm.is_list.take());
            cm.is_query = c.is_query.clone().or(cm.is_query.take());
            cm.query_type = c.query_type.clone().or(cm.query_type.take());
            cm.html_type = c.html_type.clone().or(cm.html_type.take());
            cm.dict_type = c.dict_type.clone().or(cm.dict_type.take());
            cm.sort = c.sort.or(cm.sort.take());
            cm.update_time = Some(now());
            crate::modules::system::dao::update_model(pool!(), cm).await?;
        }
        Ok(affected)
    }

    /// 导入表：读物理表元数据，按默认规则生成列配置
    pub async fn import_tables(&self, tables: &str, data_name: &str) -> Result<u64> {
        let names: Vec<&str> = tables.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        if names.is_empty() {
            return Err(Error::from("请选择要导入的表"));
        }
        let mut count = 0u64;
        for name in names {
            let table = cg_err(
                codegen::CodegenEngine::table_meta(
                    codegen::meta::reader::DEFAULT_SCHEMA,
                    name,
                )
                .await,
            )?;
            self.import_one(&table, data_name).await?;
            count += 1;
        }
        Ok(count)
    }

    async fn import_one(&self, table: &TableMeta, data_name: &str) -> Result<()> {
        let module = codegen::naming::to_snake(
            table
                .table_name
                .split('_')
                .next()
                .unwrap_or(table.table_name.as_str()),
        );
        let business = codegen::naming::business_name(&table.table_name);
        let class = codegen::naming::to_pascal(&table.table_name);
        let function = if table.table_comment.is_empty() {
            table.table_name.clone()
        } else {
            table.table_comment.clone()
        };
        let table_id = next_id();

        let gt = GenTable {
            table_id,
            data_name: Some(data_name.to_string()),
            table_name: Some(table.table_name.clone()),
            table_comment: Some(table.table_comment.clone()),
            sub_table_name: None,
            sub_table_fk_name: None,
            class_name: Some(class),
            tpl_category: Some("crud".to_string()),
            package_name: Some("backend".to_string()),
            module_name: Some(module),
            business_name: Some(business),
            function_name: Some(function),
            function_author: Some("ruoyi-rust".to_string()),
            gen_type: Some("0".to_string()),
            gen_path: Some("/".to_string()),
            options: Some("{\"parentMenuId\":0}".to_string()),
            create_dept: None,
            remark: None,
            create_by: Some(1),
            create_time: Some(now()),
            update_by: None,
            update_time: None,
        };
        crate::modules::system::dao::insert_model(pool!(), gt).await?;

        for (i, col) in table.columns.iter().enumerate() {
            let gc = self.column_from_meta(table_id, col, i as i32)?;
            crate::modules::system::dao::insert_model(pool!(), gc).await?;
        }
        Ok(())
    }

    /// 物理列 -> gen_table_column 配置（默认规则与 codegen typemap 一致）
    fn column_from_meta(
        &self,
        table_id: i64,
        col: &ColumnMeta,
        sort: i32,
    ) -> Result<gen_table_column::Model> {
        use codegen::meta::typemap as tm;
        let field = codegen::naming::to_camel(&col.column_name);
        Ok(gen_table_column::Model {
            column_id: next_id(),
            table_id,
            column_name: Some(col.column_name.clone()),
            column_comment: Some(col.column_comment.clone()),
            column_type: Some(col.column_type.clone()),
            java_field: Some(field),
            is_pk: Some(yn(col.is_pk)),
            is_increment: Some(yn(col.is_increment)),
            is_required: Some(yn(tm::default_is_required(col))),
            is_insert: Some(yn(tm::default_is_insert(col))),
            is_edit: Some(yn(tm::default_is_edit(col))),
            is_list: Some(yn(tm::default_is_list(col))),
            is_query: Some(yn(tm::default_is_query(col))),
            query_type: Some(tm::query_type_of(col).to_string()),
            html_type: Some(tm::html_type_of(col).to_string()),
            dict_type: Some(String::new()),
            java_type: None,
            sort: Some(sort),
            create_dept: None,
            create_by: Some(1),
            create_time: Some(now()),
            update_by: None,
            update_time: None,
        })
    }

    /// 同步数据库结构：以 information_schema 为准 diff 列
    pub async fn sync_db(&self, table_id: i64) -> Result<u64> {
        let gt = GenTableEntity::find()
            .filter(GenTableColumnEnum::TableId.eq(table_id))
            .one(pool!())
            .await?
            .ok_or_else(|| Error::from(format!("生成配置不存在: {}", table_id)))?;
        let table_name = gt
            .table_name
            .clone()
            .ok_or_else(|| Error::from("缺少表名"))?;

        let latest = cg_err(
            codegen::CodegenEngine::table_meta(
                codegen::meta::reader::DEFAULT_SCHEMA,
                &table_name,
            )
            .await,
        )?;
        let old_cols = gen_table_column::GenTableColumnEntity::find()
            .filter(gen_table_column::GenTableColumnEnum::TableId.eq(table_id))
            .all(pool!())
            .await?;
        let old_map: HashMap<String, gen_table_column::Model> = old_cols
            .into_iter()
            .map(|c| (c.column_name.clone().unwrap_or_default(), c))
            .collect();

        let mut changed = 0u64;
        let mut sort = 0i32;
        for col in &latest.columns {
            sort += 1;
            match old_map.get(&col.column_name) {
                Some(old) => {
                    let mut cm = old.clone();
                    cm.column_comment = Some(col.column_comment.clone());
                    cm.column_type = Some(col.column_type.clone());
                    cm.is_pk = Some(yn(col.is_pk));
                    cm.is_increment = Some(yn(col.is_increment));
                    cm.sort = Some(sort);
                    cm.update_time = Some(now());
                    crate::modules::system::dao::update_model(pool!(), cm).await?;
                }
                None => {
                    let gc = self.column_from_meta(table_id, col, sort)?;
                    crate::modules::system::dao::insert_model(pool!(), gc).await?;
                    changed += 1;
                }
            }
        }
        // 删除已不存在的列
        let latest_names: Vec<String> = latest.columns.iter().map(|c| c.column_name.clone()).collect();
        for (name, old) in &old_map {
            if !latest_names.contains(name) {
                gen_table_column::GenTableColumnEntity::delete_many()
                    .filter(gen_table_column::GenTableColumnEnum::ColumnId.eq(old.column_id))
                    .exec(pool!())
                    .await?;
                changed += 1;
            }
        }
        Ok(changed)
    }

    // ---------------- 生成 ----------------

    /// gen_table 配置 -> 渲染参数
    fn options_of(&self, gt: &GenTable, cols: &[gen_table_column::Model]) -> GenOptions {
        let mut overrides: HashMap<String, GenColumnOverride> = HashMap::new();
        for c in cols {
            let Some(name) = &c.column_name else { continue };
            overrides.insert(
                name.clone(),
                GenColumnOverride {
                    is_insert: Some(flag(&c.is_insert)),
                    is_edit: Some(flag(&c.is_edit)),
                    is_list: Some(flag(&c.is_list)),
                    is_query: Some(flag(&c.is_query)),
                    is_required: Some(flag(&c.is_required)),
                    html_type: c.html_type.clone().filter(|s| !s.is_empty()),
                    query_type: c.query_type.clone().filter(|s| !s.is_empty()),
                    dict_type: c.dict_type.clone().filter(|s| !s.is_empty()),
                },
            );
        }
        GenOptions {
            module_name: gt.module_name.clone().unwrap_or_else(|| "demo".to_string()),
            business_name: Some(
                gt.business_name
                    .clone()
                    .unwrap_or_else(|| codegen::naming::business_name(&gt.table_name.clone().unwrap_or_default())),
            ),
            function_name: Some(
                gt.function_name
                    .clone()
                    .unwrap_or_else(|| gt.table_name.clone().unwrap_or_default()),
            ),
            function_author: gt
                .function_author
                .clone()
                .unwrap_or_else(|| "ruoyi-rust".to_string()),
            parent_menu_id: Self::parent_menu_id_of(gt),
            columns: overrides,
        }
    }

    /// 从 options JSON 解析 parentMenuId（瞬态字段，官方 exist=false），缺省 0
    fn parent_menu_id_of(gt: &GenTable) -> i64 {
        gt.options
            .as_deref()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
            .and_then(|v| v.get("parentMenuId").and_then(|x| x.as_i64()))
            .unwrap_or(0)
    }

    /// 将 parentMenuId 并入 options JSON（仅当显式提供时覆盖）
    fn with_parent_menu_id(options: Option<String>, pid: Option<i64>) -> Option<String> {
        let mut map: serde_json::Map<String, serde_json::Value> = match options.as_deref() {
            Some(s) if !s.is_empty() => {
                serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(s)
                    .unwrap_or_default()
            }
            _ => serde_json::Map::new(),
        };
        if let Some(v) = pid {
            map.insert("parentMenuId".to_string(), serde_json::Value::from(v));
        }
        Some(serde_json::to_string(&map).unwrap_or_default())
    }

    async fn load_config(&self, table_id: i64) -> Result<(GenTable, Vec<gen_table_column::Model>)> {
        let gt = GenTableEntity::find()
            .filter(GenTableColumnEnum::TableId.eq(table_id))
            .one(pool!())
            .await?
            .ok_or_else(|| Error::from(format!("生成配置不存在: {}", table_id)))?;
        let cols = gen_table_column::GenTableColumnEntity::find()
            .filter(gen_table_column::GenTableColumnEnum::TableId.eq(table_id))
            .order_by_asc(gen_table_column::GenTableColumnEnum::Sort)
            .all(pool!())
            .await?;
        Ok((gt, cols))
    }

    /// 预览生成代码：`文件路径 -> 内容`
    pub async fn preview(&self, table_id: i64) -> Result<Vec<(String, String)>> {
        let (gt, cols) = self.load_config(table_id).await?;
        let table_name = gt.table_name.clone().ok_or_else(|| Error::from("缺少表名"))?;
        let opt = self.options_of(&gt, &cols);
        let map = cg_err(
            codegen::CodegenEngine::preview(
                codegen::meta::reader::DEFAULT_SCHEMA,
                &table_name,
                &opt,
            )
            .await,
        )?;
        Ok(map.into_iter().collect())
    }

    /// 按表 id 批量生成并打包 zip
    pub async fn batch_gen(&self, table_ids: &[i64]) -> Result<Vec<u8>> {
        let mut items = Vec::with_capacity(table_ids.len());
        for tid in table_ids {
            let (gt, cols) = self.load_config(*tid).await?;
            let table_name = gt.table_name.clone().ok_or_else(|| Error::from("缺少表名"))?;
            items.push((table_name, self.options_of(&gt, &cols)));
        }
        if items.is_empty() {
            return Err(Error::from("请选择要生成的表"));
        }
        cg_err(codegen::CodegenEngine::zip(codegen::meta::reader::DEFAULT_SCHEMA, items).await)
    }

    /// 数据源名（rust 版单库，固定 master）
    pub async fn data_names(&self) -> Vec<String> {
        vec!["master".to_string()]
    }

    /// 删除配置（含列配置）
    pub async fn remove(&self, table_id: i64) -> Result<u64> {
        use sea_orm::TransactionTrait;
        // 「删列配置 + 删表配置」必须原子：否则会残留孤儿列配置，
        // 导致下次生成同名表时读到上一次的脏列定义。
        let txn = pool!().begin().await.map_err(Error::from)?;

        gen_table_column::GenTableColumnEntity::delete_many()
            .filter(gen_table_column::GenTableColumnEnum::TableId.eq(table_id))
            .exec(&txn)
            .await?;
        let r = GenTableEntity::delete_many()
            .filter(GenTableColumnEnum::TableId.eq(table_id))
            .exec(&txn)
            .await?;

        txn.commit().await.map_err(Error::from)?;
        Ok(r.rows_affected)
    }
}
