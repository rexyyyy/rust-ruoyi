use std::fs::{self, File};
use std::io::{ BufWriter, Write};
use std::path::PathBuf;

use tera::Tera;

extern crate inflector;

use crate::error::Result;

use super::tableinfo::TableFld;

//使用 tera模版引擎生产前端文件
pub struct frontcode {}
//后台代码直接输出
//前台代码使用tera模板引擎输出
impl frontcode {

    pub async fn write_js_vue_file( file_path: &str,module_name: &str,table_name: &str,function_name: &str,business_name: &str,clazz_name: &str,columns_vecc: &Vec<TableFld>,pk_property: &str) -> Result<String>{

        let result="";

        let mut path = PathBuf::from(file_path);
        path.push("code");
        path.push("vue");
        path.push("api");
        path.push(&module_name.to_lowercase());

        match fs::create_dir_all(&path) {
            Ok(_) => println!("api js Directory created"),
            Err(e) => println!("Error creating api js directory: {}", e),
        }
        let js_name = format!("{}{}", business_name, ".js");
        path.push(js_name);
        let vo_name = format!("{}{}", clazz_name, "Vo");

        // 初始化 Tera 实例并加载模板
        let tera = Tera::new("templates/**/*.tera").unwrap();
        
        // Prepare the context with some data
        let mut context = tera::Context::new();
        context.insert("functionName", function_name);
        context.insert("businessName", business_name);
        context.insert("clazzName", clazz_name);
        context.insert("moduleName", module_name);
        context.insert("pkColumn", pk_property);

        let rendered = tera.render("api.js.tera", &context).unwrap();
        // 增加到 tables.rs
        let file = File::create(path)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);
        writer.write_all(rendered.as_bytes());
        //println!("{}", rendered);

        Self::write_vue(&columns_vecc,function_name,module_name,table_name,business_name ,clazz_name ,pk_property,file_path).await?;
        return Ok( result.to_string());

    }

    // 写 vue 文件
    pub async fn write_vue(columns_vecc: &Vec<TableFld>,function_name: &str, module_name: &str,table_name: &str,business_name: &str,clazz_name: &str,pk_property: &str,file_path: &str ) -> Result<String> {
        let result="";

        let mut path = PathBuf::from(file_path);
        path.push("code");
        path.push("vue");
        path.push("views");
        path.push(module_name.to_lowercase());
        path.push(&business_name.to_lowercase());
        println!("create {}",path.as_path().to_str().unwrap());
        match fs::create_dir_all(&path) {
            Ok(_) => println!("vue Directory created"),
            Err(e) => println!("Error creating vue directory: {}", e),
        }
        let vuefile_name = "index.vue";
        path.push(vuefile_name);
        let vo_name = format!("{}{}", clazz_name, "Vo");
        let mut vec_flds_query: Vec<TableFld> = Vec::with_capacity(columns_vecc.len());

        // 将 查询 对应表单单独建一个字段列表
        for row in columns_vecc {
            if row.column.eq("status") {
                let mut new_fld_query = row.clone();
                new_fld_query.dicttype=String::from("sys_normal_disable");// "sys_normal_disable".to_owned();
                new_fld_query.htmltype=String::from("select");//"select".to_owned();
            }else if row.column.eq("id") || row.pk {

            }else if !row.column.eq("DateTime") {
                vec_flds_query.push(row.clone());
            }else{
                // vec_flds_query.push(row.clone());
            }
        }
        // 用的时候 如: const { sys_show_hide, sys_normal_disable } = proxy.useDict('sys_show_hide', 'sys_normal_disable' );
        let dicts = "'sys_normal_disable'";


        // 初始化 Tera 实例并加载模板
        let tera = Tera::new("templates/**/*.tera").unwrap();
        
        // Prepare the context with some data
        let mut context = tera::Context::new();
        
        context.insert("dicts", dicts);
        context.insert("functionName", function_name);
        context.insert("businessName", &business_name);
        context.insert("clazzName", clazz_name);
        context.insert("moduleName", module_name  );
        context.insert("jsfileName",business_name);
        context.insert("pkColumn", pk_property);
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
