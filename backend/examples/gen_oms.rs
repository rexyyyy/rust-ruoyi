use codegen::service::CodegenEngine;
use codegen::GenOptions;
use sea_orm::{ConnectionTrait, DbBackend, Statement};

/// 阶段性工具：按 codegen/oms_order.sql 在 dev 库建立 oms_regorder 表，
/// 并用 codegen 引擎做一次真实渲染测试（读元数据 -> 渲染 -> 落盘到 E:\tmp\opencode\oms_gen_out）。
/// 用法：cargo run --example gen_oms -- profile=dev
#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let profile = std::env::args().find(|a| a.starts_with("profile="));
    if let Some(p) = &profile {
        std::env::set_var("profile", p.trim_start_matches("profile="));
    }
    backend::config::log::init_log();
    backend::db::init_db().await?;
    let conn = backend::db::db().expect("db not initialized");
    codegen::db::init_external(conn);

    let ddl = r#"
CREATE TABLE IF NOT EXISTS public.oms_regorder (
    "regid" int8 NOT NULL,
    "order_sn" varchar(64),
    "member_username" varchar(64),
    "total_amount" float8,
    "pay_amount" float8,
    "pay_type" varchar(2),
    "source_type" varchar(2),
    "status" varchar(2),
    "order_type" varchar(2),
    "delivery_company" varchar(64),
    "request_sn" varchar(256),
    "delivery_sn" varchar(256),
    "receiver_name" varchar(100) NOT NULL,
    "receiver_phone" varchar(32) NOT NULL,
    "receiver_province" varchar(32),
    "receiver_city" varchar(32),
    "receiver_region" varchar(32),
    "note" varchar(500),
    "confirm_status" varchar(2),
    "payment_time" timestamp,
    "delivery_time" timestamp,
    "receive_time" timestamp,
    "comment_time" timestamp,
    "del_flag" char(1),
    "create_by" varchar(64),
    "create_time" timestamp,
    "update_by" varchar(64),
    "update_time" timestamp,
    CONSTRAINT oms_regorder_pkey PRIMARY KEY ("regid")
)"#;
    conn.execute(Statement::from_string(DbBackend::Postgres, ddl)).await?;

    let comments = [
        r#"COMMENT ON TABLE public.oms_regorder IS '注册码登记'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."regid" IS '主键'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."order_sn" IS '订单号'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."member_username" IS '用户名'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."total_amount" IS '订单总金额'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."pay_amount" IS '应付金额'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."pay_type" IS '支付方式：0->未支付；1->支付宝；2->微信'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."source_type" IS '订单来源：0->PC订单；1->app订单'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."status" IS '订单状态：0->待付款；1->待发货；2->已发货；3->已完成；4->已关闭；5->无效订单'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."order_type" IS '订单类型：0->正常订单；1->秒杀订单'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."delivery_company" IS '物流公司(配送方式)'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."request_sn" IS '申请号'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."delivery_sn" IS '回复号'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."receiver_name" IS '收货人姓名'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."receiver_phone" IS '收货人电话'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."receiver_province" IS '省份/直辖市'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."receiver_city" IS '城市'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."receiver_region" IS '区'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."note" IS '订单备注'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."confirm_status" IS '确认收货状态：0->未确认；1->已确认'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."payment_time" IS '支付时间'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."delivery_time" IS '发货时间'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."receive_time" IS '确认收货时间'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."comment_time" IS '评价时间'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."del_flag" IS '删除标志（0代表存在 2代表删除）'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."create_by" IS '创建者'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."create_time" IS '创建时间'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."update_by" IS '更新者'"#,
        r#"COMMENT ON COLUMN public.oms_regorder."update_time" IS '更新时间'"#,
    ];
    for c in comments {
        conn.execute(Statement::from_string(DbBackend::Postgres, c)).await?;
    }
    println!("DDL done, table public.oms_regorder ready");

    let meta = CodegenEngine::table_meta("public", "oms_regorder").await?;
    println!("\n== TableMeta ==");
    println!("table_name={}  comment={}", meta.table_name, meta.table_comment);
    for col in &meta.columns {
        println!(
            "  {:3} {:20} {:12} pk={} inc={} null={} comment={}",
            col.ordinal,
            col.column_name,
            col.column_type,
            col.is_pk,
            col.is_increment,
            col.is_nullable,
            col.column_comment
        );
    }

    let opt = GenOptions {
        module_name: "oms".to_string(),
        business_name: Some("regorder".to_string()),
        function_name: Some("注册码登记".to_string()),
        ..Default::default()
    };
    let files = CodegenEngine::render("public", "oms_regorder", &opt).await?;
    let root = "E:/tmp/opencode/oms_gen_out";
    let written = codegen::util::write_all(&files, root)?;

    println!("\n== rendered {} files ==", written.len());
    for f in &files {
        println!("  {}", f.path);
    }

    if let Some(controller) = files
        .iter()
        .find(|f| f.path.replace('\\', "/").ends_with("regorder_controller.rs"))
    {
        println!("\n== controller (add handler context) ==");
        for (i, line) in controller.content.lines().enumerate() {
            if line.contains("get_table_maxid") || line.contains("let mut data") {
                println!("  {}: {}", i + 1, line.trim());
            }
        }
    }
    Ok(())
}