use backend::modules::system::domain::vo::UserCache;
use backend::modules::system::service::CONTEXT;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("profile", "dev");
    backend::db::init_db().await?;
    let keys = CONTEXT.cache_service.keys("login_tokens:*").await?;
    println!("login tokens in redis: {}", keys.len());
    for k in keys {
        let uc: Option<UserCache> = CONTEXT.cache_service.get_json(&k).await.ok();
        match uc {
            Some(u) => println!(
                "{}  user={} id={} menu_ids={}",
                k, u.user_name, u.id, u.menu_ids.len()
            ),
            None => println!("{}  <unreadable>", k),
        }
    }
    Ok(())
}