use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{ BufWriter, Write};
use std::path::PathBuf;

use tera::Tera;

extern crate inflector;
use inflector::cases::classcase::to_class_case;
use inflector::cases::camelcase::to_camel_case;

use crate::codegen::tableinfo::TableInfo;
use crate::pool;
use crate::error::Result;
use rbs::Value;

use super::tableinfo::TableFld;

//使用 模版引擎生产前端文件
pub struct VueInfo {}

impl VueInfo {

    pub async fn write_js_vue_file(table_name: &str,module_name: &str,file_path: &str) -> Result<String>{

        let result="";
        //let table_name="";
        let columns = TableInfo::get_pg_tables_columns(table_name).await;
        let columns_vecc = columns.ok().unwrap();

        // 取表的主键
        let primary_key_result = TableInfo::get_primary_filed( &columns_vecc );
        let mut key_field_name=String::from("");
        match primary_key_result {
            Ok(value) => key_field_name=value,
            // 没有主键 !
            Err(e) => {println!("table {} has no primary key!",table_name)},
        }
        if key_field_name.is_empty() {
            //println!("table {} has no primary key!",table_name);
            println!("table {} has no primary key! code will not generate.",table_name);
            return Err( "no primary key!".into());
        }
        let camel_case_table_name =& to_class_case(table_name);
        println!("{}", camel_case_table_name); // 输出: ThisIsATest
        let table_web_url_name = &camel_case_table_name.to_lowercase();
        let pkColumn = &to_camel_case(&key_field_name);
        let mut path = PathBuf::from(file_path);
        path.push("code");
        path.push("vue");
        path.push("api");
        path.push(&module_name.to_lowercase());

        match fs::create_dir_all(&path) {
            Ok(_) => println!("table Directory created"),
            Err(e) => println!("Error creating table directory: {}", e),
        }
        let js_name = format!("{}{}", table_web_url_name, ".js");
        path.push(js_name);
        let vo_name = format!("{}{}", camel_case_table_name, "Vo");

        // 初始化 Tera 实例并加载模板
        let mut tera = Tera::new("templates/**/*.tera").unwrap();
        
        // Prepare the context with some data
        let mut context = tera::Context::new();

        context.insert("businessName", &camel_case_table_name);
        context.insert("moduleName", &module_name.to_lowercase());
        context.insert("pkColumn", pkColumn);

        let rendered = tera.render("api.js.tera", &context).unwrap();
        // 增加到 tables.rs
        let file = File::create(path)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);
        writer.write_all(rendered.as_bytes());
        //println!("{}", rendered);
        return Ok( result.to_string());

    }

    // 写 vue 文件
    pub async fn write_vue(columns_vecc: &Vec<TableFld>,module_name: &str,table_name: &str,file_path: &str ) -> Result<String> {
        let result="";
        //let table_name="";
        let columns = TableInfo::get_pg_tables_columns(table_name).await;
        let columns_vecc = columns.ok().unwrap();

        // 取表的主键
        let primary_key_result = TableInfo::get_primary_filed( &columns_vecc );
        let mut key_field_name=String::from("");
        match primary_key_result {
            Ok(value) => key_field_name=value,
            // 没有主键 !
            Err(e) => {println!("table {} has no primary key!",table_name)},
        }
        if key_field_name.is_empty() {
            //println!("table {} has no primary key!",table_name);
            println!("table {} has no primary key! code will not generate.",table_name);
            return Err( "no primary key!".into());
        }
        let camel_case_table_name =& to_class_case(table_name);
        println!("{}", camel_case_table_name); // 输出: ThisIsATest
        let table_web_url_name = &camel_case_table_name.to_lowercase();
        let pk_column = &to_camel_case(&key_field_name);
        let mut path = PathBuf::from(file_path);
        path.push("code");
        path.push("vue");
        path.push("views");
        path.push(&module_name.to_lowercase());
        path.push(&camel_case_table_name.to_lowercase());

        match fs::create_dir_all(&path) {
            Ok(_) => println!("table Directory created"),
            Err(e) => println!("Error creating table directory: {}", e),
        }
        let vuefile_name = "index.vue";
        path.push(vuefile_name);
        let vo_name = format!("{}{}", camel_case_table_name, "Vo");
        let mut vec_flds_query: Vec<TableFld> = Vec::with_capacity(columns_vecc.len());

        // 将 查询 对应表单单独建一个字段列表
        for row in &columns_vecc {
            if row.column.eq("status") {
                let mut new_fld_query = row.clone();
                new_fld_query.dicttype=String::from("sys_normal_disable");// "sys_normal_disable".to_owned();
                new_fld_query.htmltype=String::from("select");//"select".to_owned();
            }else{
                vec_flds_query.push(row.clone());
            }
        }


        // 初始化 Tera 实例并加载模板
        let mut tera = Tera::new("templates/**/*.tera").unwrap();
        
        // Prepare the context with some data
        let mut context = tera::Context::new();

        context.insert("businessName", &camel_case_table_name);
        context.insert("moduleName", &module_name.to_lowercase());
        context.insert("pkColumn", pk_column);
        context.insert("querycolumns", &vec_flds_query);
        context.insert("columns", &columns_vecc);


        let rendered = tera.render("index.vue.tera", &context).unwrap();
        // 增加到 tables.rs
        let file = File::create(path)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);
        writer.write_all(rendered.as_bytes());

        return Ok( result.to_string());
    }
}
