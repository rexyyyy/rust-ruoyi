use std::fs::{self, File};
use std::io::{ BufWriter, Write};
use std::path::{Path, PathBuf};

extern crate inflector;
use inflector::cases::camelcase::to_camel_case;

use crate::codegen::tableinfo::TableInfo;
use crate::pool;
use crate::error::Result;
use rbs::Value;
use chrono::{Datelike, Local, NaiveDate};
use super::tableinfo::TableFld;

pub struct backcode {}

//后台代码直接输出
//前台代码使用tera模板引擎输出
impl backcode {
    
    // 获取业务流水号
    pub async fn get_table_maxid(idfield:&str,table_name:&str)->i64{
        
        let local = Local::now();
        let year = local.year();
        let month = local.month();
        let day = local.day();
        let prefix = format!("{:04}{:02}{:02}", year, month, day );

        let query_perms =format!( "select max({}) from {} where CAST({} AS TEXT)  like '{}%'",idfield,table_name,idfield,prefix);
        let max_id: i64 = pool!().query_decode(&query_perms, vec![])
            .await
            .unwrap_or(0);

        let mut newid:i64=0;
        if max_id<=0 {
            let new_str = format!("{:04}{:02}{:02}{:05}", year, month, day,1);
            let s = new_str.to_string();
            //字符串转32位有符号整型
            newid = s.parse::<i64>().unwrap();
        }else{
            newid = max_id + 1;
        }
        newid.into()
    }

    // menuname 中文名 如 商城 menupath 英文名 如 mall 等同 module name
    pub async fn make_new_module_menu(file_saved: &str,module_name:&str,menuname:&str,menupath:&str,sub_menuname:&str ,table_name:&str)-> Result<String>{
        
        let mut path = PathBuf::from(file_saved);
        path.push("code");
        path.push("sql");

        match fs::create_dir_all(&path) {
            Ok(_) => println!("sql Directory created"),
            Err(e) => println!("Error creating sql directory: {}", e),
        }
        let sql_file_name = format!("{}{}", module_name, "_menu.sql");
        path.push(sql_file_name);

        let file = File::create(path)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);

        let sql =format!( "select menu_id from sys_menu where parent_id=0 and menu_name='{}'",menuname);
        let result_queried = pool!().query_decode::<Vec<Value>>(&sql,vec![]).await.unwrap() ; 
        let result_queried_len = result_queried.len();
        let mut menu_id:u64 = 0;
        if result_queried_len<=0 {
            let max_menu_id: u64 = pool!().query_decode("select max(menu_id) as count from sys_menu where parent_id=0", vec![])
            .await
            .unwrap_or(0);
        // 加 1
            let new_menu_id=max_menu_id+1;
            let max_order_num: u64 = pool!().query_decode("select max(order_num) as count from sys_menu where parent_id=0", vec![])
            .await
            .unwrap_or(0);
            let new_order_num=max_order_num+1;

            let sql_insert_module= format!("INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query, 
  is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, 
  create_time, update_by, update_time, remark) VALUES 
        ({}, '{}', 0, {}, '{}', NULL, '', '1', '0', 'M', '0', '0', '', 'tool', 'admin', '2022-12-21 20:22:45', 
  'admin', '2023-04-10 15:26:51', '{}目录');", new_menu_id, menuname,new_order_num,menupath,menuname);

            writeln!(writer, "{}",  sql_insert_module)?;
            menu_id=new_menu_id;
            menu_id=menu_id ;
            // println!("{}",sql_insert_module)

            // let insert_result = pool!().exec(&sql_insert_module, vec![]).await;
            // match insert_result {
            //     Ok(_) => {menu_id=new_menu_id;},
            //     Err(_) => {println!("insert new module menu {} failed!",menuname)},
            // }
            
        }else{
            // 有
            let value_ = result_queried.get(0).unwrap();
            // 按字段查询到的是个 map
            let value_map = value_.as_map().unwrap();
            let omenu_id = value_map["menu_id"].as_u64().unwrap();
            menu_id=omenu_id;
            menu_id=menu_id+1;
            // dbg!(value_map);

        }
        writeln!(writer, "// {}",  "按钮权限")?;
        let business_name = & TableInfo::get_business_name(table_name).unwrap_or("".to_string());
        let sub_menupath = business_name;
        Self::make_new_menu_func(&mut writer,menu_id,module_name,sub_menuname,sub_menupath).await;
        // writer.flush();
        
