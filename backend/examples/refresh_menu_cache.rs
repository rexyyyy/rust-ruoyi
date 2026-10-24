use backend::config::global_variables::ADMIN_NAME;
use backend::modules::system::domain::vo::UserCache;
use backend::modules::system::service::{SysMenuService, CONTEXT};
use chrono::NaiveDateTime;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("profile", "dev");
    backend::config::log::init_log();
    backend::db::init_db().await?;

    let _ = CONTEXT.cache_service.del("sys_menu:all_vo").await;
    println!("deleted sys_menu:all_vo");

    let svc = SysMenuService {};
    let _ = svc.update_cache().await?;
    let all = svc.all().await?;
    let has_oms = all.iter().any(|m| m.menu_name.as_deref() == Some("oms"));
    println!("menu cache rebuilt, contains oms dir: {has_oms}, total={}", all.len());

    let uc = UserCache {
        id: 1,
        user_name: ADMIN_NAME.to_string(),
        user: None,
        permissions: vec![],
        menu_ids: vec![],
        roles: vec![],
        login_time: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
        token_key: String::new(),
    };
    let routers = svc.get_routers(&uc).await?;
    println!("admin routers count = {}", routers.len());
    for r in &routers {
        let p = r.path.clone().unwrap_or_default();
        println!("  /{} children={}", p, r.children.len());
    }
    Ok(())
}