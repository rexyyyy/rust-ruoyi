use sea_orm::{ConnectionTrait, DbBackend, Statement};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("profile", "dev");
    backend::config::log::init_log();
    backend::db::init_db().await?;
    let conn = backend::db::db().expect("db not initialized");

    let cols = conn
        .query_all(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT column_name FROM information_schema.columns WHERE table_schema='public' AND table_name='sys_menu' ORDER BY ordinal_position",
            vec![],
        ))
        .await?;
    println!("== sys_menu columns ==");
    for r in &cols {
        println!("  {}", r.try_get::<String>("", "column_name")?);
    }

    let rows = conn
        .query_all(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT menu_id, menu_name, parent_id, menu_type, perms FROM sys_menu WHERE menu_name LIKE '%oms%' OR perms LIKE 'oms:%' ORDER BY menu_id",
            vec![],
        ))
        .await?;
    println!("\n== existing oms menus: {} ==", rows.len());
    for r in &rows {
        let id: i64 = r.try_get("", "menu_id")?;
        let name: String = r.try_get("", "menu_name")?;
        let pid: i64 = r.try_get("", "parent_id")?;
        let mtype: String = r.try_get("", "menu_type")?;
        let perms: String = r.try_get("", "perms")?;
        println!("  id={id} parent={pid} type={mtype} name={name} perms={perms}");
    }

    let max: Option<i64> = conn
        .query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT MAX(menu_id) AS m FROM sys_menu",
            vec![],
        ))
        .await?
        .and_then(|r| r.try_get::<Option<i64>>("", "m").ok().flatten());
    println!("\nmax menu_id = {:?}", max);

    let m = conn
        .query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT COUNT(*) AS c FROM sys_menu WHERE perms='oms:regorder:list'",
            vec![],
        ))
        .await?;
    if let Some(r) = m {
        let c: i64 = r.try_get("", "c")?;
        println!("existing oms:regorder:list menu records = {}", c);
    }
    Ok(())
}