        // let count = pool!()
        // .query_decode("select count(1) as count from biz_activity", vec![])
        // .await;
        return Ok(("").to_string());
    }

    // menuname 中文名 如 商城 menupath 英文名 如 mall
    // module_name 与上级 path 一致
    async fn make_new_menu_func(writer: &mut BufWriter<File>,parent_menu_id:u64,module_name:&str,menuname:&str,menupath:&str)-> Result<String>{
       
        let sql =format!( "select menu_id from sys_menu where parent_id={} and menu_name='{}'",parent_menu_id, menuname);
        //println!( "sql {}",sql);
        let menu_recodrd = pool!().query_decode::<Vec<Value>>(&sql,vec![]).await;
        let is_ook = menu_recodrd.is_ok();
        let mut menu_id:u64 = 0;
        let perms="";

        if(is_ook){
            let value_ = menu_recodrd.unwrap();
            let mut omenu_id=1;
            if value_.len()>0 {
                let value_ = value_.get(0).unwrap();
                let value_map = value_.as_map().unwrap();
                omenu_id = value_map["menu_id"].as_u64().unwrap();
            }
            // 按字段查询到的是个 map
            if omenu_id>1 {
                menu_id=omenu_id+1;
            }
            println!( "{} parent_menu sql {}",omenu_id,sql);
        }else{
            println!( "get menu_id {}",menu_id);
        }
        format!( "get menu_id {}",menu_id);
        if menu_id<=1 {
            let max_menu_id: u64 = pool!().query_decode("select max(menu_id) as count from sys_menu ", vec![])
            .await
            .unwrap_or(0);
            menu_id=max_menu_id+1;
            println!( "get max(menu_id) sql {} add 1 {}",max_menu_id, menu_id);
        }
        
        // println!("{}",sql);
        if menu_id > 0 {
            let mut new_menu_id=menu_id+1;
            let max_order_num: u64 = pool!().query_decode("select max(order_num) as count from sys_menu where parent_id=?", vec![rbs::to_value!(parent_menu_id)])
            .await
            .unwrap_or(0);
        
            let new_order_num=max_order_num+1;

            let component =&format!( "{}/{}/index",module_name,menupath);
            let perms =&format!( "{}:{}:list",module_name,menupath);

            let sql_insert_module= format!("INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query, 
  is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, 
  create_time, update_by, update_time, remark) VALUES 
        ({}, '{}', {}, {}, '{}', '{}', '', '1', '0', 'C', '0', '0', '{}', 'tool', 'admin', '2022-12-21 20:22:45', 
  'admin', '2023-04-10 15:26:51', '{}');", new_menu_id,menuname,parent_menu_id, new_order_num,menupath,component,perms,menuname);

            // println!(" {}",sql_insert_module);
            writeln!(writer, "{}",  sql_insert_module)?;
            // let insert_result = pool!().exec(&sql_insert_module, vec![]).await;
            // match insert_result {
            //     Ok(_) => {menu_id=new_menu_id;},
            //     Err(_) => {println!("insert new menu {} failed!",menuname)},
            // }

            // 菜单 按钮
            let new_parent_menu_id=new_menu_id;
            let max_menu_id: u64 = pool!().query_decode("select max(menu_id) as count from sys_menu", vec![ ])
            .await
            .unwrap_or(0);
            
            let max_order_num: u64 = pool!().query_decode("select max(order_num) as count from sys_menu where parent_id=?", vec![ ])
            .await
            .unwrap_or(0);
    
            new_menu_id=new_menu_id+1;
            let query_new_menu_id=new_menu_id+1;
            let query_new_order_num=max_order_num+1;

            let query_perms =&format!( "{}:{}:query",module_name,menupath);
            let query_menu_name = &format!( "{}查询", menuname);

            let sql_insert_module= format!("INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query, 
  is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, 
  create_time, update_by, update_time, remark) VALUES 
        ({}, '{}', {}, {}, '#', '', '', '1', '0', 'F', '0', '0', '{}', '#', 'admin', '2022-12-21 20:22:45', 
  'admin', '2023-04-10 15:26:51', '{}');", query_new_menu_id,query_menu_name,new_parent_menu_id, query_new_order_num,query_perms,query_menu_name);
            writeln!(writer, "{}",  sql_insert_module)?;
            // let insert_result = pool!().exec(&sql_insert_module, vec![]).await;
            // match insert_result {
            //     Ok(_) => {println!("insert new menu query {} ok!",query_menu_name)},
            //     Err(_) => {println!("insert new menu query {} failed!",query_menu_name)},
            // }

            let add_new_menu_id=query_new_menu_id+1;
            let add_new_order_num=query_new_order_num+1;

            let add_perms =&format!( "{}:{}:add",module_name,menupath);
            let add_menu_name = &format!( "{}新增", menuname);

            let sql_insert_module= format!("INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query, 
  is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, 
  create_time, update_by, update_time, remark) VALUES 
        ({}, '{}', {}, {}, '#', '', '', '1', '0', 'F', '0', '0', '{}', '#', 'admin', '2022-12-21 20:22:45', 
  'admin', '2023-04-10 15:26:51', '{}');", add_new_menu_id,add_menu_name,new_parent_menu_id, add_new_order_num,add_perms,add_menu_name);
            writeln!(writer, "{}",  sql_insert_module)?;
            // let insert_result = pool!().exec(&sql_insert_module, vec![]).await;
            // match insert_result {
            //     Ok(_) => {println!("insert new menu add {} ok!",add_menu_name)},
            //     Err(_) => {println!("insert new menu add {} failed!",add_menu_name)},
            // }

            let edit_new_menu_id=add_new_menu_id+1;
            let edit_new_order_num=add_new_order_num+1;

            let edit_perms =&format!( "{}:{}:edit",module_name,menupath);
            let edit_menu_name = &format!( "{}修改", menuname);

            let sql_insert_module= format!("INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query, 
  is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, 
  create_time, update_by, update_time, remark) VALUES 
        ({}, '{}', {}, {}, '#', '', '', '1', '0', 'F', '0', '0', '{}', '#', 'admin', '2022-12-21 20:22:45', 
  'admin', '2023-04-10 15:26:51', '{}');", edit_new_menu_id,edit_menu_name,new_parent_menu_id, edit_new_order_num,edit_perms,edit_menu_name);
            writeln!(writer, "{}",  sql_insert_module)?;
            // let insert_result = pool!().exec(&sql_insert_module, vec![]).await;
            // match insert_result {
            //     Ok(_) => {println!("insert new edit  {} ok!",edit_menu_name)},
            //     Err(_) => {println!("insert new edit  {} failed!",edit_menu_name)},
            // }
            
            let remove_new_menu_id=edit_new_menu_id+1;
            let remove_new_order_num=edit_new_order_num+1;

            let remove_perms =&format!( "{}:{}:remove",module_name,menupath);
            let remove_menu_name = &format!( "{}删除", menuname);

            let sql_insert_module= format!("INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query, 
  is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, 
  create_time, update_by, update_time, remark) VALUES 
        ({}, '{}', {}, {}, '#', '', '', '1', '0', 'F', '0', '0', '{}', '#', 'admin', '2022-12-21 20:22:45', 
  'admin', '2023-04-10 15:26:51', '{}');", remove_new_menu_id,remove_menu_name,new_parent_menu_id, remove_new_order_num,remove_perms,remove_menu_name);
            writeln!(writer, "{}",  sql_insert_module)?;
            // let insert_result = pool!().exec(&sql_insert_module, vec![]).await;
            // match insert_result {
            //     Ok(_) => {println!("insert new menu remove {} ok!",remove_menu_name)},
            //     Err(_) => {println!("insert new menu remove {} failed!",remove_menu_name)},
            // }

            let export_new_menu_id=remove_new_menu_id+1;
            let export_new_order_num=remove_new_order_num+1;

            let export_perms =&format!( "{}:{}:export",module_name,menupath);
            let export_menu_name = &format!( "{}删除", menuname);

            let sql_insert_module= format!("INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query, 
  is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, 
  create_time, update_by, update_time, remark) VALUES 
        ({}, '{}', {}, {}, '#', '', '', '1', '0', 'F', '0', '0', '{}', '#', 'admin', '2022-12-21 20:22:45', 
  'admin', '2023-04-10 15:26:51', '{}');", export_new_menu_id,export_menu_name,new_parent_menu_id, export_new_order_num,export_perms,export_menu_name);
            writeln!(writer, "{}",  sql_insert_module)?;
            // let insert_result = pool!().exec(&sql_insert_module, vec![]).await;
            // match insert_result {
            //     Ok(_) => {println!("insert new menu export {} ok!",export_menu_name)},
            //     Err(_) => {println!("insert new menu export {} failed!",export_menu_name)},
            // }
        }
        return Ok(("").to_string());
        // let count = pool!()
        // .query_decode("select count(1) as count from biz_activity", vec![])
        // .await;
        
    }

    
    pub fn make_service_op_file( save_path: &str,modle_name: &str,service_file_name: &str,service_class_name: &str,service_name:&str ) -> Result<String> {
        let result="";
        let path = Path::new(save_path);
        if ! path.exists(){
            match fs::create_dir_all(&path) {
                Ok(_) => println!("table Directory created"),
                Err(e) => println!("Error creating table directory: {}", e),
            }
        }
        
        let mut path_b = PathBuf::from(save_path);
        path_b.push("NEED_MODIFY_system-service-mod.txt");
        let path = Path::new(&path_b);

    if path.exists() {
        println!("File exists.");
    } else {
        let file = File::create(path)?;
        
        let mut writer: BufWriter<File> = BufWriter::new(file);
        writeln!(writer, "{}",  "modify file src\\modules\\system\\service\\mod.rs")?;
        writeln!(writer, "{}", "add use------------")?;
        writeln!(writer, "pub use crate::modules::{}::service::{}::{};", modle_name,service_file_name,service_class_name)?;
        writeln!(writer, "{}", "into ServiceContext property------------")?;
        writeln!(writer, "pub {}: {},", service_name,service_class_name)?;
        writer.flush();

    }
    return Ok(("").to_string());
}

