
use std::sync::{Mutex, OnceLock, RwLock};

use rbdc_pg::driver::PgDriver;
// use once_cell::sync::Lazy;
use lazy_static::lazy_static;

pub mod tableinfo;
pub mod frontcode;
pub mod backcode;
pub mod sysmenu;

/// CONTEXT is all of the service struct
//pub static CONTEXT: OnceLock<RwLock<ServiceContext>> = OnceLock::new();
lazy_static! {
    pub static ref CONTEXT: Mutex<ServiceContext> = Mutex::new(ServiceContext::default());
}
//pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &$crate::codegen::CONTEXT.lock().unwrap().rb
    };
}

pub struct ServiceContext {
 
    pub rb: rbatis::RBatis,
    pub char_array: [char;52]
}

impl ServiceContext {
        /// init database pool
        pub async fn init_pool(&self) {
            //连接数据库
            let database_url: &str = "postgres://postgres:123456@127.0.0.1:5432/ruoyirust";
            println!(
                "[ruoyi_rust] rbatis pool init ({})..." , database_url
            );
            

            self.rb
                .init(PgDriver {}, &database_url)
                
                .expect("[ruoyi_rust] rbatis pool init fail!");
    
                self.rb.get_pool().unwrap().set_max_open_conns(10).await;
            log::info!(
                "[ruoyi_rust] rbatis pool init success! pool state = {:?}",
                self.rb.get_pool().expect("pool not init!").state().await
            );
            //self.rb.get_pool().unwrap().;
    
            
        }
    }

    impl Default for ServiceContext {
        fn default() -> Self {
            
            let str = "abcdefghkmnopqrstuwxyz23456789ABCDEFGHKMNOPQRSTUWXYZ";
            let char_array1: Vec<char> = str.chars().collect();
            let char_array2: [char; 52] = char_array1.into_iter().collect::<Vec<char>>().try_into().unwrap();

            let rbatis = rbatis::RBatis::new();
    if rbatis.is_debug_mode() == false  {
        dbg!("rbatis 已使用release模式运行"
        );
    }


            ServiceContext {
                rb: rbatis,
                char_array:char_array2
            }
        }
    }
    