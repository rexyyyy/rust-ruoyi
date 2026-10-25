pub mod context;

use serde::Serialize;
use std::collections::BTreeMap;

use crate::error::Result;
use crate::template;

pub use context::{ColCtx, GenColumnOverride, GenCtx, GenOptions, PkCtx};

/// 一个待输出的文件
#[derive(Clone, Debug)]
pub struct GenFile {
    /// 相对路径（POSIX 分隔符），例如 `backend/src/modules/demo/domain/entity.rs`
    pub path: String,
    pub content: String,
}

impl GenFile {
    pub fn new(path: impl Into<String>, content: impl Into<String>) -> Self {
        GenFile {
            path: path.into(),
            content: content.into(),
        }
    }
}

fn ctx_of<T: Serialize>(ctx: &T) -> Result<tera::Context> {
    Ok(tera::Context::from_serialize(ctx)?)
}

/// 渲染全部文件（后端 + 前端 + 菜单 SQL）
pub fn render_all(ctx: &GenCtx) -> Result<Vec<GenFile>> {
    let c = ctx_of(ctx)?;
    let mut files = Vec::new();

    // ---------------- 后端 Rust ----------------
    let prefix = format!("backend/src/modules/{}", ctx.rust_module);
    push(&mut files, &prefix, "mod.rs", template::render(template::name::BE_MODULE_MOD, &c)?);
    push(&mut files, &format!("{prefix}/domain"), "mod.rs", template::render(template::name::BE_DOMAIN_MOD, &c)?);
    push(&mut files, &format!("{prefix}/domain"), "entity.rs", template::render(template::name::BE_ENTITY, &c)?);
    push(&mut files, &format!("{prefix}/domain"), "bo.rs", template::render(template::name::BE_BO, &c)?);
    push(&mut files, &format!("{prefix}/domain"), "vo.rs", template::render(template::name::BE_VO, &c)?);
    push(&mut files, &format!("{prefix}/service"), "mod.rs", template::render(template::name::BE_SERVICE_MOD, &c)?);
    push(
        &mut files,
        &format!("{prefix}/service"),
        &format!("{}_service.rs", ctx.biz_file),
        template::render(template::name::BE_SERVICE, &c)?,
    );
    push(&mut files, &format!("{prefix}/controller"), "mod.rs", template::render(template::name::BE_CONTROLLER_MOD, &c)?);
    push(
        &mut files,
        &format!("{prefix}/controller"),
        &format!("{}_controller.rs", ctx.biz_file),
        template::render(template::name::BE_CONTROLLER, &c)?,
    );

    // ---------------- 前端 Vue3 ----------------
    let fe_api = format!("frontend/src/api/{}/{}", ctx.module_name, ctx.business_name);
    push(&mut files, &fe_api, "index.ts", template::render(template::name::FE_API, &c)?);
    push(&mut files, &fe_api, "types.ts", template::render(template::name::FE_TYPES, &c)?);
    let fe_view = format!("frontend/src/views/{}/{}", ctx.module_name, ctx.business_name);
    push(&mut files, &fe_view, "index.vue", template::render(template::name::FE_INDEX_VUE, &c)?);

    // ---------------- 菜单 SQL ----------------
    push(
        &mut files,
        "sql",
        &format!("{}_{}_menu.sql", ctx.module_name, ctx.business_name),
        template::render(template::name::SQL_MENU, &c)?,
    );

    Ok(files)
}

fn push(files: &mut Vec<GenFile>, dir: &str, name: &str, content: String) {
    files.push(GenFile::new(format!("{dir}/{name}"), content));
}

/// 预览用：`文件名 -> 内容`，给到 `/tool/gen/preview/{id}`
pub fn to_preview_map(files: &[GenFile]) -> BTreeMap<String, String> {
    files
        .iter()
        .map(|f| (f.path.clone(), f.content.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta::column::{ColumnMeta, TableMeta};

    fn sample_table(pk_type: &str) -> TableMeta {
        TableMeta {
            table_name: "demo_course".to_string(),
            table_comment: "课程表".to_string(),
            columns: vec![
                ColumnMeta {
                    column_name: "course_id".to_string(),
                    column_type: pk_type.to_string(),
                    column_comment: "课程ID".to_string(),
                    is_pk: true,
                    is_nullable: false,
                    is_increment: false,
                    ordinal: 1,
                    max_length: None,
                    numeric_precision: None,
                    numeric_scale: None,
                },
                ColumnMeta {
                    column_name: "course_name".to_string(),
                    column_type: "varchar".to_string(),
                    column_comment: "课程名".to_string(),
                    is_pk: false,
                    is_nullable: true,
                    is_increment: false,
                    ordinal: 2,
                    max_length: Some(100),
                    numeric_precision: None,
                    numeric_scale: None,
                },
                ColumnMeta {
                    column_name: "create_time".to_string(),
                    column_type: "timestamp".to_string(),
                    column_comment: "创建时间".to_string(),
                    is_pk: false,
                    is_nullable: true,
                    is_increment: false,
                    ordinal: 3,
                    max_length: None,
                    numeric_precision: None,
                    numeric_scale: None,
                },
            ],
        }
    }

    /// 生成的 controller 的 add 应保留「表内自增主键」提示（get_table_maxid），
    /// 且按主键类型给出正确的赋值写法（String 主键需 to_string）。
    #[test]
    fn controller_has_table_maxid_hint() {
        for (db_type, _rust_type) in [("int8", "i64"), ("varchar", "String")] {
            let table = sample_table(db_type);
            let opt = GenOptions {
                module_name: "demo".to_string(),
                business_name: Some("course".to_string()),
                function_name: Some("课程".to_string()),
                ..Default::default()
            };
            let ctx = GenCtx::build(&table, &opt).unwrap();
            let files = render_all(&ctx).unwrap();
            let controller = files
                .iter()
                .find(|f| f.path.ends_with("course_controller.rs"))
                .expect("controller file generated");
            assert!(
                controller.content.contains(
                    "data.course_id = crate::modules::system::service::ServiceContext::get_table_maxid(\"course_id\", \"demo_course\")"
                ),
                "controller 应包含 get_table_maxid 主键赋值提示"
            );
            assert!(
                controller.content.contains("get_table_maxid_with_date_prefix"),
                "controller 应提示带日期前缀的变体"
            );
        }
    }
}