pub fn make_coltroller_op_file( save_path: &str,modle_name: &str, controller_file_name:&str ) -> Result<String> {
    let result="";
    let path = Path::new(save_path);
    if ! path.exists(){
        match fs::create_dir_all(&path) {
            Ok(_) => println!("table Directory created"),
            Err(e) => println!("Error creating table directory: {}", e),
        }
    }
    
    let mut path_b = PathBuf::from(save_path);
    path_b.push("NEED_MODIFY_src-http_server.txt");
    let path = Path::new(&path_b);

if path.exists() {
    println!("File exists.");
} else {
    let file = File::create(path)?;
    
    let mut writer: BufWriter<File> = BufWriter::new(file);
    writeln!(writer, "{}",  "modify file src\\http_server.rs")?;
    writeln!(writer, "{}", "add use------------")?;
    writeln!(writer, "use crate::modules::{}::controller::{{{}}};", modle_name,controller_file_name)?;
    writeln!(writer, "{}", "into ServiceContext property------------")?;
    writeln!(writer, ".service(
                web::scope(\"/{}\") //系统应
                    .service({}::page)
                    .service({}::detail)
                    .service({}::add)
                    .service({}::update)
                    .service({}::remove)
            )", modle_name,controller_file_name,controller_file_name,controller_file_name,controller_file_name,controller_file_name)?;
    writer.flush();

}
return Ok(("").to_string());
}

    pub fn make_mod_file( save_path: &str,name_into: &str, ) -> Result<String> {
        let result="";
        let path = Path::new(save_path);
        if ! path.exists(){
            match fs::create_dir_all(&path) {
                Ok(_) => println!("table Directory created"),
                Err(e) => println!("Error creating table directory: {}", e),
            }
        }
        
        let mut path_b = PathBuf::from(save_path);
        path_b.push("mod.rs");
        let path = Path::new(&path_b);

    if path.exists() {
        println!("File exists.");
    } else {
        let file = File::create(path)?;
        
        let mut writer: BufWriter<File> = BufWriter::new(file);
        writeln!(writer, "pub mod {};",  name_into)?;
        writer.flush();

    }
        

        return  Ok(String::from(result)  );
    }

    pub fn make_mod_manyfile( save_path: &str,names_vecc: &Vec<&str>) -> Result<String> {
        let result="";
        let path = Path::new(save_path);
        if ! path.exists(){
            match fs::create_dir_all(&path) {
                Ok(_) => println!("table Directory created"),
                Err(e) => println!("Error creating table directory: {}", e),
            }
        }
        
        let mut path_b = PathBuf::from(save_path);
        path_b.push("mod.rs");
        let path = Path::new(&path_b);

    if path.exists() {
        println!("File exists.");
    } else {
        let file = File::create(path)?;
        
        let mut writer: BufWriter<File> = BufWriter::new(file);
        
        for row in names_vecc {
            writeln!(writer, "pub mod {};",  row)?;
        }
        writer.flush();

    }
        return  Ok(String::from(result)  );
    }

    // 根据表名  保存路径 生成代码
    pub async fn write_domain_entities(file_path: &str ,module_name: &str,table_name: &str,columns_vecc: &Vec<TableFld>,business_name: &str,class_name: &str ) -> Result<String> {
        let result="";
        //let table_name="";


        let mut path = PathBuf::from(file_path);
        path.push("code");
        path.push("src");
        path.push("modules");
        path.push(module_name);
        path.push("domain");

        let mod_file_path =  path.as_os_str().to_str().unwrap();
        let vec = vec!["dto", "mapper", "table", "vo"];
        backcode::make_mod_manyfile(mod_file_path,&vec);

        path.push("table");
        
         
        match fs::create_dir_all(&path) {
            Ok(_) => println!("table Directory created"),
            Err(e) => println!("Error creating table directory: {}", e),
        }

        path.push("tables.rs");
        let mod_file_path =  path.parent().unwrap().as_os_str().to_str().unwrap();
        //path.push("file.txt");
        println!( "table file {}",mod_file_path);
        // 增加到 tables.rs
        Self::make_mod_file(mod_file_path,"tables");
        let file = File::create(&path)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);
        
        let head = "#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]";
        //let entity_head ="pub struct SysPost {";
        writeln!(writer, "{}",  "use rbatis::rbdc::datetime::DateTime;")?;
        writeln!(writer, "{}",  "//   在tables.rs 增加")?;
        writeln!(writer, "{}",  head)?;
        writeln!(writer, "pub struct {} {{",  class_name)?;
        for row in columns_vecc {
            //let s = format!("pub {}: Option<{}>,", row.column_name,row.column_type);
            
            //writer.write(s.as_bytes());
            writeln!(writer, "    pub {}: Option<{}>, // {}",  row.column,row.rtype,row.comment)?;
            //writer.write(b"\n");
            //writer.write_all(b"");
        }
        writeln!(writer, "}}");
        //let entity_head ="}";
        writer.flush()?; // 刷新缓冲区




        // 写 Vo 文件
        Self::write_domain_vo(module_name,&columns_vecc,&class_name,file_path);
        // 写 DTO 文件
        Self::write_domain_dto_class(module_name,&columns_vecc,table_name,&class_name,file_path);

        // 写 mapper 文件
        Self::write_domain_mapper_class(module_name,&columns_vecc,table_name,&class_name,file_path);
        // 写 service 文件
        Self::write_service_class(module_name,&columns_vecc,table_name ,business_name,class_name,file_path );
        // 写 controller 文件
        Self::write_controller_class(module_name,&columns_vecc,table_name ,business_name,class_name,file_path );

        return Ok( result.to_string());
        
    }

    // 写 controller 文件
    pub fn write_controller_class(module_name:&str,columns_vecc: &Vec<TableFld>,table_name: &str,business_name: &str,class_name: &str,file_path: &str ) -> Result<String> {
        let result="";
        //let table_name="";

        //let camel_case_table_name = to_class_case(table_name);
        //println!("{}", camel_case_table_name); // 输出: ThisIsATest
        let dto_name = format!("{}{}", class_name, "PageDTO");
        let dto_add_name = format!("{}{}", class_name, "AddDTO");
        let dto_update_name = format!("{}{}", class_name, "UpdateDTO");
        let vo_name = format!("{}{}", class_name, "Vo");

        let mut path = PathBuf::from(file_path);
        path.push("code");
        path.push("src");
        path.push("modules");
        path.push(module_name);
        path.push("controller");
        //path.push("table");

        match fs::create_dir_all(&path) {
            Ok(_) => println!("controller Directory created"),
            Err(e) => println!("Error creating controller directory: {}", e),
        }
        let controller_file_name = format!("{}_{}", class_name.to_lowercase(), "controller.rs");
        path.push(controller_file_name);
        // 写个 mod.rs
        let mod_file_path =  path.parent().unwrap().as_os_str().to_str().unwrap();
        //
        // 
        let controller_mod_name = & format!("{}_{}", class_name.to_lowercase(), "controller");
        Self::make_mod_file(mod_file_path,controller_mod_name);
        // 增加到 tables.rs
        let file = File::create(path)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);
        
        writeln!(writer, "// {}_{}",  business_name,"controller")?;

        //writeln!(writer, "{}",  "use crate::config::global_variables::STATUS_NORMAL;")?;
        //writeln!(writer, "{}",  "// permit_lib")?;
        writeln!(writer, "{}",  "use actix_web::{get, post, put, delete, web,HttpRequest, Responder};")?;
        writeln!(writer, "{}",  "use permit_lib::has_permit;")?;

        
        writeln!(writer, "{}",  "// 引用 数据表相关对象")?;
        writeln!(writer, "use crate::modules::{}::domain::table::tables::{};", module_name, class_name)?;
        writeln!(writer, "use crate::modules::{}::domain::dto::{}::{{{}, {}, {}}};",  module_name,table_name,dto_add_name,dto_update_name,dto_name)?;
        //writeln!(writer, "{}",  "// service::CONTEXT ")?;
        writeln!(writer, "{}",  "use crate::modules::system::service::{ServiceContext, CONTEXT};")?;
        writeln!(writer, "{}",  "use crate::modules::system::domain::vo::{PageVO, RespVO};")?;
        // writeln!(writer, "use crate::domain::dto::{}::{{{}, {}, {}}};",  table_name,dto_add_name,dto_update_name,dto_name)?;
        //writeln!(writer, "{}",  "use rbatis::plugin::page::Page;")?;


        writeln!(writer, "{}",  "")?;
        writeln!(writer, "/// {} controller",  business_name)?;
        let service_class_name = format!("{}{}", class_name, "Service");
        let service_param_name = format!("{}_{}", table_name.to_lowercase(), "service");
        
        //let service_param_name = service_class_name.to_lowercase();
        // 取表的主键
        let primary_key_result = TableInfo::get_primary_filed( &columns_vecc );
        let mut key_field_name=String::from("");
        match primary_key_result {
            Ok(value) => key_field_name=value,
            // 没有主键 !
            Err(e) => {println!("table {} has no primary key!",table_name)},
        }
        let primary_key_type_result = TableInfo::get_primary_filed_type( &columns_vecc );
        let mut key_field_type=String::from("");
        match primary_key_type_result {
            Ok(value) => key_field_type=value,
            // 没有主键 !
            Err(e) => {println!("table {} has no primary key!",table_name)},
        }

        writeln!(writer, "// 分页查询方法 传入{},返回 Page<{}> json",  dto_name,vo_name)?;
        writeln!(writer, "#[get(\"/{}/list\")]",  business_name)?;
        writeln!(writer, "#[has_permit(\"{}:{}:query\")]", module_name, business_name)?;
        
        writeln!(writer, "pub async fn page(query: web::Query<{}>) -> impl Responder {{", dto_name)?;
        writeln!(writer, "    let data = CONTEXT.{}.page(&query).await;",  service_param_name)?;
        writeln!(writer, "{}",  "    PageVO::from_result(&data).resp_json()")?;
        writeln!(writer, "{}",  "}")?;

        writeln!(writer, "{}","// 根据id返回一条" )?;
        writeln!(writer, "#[get(\"/{}/{{{}}}\")]", business_name, key_field_name)?;
        writeln!(writer, "#[has_permit(\"{}:{}:query\")]",module_name,business_name )?;
        writeln!(writer, "pub async fn detail({}: web::Path<{}>) -> impl Responder {{",  key_field_name,key_field_type)?;
        writeln!(writer, "    let {} = {}.into_inner();",  key_field_name,key_field_name)?;
        writeln!(writer, "    let row_vo = CONTEXT.{}.detail(&{}).await;",  service_param_name,key_field_name)?;
        writeln!(writer, "{}",  "    RespVO::from_result(&row_vo).resp_json()")?;
        writeln!(writer, "{}",  "}")?;

        writeln!(writer, "{}","// post add 一条 " )?;
        writeln!(writer, "#[post(\"/{}\")]",  business_name)?;
        writeln!(writer, "#[has_permit(\"{}:{}:add\")]",module_name,business_name )?;
        writeln!(writer, "pub async fn add(arg: web::Json<{}>) -> impl Responder {{",  dto_add_name)?;
        writeln!(writer, "    let mut data = {}::from(arg.0);",  class_name)?;
        writeln!(writer, "{}","    // 创建记录人员" )?;
        writeln!(writer, "{}",  "    data.create_by = Some(crate::web_data::get_user_name());")?;
        writeln!(writer, "{}","    // 根据需求可以使给id填业务流水号" )?;
        writeln!(writer, "    //let a = ServiceContext::get_table_maxid(\"{}\",\"{}\").await ;", key_field_name, table_name)?;
        writeln!(writer, "    //data.{}= Some(a);",  key_field_name)?;
        writeln!(writer, "    let res = {}::from(data);",  class_name)?;
        writeln!(writer, "    let data = CONTEXT.{}.add(&res).await.unwrap();",  service_param_name)?;
        writeln!(writer, "{}",  "    RespVO::<u64>::judge(data, \"\".to_string(), \"保存失败！\".to_string()).resp_json()")?;
        writeln!(writer, "{}",  "}")?;

        writeln!(writer, "{}","// put update 一条 " )?;
        writeln!(writer, "#[put(\"/{}\")]",  business_name)?;
        writeln!(writer, "#[has_permit(\"{}:{}:edit\")]",module_name,business_name )?;
        writeln!(writer, "pub async fn update(arg: web::Json<{}>) -> impl Responder {{",  dto_update_name)?;
        writeln!(writer, "    let mut data = {}::from(arg.0);",  class_name)?;
        writeln!(writer, "{}",  "    data.update_by = Some(crate::web_data::get_user_name());")?;

        writeln!(writer, "    let data = CONTEXT.{}.update(data).await.unwrap();",  service_param_name)?;
        writeln!(writer, "{}",  "    RespVO::<u64>::judge(data, \"\".to_string(), \"更新失败！\".to_string()).resp_json()")?;
        writeln!(writer, "{}",  "}")?;

        writeln!(writer, "{}","// delete 删除 一条 " )?;
        writeln!(writer, "#[delete(\"/{}/{{{}}}\")]",  business_name,key_field_name)?;
        writeln!(writer, "#[has_permit(\"{}:{}:remove\")]",module_name,business_name )?;
        writeln!(writer, "pub async fn remove({}: web::Path<String>) -> impl Responder {{",  key_field_name)?;
        writeln!(writer, "    let {} = {}.into_inner();",  key_field_name, key_field_name)?;
        writeln!(writer, "{}",  "    let data = CONTEXT")?;

        writeln!(writer, "        .{}",  service_param_name)?;
        writeln!(writer, "        .remove(&{})",  key_field_name)?;
        writeln!(writer, "{}",  "        .await.unwrap();")?;
        writeln!(writer, "{}",  "    RespVO::<u64>::judge(data, \"\".to_string(), \"删除失败！\".to_string()).resp_json()")?;
        writeln!(writer, "{}",  "}")?;

        writer.flush()?; // 刷新缓冲区
        
        return Ok( result.to_string());
        
    }
    // 写 service 文件
    pub fn write_service_class(module_name:&str,columns_vecc: &Vec<TableFld>,table_name: &str,business_name: &str,class_name: &str,file_path: &str ) -> Result<String> {
        let result="";
        //let table_name="";

        // let camel_case_table_name = to_class_case(table_name);
        // println!("{}", camel_case_table_name); // 输出: ThisIsATest
        let dto_name = format!("{}{}", class_name, "PageDTO");
        let dto_add_name = format!("{}{}", class_name, "AddDTO");
        let dto_update_name = format!("{}{}", class_name, "UpdateDTO");
        let vo_name = format!("{}{}", class_name, "Vo");

        let mut path = PathBuf::from(file_path);
        path.push("code");
        path.push("src");
        path.push("modules");
        path.push(module_name);
        path.push("service");
        //path.push("table");

        match fs::create_dir_all(&path) {
            Ok(_) => println!("service Directory created"),
            Err(e) => println!("Error creating service directory: {}", e),
        }
        let service_file_name = format!("{}_{}", class_name.to_lowercase(), "service.rs");
        path.push(service_file_name);
        //
        // 写个 mod.rs
        let mod_file_path =  path.parent().unwrap().as_os_str().to_str().unwrap();
        //
        let service_class_name =& format!("{}{}", class_name, "Service");
        let service_mod_name = & format!("{}_{}", class_name.to_lowercase(), "service");
        let service_param_name =& format!("{}_{}", table_name.to_lowercase(), "service");
        Self::make_mod_file(mod_file_path,service_mod_name);
        // 生成 system/service/mod.rs 里面的修改说明
        Self::make_service_op_file(mod_file_path,module_name,service_mod_name,service_class_name,service_param_name);
        // 增加到 tables.rs
        let file = File::create(path)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);
        

        writeln!(writer, "{}",  "use rbatis::plugin::page::PageRequest;")?;
        writeln!(writer, "{}",  "use rbatis::plugin::page::Page;")?;

        writeln!(writer, "{}",  "// 引用 数据表相关对象")?;
        writeln!(writer, "use crate::modules::{}::domain::table::tables::{};", module_name, class_name)?;
        writeln!(writer, "use crate::modules::{}::domain::dto::{}::{{{}, {}, {}}};",  module_name,table_name,dto_add_name,dto_update_name,dto_name)?;
        let vo_mod_name=vo_name.to_lowercase();
        writeln!(writer, "use crate::modules::{}::domain::vo::{}::{};",module_name,vo_mod_name, vo_name)?;
        writeln!(writer, "{}",  "use crate::modules::system::service::CONTEXT;")?;
        writeln!(writer, "{}",  "//  ")?;
        writeln!(writer, "{}",  "use crate::error::Error;")?;
        writeln!(writer, "{}",  "use crate::error::Result;")?;
        writeln!(writer, "{}",  "// pool 数据库 ")?;
        writeln!(writer, "{}",  "use crate::pool;")?;

        writeln!(writer, "/// {} service",  table_name)?;
        


        writeln!(writer, "pub struct {} {{}}",  service_class_name)?;
        writeln!(writer, "impl {} {{",  service_class_name)?;
        writeln!(writer, "    // 分页查询方法 传入{},返回 Page<{}>",  dto_name,vo_name)?;
        writeln!(writer, "    pub async fn page(&self, arg: &{}) -> Result<Page<{}>> {{", dto_name, vo_name)?;
        writeln!(writer, "{}",  "        let page_req =&PageRequest::from(arg);")?;
        writeln!(writer, "{}",  "        // 由于 使用的 postgresql 所以 limit 语句要从新按postgresql的语法注入参数")?;
        writeln!(writer, "{}",  "        let limitstr=format!(\" limit {} offset {}\", page_req.page_size,(page_req.page_no-1)*page_req.page_size);")?;
        writeln!(writer, "{}",  "        let linits=&limitstr;")?;
