//! 引擎门面：把「读元数据 + 渲染 + 输出」串成几个高层 API。
//!
//! backend 的 `/tool/gen/*` 接口只需要用到这几个方法。

use std::collections::BTreeMap;

use crate::error::Result;
use crate::gen::{self, GenCtx, GenFile, GenOptions};
use crate::meta;
use crate::util::zip::zip_files;

pub struct CodegenEngine;

impl CodegenEngine {
    /// 读取一张表的元数据
    pub async fn table_meta(schema: &str, table_name: &str) -> Result<meta::TableMeta> {
        meta::reader::load_table(schema, table_name).await
    }

    /// 列出库里所有可导入的物理表
    pub async fn db_tables(schema: &str) -> Result<Vec<meta::DbTableRow>> {
        meta::reader::list_tables(schema).await
    }

    /// 按表元数据构造渲染上下文
    pub fn build_ctx(table: &meta::TableMeta, opt: &GenOptions) -> Result<GenCtx> {
        GenCtx::build(table, opt)
    }

    /// 预览：`文件路径 -> 内容`
    pub async fn preview(
        schema: &str,
        table_name: &str,
        opt: &GenOptions,
    ) -> Result<BTreeMap<String, String>> {
        let table = Self::table_meta(schema, table_name).await?;
        let ctx = Self::build_ctx(&table, opt)?;
        Ok(gen::to_preview_map(&gen::render_all(&ctx)?))
    }

    /// 渲染出全部文件（不落盘）
    pub async fn render(schema: &str, table_name: &str, opt: &GenOptions) -> Result<Vec<GenFile>> {
        let table = Self::table_meta(schema, table_name).await?;
        let ctx = Self::build_ctx(&table, opt)?;
        gen::render_all(&ctx)
    }

    /// 生成并写入磁盘
    pub async fn generate_to(
        root: &str,
        schema: &str,
        table_name: &str,
        opt: &GenOptions,
    ) -> Result<Vec<GenFile>> {
        let files = Self::render(schema, table_name, opt).await?;
        crate::util::write_all(&files, root)?;
        Ok(files)
    }

    /// 多张表打包成一个 zip（`/tool/gen/batchGenCode` 用）
    pub async fn zip(
        schema: &str,
        items: Vec<(String, GenOptions)>,
    ) -> Result<Vec<u8>> {
        let mut out: Vec<(String, String)> = Vec::new();
        for (table_name, opt) in items {
            let files = Self::render(schema, &table_name, &opt).await?;
            for f in files {
                out.push((f.path, f.content));
            }
        }
        Ok(zip_files(out.iter().map(|(p, c)| (p.as_str(), c.as_str()))))
    }
}
