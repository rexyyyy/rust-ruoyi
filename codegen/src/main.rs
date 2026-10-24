//! codegen 命令行工具。
//!
//! 用法示例：
//! ```text
//! cargo run --bin codegen-cli -- \
//!     --database-url postgres://postgres:123456@localhost:5434/ruoyirust \
//!     --table demo_course \
//!     --module demo --business course --function 课程 \
//!     --out D:/tmp/gen-out
//! ```
//!
//! 只传 `--preview` 时不落盘，把生成内容打到标准输出。

use codegen::db;
use codegen::{CodegenEngine, GenOptions};

#[tokio::main]
async fn main() {
    let args = Args::parse(std::env::args().skip(1).collect());

    if args.database_url.is_empty() {
        eprintln!("缺少 --database-url");
        std::process::exit(2);
    }
    if args.table.is_empty() {
        eprintln!("缺少 --table");
        std::process::exit(2);
    }

    if let Err(e) = db::init_db(&args.database_url).await {
        eprintln!("连接数据库失败: {}", e);
        std::process::exit(1);
    }
    println!("[codegen] 已连接 {}", args.database_url);

    let opts = args.to_options();
    match run(&args, &opts).await {
        Ok(()) => {}
        Err(e) => {
            eprintln!("生成失败: {}", e);
            std::process::exit(1);
        }
    }
    db::close_db().await;
}

async fn run(args: &Args, opts: &GenOptions) -> codegen::Result<()> {
    if args.preview {
        let map = CodegenEngine::preview(&args.schema, &args.table, opts).await?;
        for (path, _) in &map {
            println!("---- {} ----", path);
        }
        println!("共 {} 个文件（--preview 模式，未落盘）", map.len());
        return Ok(());
    }

    let files = CodegenEngine::generate_to(&args.out, &args.schema, &args.table, opts).await?;
    println!("\n[codegen] 生成完毕，共 {} 个文件 -> {}", files.len(), args.out);
    for f in &files {
        println!("  {}", f.path);
    }
    Ok(())
}

struct Args {
    database_url: String,
    schema: String,
    table: String,
    module: String,
    business: String,
    function: String,
    author: String,
    parent_menu_id: i64,
    out: String,
    preview: bool,
}

impl Args {
    fn parse(argv: Vec<String>) -> Self {
        let mut a = Args {
            database_url: String::new(),
            schema: codegen::meta::reader::DEFAULT_SCHEMA.to_string(),
            table: String::new(),
            module: String::new(),
            business: String::new(),
            function: String::new(),
            author: "ruoyi-rust".to_string(),
            parent_menu_id: 0,
            out: "./codegen-out".to_string(),
            preview: false,
        };

        // 极简键值解析：不引入 clap，够用且零依赖
        // （不要用宏去捕获迭代器 —— macro_rules 的标识符卫生规则会让 `it` 取不到）
        let mut it = argv.into_iter();
        while let Some(arg) = it.next() {
            match arg.as_str() {
                "--database-url" => a.database_url = it.next().unwrap_or_default(),
                "--schema" => a.schema = it.next().unwrap_or_default(),
                "--table" => a.table = it.next().unwrap_or_default(),
                "--module" => a.module = it.next().unwrap_or_default(),
                "--business" => a.business = it.next().unwrap_or_default(),
                "--function" => a.function = it.next().unwrap_or_default(),
                "--author" => a.author = it.next().unwrap_or_default(),
                "--parent-menu-id" => {
                    a.parent_menu_id = it.next().and_then(|v| v.parse().ok()).unwrap_or(0)
                }
                "--out" => a.out = it.next().unwrap_or_default(),
                "--preview" => a.preview = true,
                "-h" | "--help" => {
                    print_help();
                    std::process::exit(0);
                }
                other => eprintln!("忽略未知参数: {}", other),
            }
        }
        a
    }

    fn to_options(&self) -> GenOptions {
        // 未显式指定模块名时，取表名的第一段（demo_course -> demo）
        let module = if self.module.is_empty() {
            self.table
                .split('_')
                .next()
                .unwrap_or("demo")
                .to_string()
        } else {
            self.module.clone()
        };
        GenOptions {
            module_name: module,
            business_name: Some(self.business.clone()),
            function_name: Some(self.function.clone()),
            function_author: self.author.clone(),
            parent_menu_id: self.parent_menu_id,
            ..Default::default()
        }
    }
}

fn print_help() {
    println!(
        r#"codegen-cli —— RuoYi-Rust 代码生成器

必填:
  --database-url <postgres://user:pass@host:port/db>
  --table        <表名>

可选:
  --schema        <schema>        默认 public
  --module        <模块名>       默认取表名首段
  --business      <业务名>       默认取表名去掉首段后的 camelCase
  --function      <功能中文名>   默认取表注释
  --author        <作者>         默认 ruoyi-rust
  --parent-menu-id <菜单ID>      默认 0（生成一级目录）
  --out           <输出根目录>   默认 ./codegen-out
  --preview                      只预览不落盘
"#
    );
}