//        writeln!(writer, "{}",  "        dbg!(arg);")?;
        writeln!(writer, "        let data = {}::select_page(", class_name)?;

        writeln!(writer, "{}",  "            pool!(),")?;
        writeln!(writer, "{}",  "            page_req,")?;
        writeln!(writer, "{}",  "            linits,")?;
        writeln!(writer, "{}",  "            arg")?;
        writeln!(writer, "{}",  "        )")?;
        writeln!(writer, "{}",  "            .await?;")?;
        writeln!(writer, "        let page = Page::<{}>::from(data);", vo_name)?;
        writeln!(writer, "{}",  "        Ok(page)")?;
        writeln!(writer, "{}",  "    }")?;

        writeln!(writer, "    // 调用表实体对象 {} 的select_all方法 ,返回 Page<{}>", class_name, vo_name)?;
        writeln!(writer, "    pub async fn finds_all(&self) -> Result<Vec<{}>> {{",  vo_name)?;
        writeln!(writer, "        let data = {}::select_all(pool!()).await?;",  class_name)?;
        writeln!(writer, "{}",  "        let mut post_vos = vec![];")?;
        writeln!(writer, "{}",  "        for s in data {")?;
        writeln!(writer, "            post_vos.push({}::from(s));",  vo_name)?;
        writeln!(writer, "{}",  "        }")?;
        writeln!(writer, "{}",  "        Ok(post_vos)")?;
        writeln!(writer, "{}",  "    }")?;
        // 取表的主键
        let primary_key_result = TableInfo::get_primary_filed( &columns_vecc );
        let mut key_field_name=String::from("");
        match primary_key_result {
            Ok(value) => key_field_name=value,
            // 没有主键 !
            Err(e) => {println!("table {} has no primary key!",table_name)},
        }

        writeln!(writer, "    // 调用表实体对象 {} select_by_map ,根据{}返回 {}", class_name, key_field_name, vo_name)?;
        writeln!(writer, "    pub async fn detail(&self, {}: &str) -> Result<{}> {{", key_field_name, vo_name)?;
        writeln!(writer, "        let row = {}::select_by_map(pool!(), value!{{
        \"{}\":  {} }})", 
        class_name,  key_field_name,key_field_name)?;
        writeln!(writer, "{}",  "            .await?")?;
        writeln!(writer, "{}",  "            .into_iter()")?;
        writeln!(writer, "            .next().ok_or_else(|| Error::from(format!(\"不存在 {{:?}} 不存在！\", {})))?;",  key_field_name)?;
        writeln!(writer, "        let row_vo = {}::from(row);",  vo_name)?;
        writeln!(writer, "{}",  "        return Ok(row_vo);")?;
        writeln!(writer, "{}",  "    }")?;

        writeln!(writer, "    //  add 方法 , {}::insert ", class_name )?;
        writeln!(writer, "    pub async fn add(&self, arg: &{}) -> Result<u64> {{",  class_name)?;
        writeln!(writer, "        //let old = {}::select_by_map(", 
        class_name )?;
        writeln!(writer, "{}",  "            //pool!(),")?;
        writeln!(writer, "            // value!{{  ")?;
        writeln!(writer, "            //arg.{}.as_deref().unwrap_or_default(),",  key_field_name)?;
        writeln!(writer, "{}",  "        //)")?;
        writeln!(writer, "{}",  "            //.await?;")?;
        writeln!(writer, "        let result = Ok({}::insert(pool!(), &arg).await?.rows_affected);", class_name)?;
        writeln!(writer, "{}",  "        return result;")?;
        writeln!(writer, "{}",  "    }")?;

        writeln!(writer, "    //  update 方法 , {}::update_by_column ", class_name )?;
        writeln!(writer, "    pub async fn update(&self, data: {}) -> Result<u64> {{",  class_name)?;
        writeln!(writer, "        let result = {}::update_by_column(pool!(), &data, \"{}\").await;", 
        class_name ,key_field_name)?;

        writeln!(writer, "{}",  "        return Ok(result?.rows_affected);")?;
        writeln!(writer, "{}",  "    }")?;

        writeln!(writer, "    //  remove 方法 , {}::delete_by_column 并且调用sys_trash_service.add 保存到回收站表", class_name )?;
        writeln!(writer, "    pub async fn remove(&self, {}: &str) -> Result<u64> {{",  key_field_name)?;
        writeln!(writer, "        let targets = {}::select_by_map(pool!(), value!{{\"{}\":  {} }}).await?;", 
        class_name ,key_field_name, key_field_name)?;

        writeln!(writer, "        let r = {}::delete_by_map(pool!(),  value!{{\"{}\":  {} }}).await?;", 
        class_name ,key_field_name, key_field_name)?;
        
        writeln!(writer, "{}",  "        if r.rows_affected > 0 {")?;
        writeln!(writer, "{}",  "            //copy data to trash")?;
        writeln!(writer, "            CONTEXT.sys_trash_service.add(\"{}\", &targets).await?;",table_name)?;
        writeln!(writer, "{}",  "        }")?;
        writeln!(writer, "{}",  "        Ok(r.rows_affected)")?;
        writeln!(writer, "{}",  "    }")?;
        writeln!(writer, "{}",  "}")?;
        //let entity_head ="}";
        writer.flush()?; // 刷新缓冲区
        
        return Ok( result.to_string());
        
    }

    // dto 对象 在controller 传入后转为Vo调用
    pub fn write_domain_dto_class(module_name:&str, columns_vecc: &Vec<TableFld>, table_name:&str,class_name:&str,file_path: &str  ) -> Result<String> {

        let path = PathBuf::from(file_path);
        let mut path = PathBuf::from(file_path);
        path.push("code");
        path.push("src");
        path.push("modules");
        path.push(module_name);
        path.push("domain");
        path.push("dto");
        match fs::create_dir_all(&path) {
            Ok(_) => println!("dto Directory created"),
            Err(e) => println!("Error creating dto directory: {}", e),
        }

        let vo_file_name = format!("{}{}",  table_name,".rs");
        path.push(vo_file_name);
        //path.push("file.txt");
        // 写个 mod.rs
        let mod_file_path =  path.parent().unwrap().as_os_str().to_str().unwrap();
        //
        // 
        
        Self::make_mod_file(mod_file_path,table_name);
        // dto 对象
        let file = File::create(path)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);
        // dto 对象
        writeln!(writer, "use crate::modules::{}::domain::table::tables::{};", module_name, class_name)?;
        writeln!(writer, "{}",  "use rbatis::object_id::ObjectId;")?;
        writeln!(writer, "{}",  "use rbatis::snowflake::new_snowflake_id;")?;
        let head = format!("use crate::domain::table::{};",class_name );
        writeln!(writer, "{}",  "use rbatis::rbdc::datetime::DateTime;")?;
        writeln!(writer, "{}",  "use rbatis::plugin::page::PageRequest;")?;
        writeln!(writer, "{}",  "use serde::{Deserialize, Serialize};")?;

        writeln!(writer, "{}",  "    ")?;
        writeln!(writer, "{}",  "    ")?;
        writeln!(writer, "{}",  "/// page DTO")?;
        writeln!(writer, "{}",  "#[derive(Serialize, Deserialize, Clone, Debug)]")?;
        writeln!(writer, "{}",  "#[serde(rename_all = \"camelCase\")]")?;
        
        let dto_name = format!("{}{}", class_name, "PageDTO");
        writeln!(writer, "pub struct {} {{",  dto_name)?;
        writeln!(writer, "{}",  "    #[serde(rename(deserialize = \"pageNum\"))]")?;
        writeln!(writer, "{}",  "    pub page_no: Option<u64>,")?;
        writeln!(writer, "{}",  "    #[serde(rename(deserialize = \"pageSize\"))]")?;
        writeln!(writer, "{}",  "    pub page_size: Option<u64>,")?;
        for row in columns_vecc {
            //let s = format!("pub {}: Option<{}>,", row.column_name,row.column_type);
            if row.column.eq("id") || row.pk {

            }else if !row.column.eq("DateTime") && !row.rtype.eq("DateTime") {
                writeln!(writer, "    pub {}: Option<{}>,//{}",  row.column,row.rtype,row.comment)?;
            }
            //writer.write(s.as_bytes());
            
            //writer.write(b"\n");
            //writer.write_all(b"");
        }
        writeln!(writer, "}}");

        // impl from DTO to PageRequest
        writeln!(writer, "{}",  "//    实现从 dto对象 到 PageRequest")?;
        writeln!(writer, "impl From<{}> for PageRequest {{",  dto_name)?;
        writeln!(writer, "    fn from(arg: {}) -> Self {{",  dto_name)?;
        writeln!(writer, "{}",  "        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))")?;
        writeln!(writer, "{}",  "    }")?;
        writeln!(writer, "{}",  "}")?;

        // impl from &DTO to PageRequestdto_name
        writeln!(writer, "{}",  "//    实现从 &dto对象 到 PageRequest")?;
        writeln!(writer, "impl From<&{}> for PageRequest {{",  dto_name)?;
        writeln!(writer, "    fn from(arg: &{}) -> Self {{",  dto_name)?;
        writeln!(writer, "{}",  "        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))")?;
        writeln!(writer, "{}",  "    }")?;
        writeln!(writer, "{}",  "}")?;

        // 给 DTO 添加 add
        Self::write_adddto_class(&mut writer,columns_vecc,table_name,class_name);
        // 给 DTO 添加 update
        Self::write_updatedto_class(&mut writer,columns_vecc,table_name,class_name);
        //let entity_head ="}";
        writer.flush()?; // 刷新缓冲区
        return Ok( "".to_string());
    }

    pub fn write_adddto_class(writer: &mut BufWriter<File>, columns_vecc: &Vec<TableFld>, table_name:&str,class_name:&str   ) -> Result<String> {
        writeln!(writer, "{}",  "#[derive(Serialize, Deserialize, Clone, Debug)]")?;
        writeln!(writer, "{}",  "#[serde(rename_all = \"camelCase\")]")?;
        let dto_add_name = format!("{}{}", class_name, "AddDTO");
        writeln!(writer, "pub struct {} {{",  dto_add_name)?;
        for row in columns_vecc {
            //let s = format!("pub {}: Option<{}>,", row.column_name,row.column_type);
            if row.column.eq("id") || row.pk {

            }else if !row.column.to_lowercase().eq("create_by")
            && !row.column.to_lowercase().eq("create_time") 
            && !row.column.to_lowercase().eq("update_by") 
            && !row.column.to_lowercase().eq("update_time") {
                writeln!(writer, "    pub {}: Option<{}>,//{}",  row.column,row.rtype,row.comment)?;
            }

        }
        writeln!(writer, "}}");
        
// impl
        writeln!(writer, "{}",  "//    实现从 table对象 到 AddDTO对象的复制创建")?;
        let head_impl = format!("impl From<{}> for {} {{",dto_add_name,class_name );
        writeln!(writer, "{}",  head_impl)?;
        let head_impl = format!("    fn from(arg: {}) -> Self {{",dto_add_name );
        writeln!(writer, "{}",  head_impl)?;

        writeln!(writer, "{}",  "        Self {")?;
        //  let head_impl = format!("impl From<{}> for SysPostVO {{",vo_name );

    
      
        // writeln!(writer, "pub struct {} {{",  vo_name)?;
        for row in columns_vecc {
            if (row.column.eq("id") || row.pk) && row.rtype.eq("String") {
                writeln!(writer, "            {}: ObjectId::new().to_string().into(),",  row.column)?;
            }else if (row.column.eq("id") || row.pk) && (row.rtype.eq("i64") || row.rtype.eq("u64") ){
                writeln!(writer, "            {}: Some(new_snowflake_id() as i64),",  row.column)?;
            }else if !row.column.to_lowercase().eq("create_by")
            && !row.column.to_lowercase().eq("create_time") 
            && !row.column.to_lowercase().eq("update_by") 
            && !row.column.to_lowercase().eq("update_time") {
                writeln!(writer, "            {}: arg.{},",  row.column,row.column)?;
            }
        }
        let columns_len = columns_vecc.len();
        let mut column_names  = Vec::with_capacity(columns_len);
        // 循环查到的 数据  根据普通sql 直接查到的 是 Map 类型的列表
        for row in columns_vecc {
            let table_name = row.column.to_lowercase();
            // 将得到的表名 放入 上面定义的存表名的列表中
            column_names.push( table_name);
        }
        if column_names.contains(&String::from("create_by")) {
            writeln!(writer, "{}",  "            create_by: None,")?;
        }
        if column_names.contains(&String::from("create_time")) {
            writeln!(writer, "{}",  "            create_time: DateTime::now() .into(),")?;
        }
        if column_names.contains(&String::from("update_by")) {
            writeln!(writer, "{}",  "            update_by: None,")?;
        }
        if column_names.contains(&String::from("update_time")) {
            writeln!(writer, "{}",  "            update_time: None,")?;
        }
        
        writeln!(writer, "        }}");
        writeln!(writer, "    }}");
        writeln!(writer, "}}");
        return Ok( "".to_string());

    }

    pub fn write_updatedto_class(writer: &mut BufWriter<File>, columns_vecc: &Vec<TableFld>, table_name:&str,class_name:&str   ) -> Result<String> {
        writeln!(writer, "{}",  "#[derive(Serialize, Deserialize, Clone, Debug)]")?;
        writeln!(writer, "{}",  "#[serde(rename_all = \"camelCase\")]")?;
        let dto_update_name = format!("{}{}", class_name, "UpdateDTO");
        writeln!(writer, "pub struct {} {{",  dto_update_name)?;
        for row in columns_vecc {
            //let s = format!("pub {}: Option<{}>,", row.column_name,row.column_type);
            if row.column.eq("id") || row.pk {
                writeln!(writer, "    pub {}: Option<{}>,//{}",  row.column,row.rtype,row.comment)?;
                
            }else if !row.column.to_lowercase().eq("create_by")
            && !row.column.to_lowercase().eq("create_time") 
            && !row.column.to_lowercase().eq("update_by") 
            && !row.column.to_lowercase().eq("update_time") {
                writeln!(writer, "    pub {}: Option<{}>,",  row.column,row.rtype)?;
            }

        }
        writeln!(writer, "}}");
        
// impl
        writeln!(writer, "{}",  "//    实现从 table对象 到 UpdateDTO对象的复制创建")?;
        let head_impl = format!("impl From<{}> for {} {{",dto_update_name,class_name );
        writeln!(writer, "{}",  head_impl)?;
        let head_impl = format!("    fn from(arg: {}) -> Self {{",dto_update_name );
        writeln!(writer, "{}",  head_impl)?;

        writeln!(writer, "{}",  "        Self {")?;
        //  let head_impl = format!("impl From<{}> for SysPostVO {{",vo_name );

        
      
        // writeln!(writer, "pub struct {} {{",  vo_name)?;
        for row in columns_vecc {
            if row.column.eq("id") || row.pk {
                writeln!(writer, "            {}: arg.{},",  row.column,row.column)?;
            }else if !row.column.to_lowercase().eq("create_by")
            && !row.column.to_lowercase().eq("create_time") 
            && !row.column.to_lowercase().eq("update_by") 
            && !row.column.to_lowercase().eq("update_time") {
                writeln!(writer, "            {}: arg.{},",  row.column,row.column)?;
            }
        }
        let columns_len = columns_vecc.len();
        let mut column_names  = Vec::with_capacity(columns_len);
        // 循环查到的 数据  根据普通sql 直接查到的 是 Map 类型的列表
        for row in columns_vecc {
            let table_name = row.column.to_lowercase();
            // 将得到的表名 放入 上面定义的存表名的列表中
            column_names.push( table_name);
        }
        if column_names.contains(&String::from("create_by")) {
            writeln!(writer, "{}",  "            create_by: None,")?;
        }
        if column_names.contains(&String::from("create_time")) {
            writeln!(writer, "{}",  "            create_time: None,")?;
        }
        if column_names.contains(&String::from("update_by")) {
            writeln!(writer, "{}",  "            update_by: None,")?;
        }
        if column_names.contains(&String::from("update_time")) {
            writeln!(writer, "{}",  "            update_time: DateTime::now() .into(),")?;
        }
        
        writeln!(writer, "        }}");
        writeln!(writer, "    }}");
        writeln!(writer, "}}");
        return Ok( "".to_string());

    }

    pub fn write_domain_mapper_class(module_name:&str, columns_vecc: &Vec<TableFld>, table_name:&str,class_name:&str,file_path: &str  ) -> Result<String> {

        let path = PathBuf::from(file_path);
        let mut path = PathBuf::from(file_path);
        path.push("code");
        path.push("src");
        path.push("modules");
        path.push(module_name);
        path.push("domain");
        path.push("mapper");
        match fs::create_dir_all(&path) {
            Ok(_) => println!("mapper Directory created"),
            Err(e) => println!("Error creating mapper directory: {}", e),
        }

        let vo_file_name = format!("{}{}",  table_name,".rs");
        path.push(vo_file_name);
        //path.push("file.txt");
        // 写个 mod.rs
        let mod_file_path =  path.parent().unwrap().as_os_str().to_str().unwrap();
        //
        let mod_file_name = 
        Self::make_mod_file(mod_file_path,table_name);
        // 增加到 tables.rs
        let file = File::create(path)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);
        
        writeln!(writer, "use crate::modules::{}::domain::table::tables::{};",  module_name,class_name)?;
        let dto_name =& format!("{}{}", class_name, "PageDTO");
        writeln!(writer, "use crate::modules::{}::domain::dto::{}::{};",  module_name,table_name,dto_name)?;

        let head = format!("crud!({} {{}});",class_name );
        
        writeln!(writer, "{}",  head)?;
        let head = format!("impl_select_page!({}{{select_page(limit_sql:&str,dto: &{}) =>",class_name,dto_name );
        writeln!(writer, "{}",  head)?;
        
        writeln!(writer, "{}",  "    \"`where 1=1 `")?;
        let mut has_create_time=false;
        for row in columns_vecc {
            //let s = format!("pub {}: Option<{}>,", row.column_name,row.column_type);
            if row.column.eq("status") {
                writeln!(writer, "{}",  "if dto.status != '' && dto.status != None:")?;
                writeln!(writer, "{}",  "      ` and status = #{dto.status}`")?;
            }else if row.rtype.eq("String") {
                if row.length>1 {
                    let camel_case_column_name = to_camel_case(&row.column);
                    writeln!(writer, "    if dto.{} != '' && dto.{} != None:",  camel_case_column_name,camel_case_column_name)?;
                    writeln!(writer, "      ` and {} like #{{'%'+dto.{}+'%'}}`",  row.column,camel_case_column_name)?;
                }
 
            }
            if row.column.eq_ignore_ascii_case("create_time") {
                has_create_time=true;
          
            }
            //writer.write(s.as_bytes());
            
            //writer.write(b"\n");
            //writer.write_all(b"");
        }
        if has_create_time {
            writeln!(writer, "{}",  "    if !sql.contains('count'):")?;
            writeln!(writer, "{}",  "     ` order by create_time desc`\"} );")?;

        }

        return Ok( "".to_string());
    }

    pub fn write_domain_vo(module_name:&str, columns_vecc: &Vec<TableFld>,class_name:&str,file_path: &str  ) -> Result<String> {
        

        let mut path = PathBuf::from(file_path);
        path.push("code");
        path.push("src");
        path.push("modules");
        path.push(module_name);
        path.push("domain");
        path.push("vo");
        match fs::create_dir_all(&path) {
            Ok(_) => println!("vo Directory created"),
            Err(e) => println!("Error creating vo directory: {}", e),
        }
        let vo_file_name = format!("{}{}", class_name.to_lowercase(), "vo.rs");
        path.push(vo_file_name);
        //path.push("file.txt");
        // 写个 mod.rs
        let mod_file_path =  path.parent().unwrap().as_os_str().to_str().unwrap();
        //
        // 
        let vo_mod_name = & format!("{}{}", class_name.to_lowercase(), "vo");
        Self::make_mod_file(mod_file_path,vo_mod_name);
        // 增加到 tables.rs
        let file = File::create(path)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);
        writeln!(writer, "{}",  "use rbatis::rbdc::datetime::DateTime;")?;
        let head = format!("use crate::modules::{}::domain::table::tables::{};",module_name,class_name );
        
        //writeln!(writer, "{}",  "use rbatis::rbdc::datetime::DateTime;")?;
        writeln!(writer, "{}",  head)?;
        writeln!(writer, "{}",  "//  Vo 对象 可以根据需要增加交互所需的属性")?;
        writeln!(writer, "{}",  "#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]")?;
        writeln!(writer, "{}",  "#[serde(rename_all = \"camelCase\")]")?;
        

        let vo_name = format!("{}{}", class_name, "Vo");
        writeln!(writer, "pub struct {} {{",  vo_name)?;
        for row in columns_vecc {
            //let s = format!("pub {}: Option<{}>,", row.column,row.rtype);
            
            //writer.write(s.as_bytes());
            writeln!(writer, "    pub {}: Option<{}>, // {}",  row.column,row.rtype,row.comment)?;
            //writer.write(b"\n");
            //writer.write_all(b"");
        }
        writeln!(writer, "}}");

        // impl
        writeln!(writer, "{}",  "//    实现从 table对象 到 Vo对象的复制创建")?;
        let head_impl = format!("impl From<{}> for {} {{",class_name,vo_name );
        writeln!(writer, "{}",  head_impl)?;
        let head_impl = format!("    fn from(arg: {}) -> Self {{",class_name );
        writeln!(writer, "{}",  head_impl)?;

        writeln!(writer, "{}",  "        Self {")?;
        //  let head_impl = format!("impl From<{}> for SysPostVO {{",vo_name );

        
      
        // writeln!(writer, "pub struct {} {{",  vo_name)?;
        for row in columns_vecc {
            //let s = format!("pub {}: Option<{}>,", row.column_name,row.column_type);
            
            //writer.write(s.as_bytes());
            writeln!(writer, "            {}: arg.{},",  row.column,row.column)?;
            //writer.write(b"\n");
            //writer.write_all(b"");
        }
        writeln!(writer, "        }}");
        writeln!(writer, "    }}");
        writeln!(writer, "}}");
        //let entity_head ="}";
        writer.flush()?; // 刷新缓冲区
        return Ok( "".to_string());
    }

}