
use std::fs;
use std::path::PathBuf;

extern crate inflector;
use inflector::cases::classcase::to_class_case;
use inflector::cases::camelcase::to_camel_case;

use crate::pool;
use crate::error::Result;
use rbs::Value;

use super::backcode::backcode;
use super::frontcode::frontcode;

// 表的字段描述
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TableFld{
    pub tablename :String,  //表名
    pub column :String,     //列名
    pub prop :String,       //属性名 驼峰命名 小写开头
    pub dbtype :String,     //数据库中类型
    pub rtype :String,      //rust中类型
    pub length : u32 ,      //长度
    pub dig : u64 ,      //小数长度
    pub nullable : bool ,   //是否可为null true  false
    pub pk : bool ,         //主键 true  false
    pub comment :String,    //字段说明
    pub dicttype :String,   //字典类型 没有则为空
    pub htmltype :String,   //html类型 没有则为空
    pub querytype :String,   //html类型 没有则为空
}

// 表名 信息
pub struct TableInfo {}

impl TableInfo {

    pub async fn generate_code(module_name: &str,path_save: &str,table_name: &str,function_name: &str) -> Result<String> {
        let result="";
        // 获取表的列信息
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
        // 表没有主键 不能运行
        if key_field_name.is_empty() {
            //println!("table {} has no primary key!",table_name);
            println!("table {} has no primary key! code will not generate.",table_name);
            return Err( "no primary key!".into());
        }
        let mut backcode_path = PathBuf::from(path_save);
        backcode_path.push("code");
        backcode_path.push("src");
        backcode_path.push("modules");
        backcode_path.push(module_name);
        match fs::create_dir_all(&backcode_path) {
            Ok(_) => println!("{} Directory created",module_name),
            Err(e) => println!("Error creating directory: {} {}",module_name, e),
        }

        let mod_file_path =  backcode_path.as_os_str().to_str().unwrap();
        let vec = vec!["controller", "domain", "service"];
        backcode::make_mod_manyfile(mod_file_path,&vec);

        // 根据表名取类名
        let clazz_name = to_class_case(table_name);
        // println!("{}", clazz_name); // 输出: ThisIsATest
        // 根据表名取业务功能名
        let business_name = & Self::get_business_name(table_name).unwrap_or("".to_string());
        // 主键字段转换为驼峰命名的属性名
        let pk_property = to_camel_case(&key_field_name);
        //生成后台代码
        let _ = backcode::write_domain_entities(path_save,module_name,table_name, &columns_vecc,business_name,&clazz_name).await;
        //生成前台代码
        let _ = frontcode::write_js_vue_file( path_save,module_name,table_name,function_name,business_name ,&clazz_name ,&columns_vecc ,&pk_property).await;
        
        let mut path = PathBuf::from(path_save);
        path.push("code");
        return Ok( path.as_os_str().to_str().unwrap_or_default().to_string() );
    }

    pub fn get_business_name(table_name: &str) -> Result<String> {
        let mut name2 =String::from(table_name);
        let idx = table_name.find("_").unwrap_or(9999);
        if idx>0 && idx<9999{
            let slice = &table_name[idx+1..];
            name2=slice.to_string();
        }
        if !name2.is_empty() {
            name2 = to_class_case(&name2).to_lowercase();
        }
        
        // let idx = table_name.find("-").unwrap_or(9999);
        // if idx>0 && idx<9999{
        //     let slice = &table_name[idx+1..];
        //     name2=slice.to_string();
        // }
        if name2.is_empty() {
            return  Err( "".into());
        }else{
            return Ok( name2);
        }
    }

    pub fn get_primary_filed( columns_vecc: &Vec<TableFld> ) -> Result<String> {
        let mut key_field =String::from("");
        for row in columns_vecc {
            if  row.pk {
                key_field = row.column.clone();
            }
        }
        if key_field.is_empty() {
            return  Err( "".into());
        }else{
            return Ok( key_field);
        }
    }

    pub fn get_primary_filed_type( columns_vecc: &Vec<TableFld> ) -> Result<String> {
        let mut key_field =String::from("");
        for row in columns_vecc {
            if  row.pk {
                key_field = row.rtype.clone();
            }
        }
        if key_field.is_empty() {
            return  Err( "".into());
        }else{
            return Ok( key_field);
        }
    }

    // 所有表名 信息
    pub async fn get_pg_tables( ) -> Result<Vec<String>> {
        // postgresql 查询数据库里面的表  table_schema=public
        let sql = "SELECT \"table_name\" FROM information_schema.tables where table_type='BASE TABLE' AND table_schema='public'"; // 只选择name字段
        // 查询得到 rbs::Value 类型的 列表
        let result_string = pool!().query_decode::<Vec<Value>>(sql,vec![]).await.unwrap() ; 
        // 新建一个 存 表名称的 列表
        let capnames_strs = result_string.len();
        let mut vecnames_strs  = Vec::with_capacity(capnames_strs);
        // 循环查到的 数据  根据普通sql 直接查到的 是 Map 类型的列表
        for item in result_string.iter() {
            // 取一条的map 对象,  该mao 的key 是 table_name value是表名称
            let value_map = item.as_map().unwrap();
            let result= &value_map["table_name"];
            let table_name = result.as_string().unwrap();
            // 将得到的表名 放入 上面定义的存表名的列表中
            vecnames_strs.push( table_name);

            // match item{
            //     rbs::Value::Map(value_map ) => {
            //         let result= &value_map["table_name"];
            //         let table_name = result.as_string().unwrap();
            //         vecnames_strs.push( table_name);
                    
            //     }
            //     _ =>{

            //     }
            // }
        }
        dbg!("vecnames_strs: ",&vecnames_strs );
        return Ok( vecnames_strs);
        
    }
    // 表的所有字段 信息
    pub async fn get_pg_tables_columns(table_name: &str)-> Result<Vec<TableFld>> {
        // postgresql 查询数据库里面的表字段信息
        // 2026.1.19  根据反馈 有些表没有写注释
        let sql2=" SELECT pc.relname AS table_name,     pa.attname AS column_name, info.udt_name AS column_type,  info.data_type AS column_type2, 
  info.character_maximum_length AS column_length2, info.numeric_precision AS column_length,  info.numeric_scale AS column_dig, pd.description AS column_description ,  pa.attnum,pd.objsubid,
  pa.attnotnull AS is_nullable, ( CASE WHEN ( SELECT COUNT(*) FROM pg_constraint WHERE conrelid = pa.attrelid AND conkey[1]= attnum AND contype = 'p' ) > 0 THEN  TRUE ELSE FALSE      END ) 
    AS is_primary from pg_class pc
    left  join pg_attribute pa on pa.attrelid= pc.oid
    left  join INFORMATION_SCHEMA.COLUMNS info on info.table_name=pc.relname
    left OUTER join pg_description pd on pd.objoid = pa.attrelid
    WHERE  pc.relname = '{}' AND info.column_name=pa.attname
    ORDER BY pa.attnum ASC".to_string();
        //简单替换 以表名为参数
        let sql = sql2.replace("{}", table_name);
        println!("get_pg_tables_columns: \n {}", &sql );
        dbg!("get_pg_tables_columns SQL: ",&sql );
        let result_string = pool!().query_decode::<Vec<Value>>(&sql,vec![]).await.unwrap() ; 
        // 新建一个 存   列表
        let capnames_strs = result_string.len();
        let mut vec_flds  = Vec::with_capacity(capnames_strs);
        
        // 循环查到的 数据  根据普通sql 直接查到的 是 Map 类型的列表
        let mut index =0;
        for item in result_string.iter() {
            // 取一条的map 对象,  该mao 的key 是 table_name value是表名称
            let value_map = item.as_map().unwrap();
            // dbg!( &value_map );
            let table_name = &value_map["table_name"].as_string().unwrap();
            let column_name = &value_map["column_name"].as_string().unwrap();
            let column_db_type = &value_map["column_type"].as_string().unwrap();
            let is_null_len = value_map["column_length"].is_null();
            let mut  column_length = 0;
            if is_null_len {

            }else{
                column_length = value_map["column_length"].as_i64().unwrap();
            }
            index += 1;
            let is_nullable = &value_map["is_nullable"].as_bool().unwrap();
            let is_primary = &value_map["is_primary"].as_bool().unwrap();
            let mut column_description = value_map["column_description"].as_string().unwrap_or_default();
            if column_description.is_empty() {
                //println!("table {} column {} has no description!",table_name,column_name);
                let no_comment_name = format!("列{}", index);
                column_description = no_comment_name;
            }

            let camel_case_prop_name = &to_camel_case(column_name);
            
            let nullable = *is_nullable as bool;
            let primary = *is_primary as bool;
            if column_db_type.eq_ignore_ascii_case("bpchar") 
            || column_db_type.eq_ignore_ascii_case("character")
            || column_db_type.eq_ignore_ascii_case("varchar")
            || column_db_type.eq_ignore_ascii_case("varchar2"){
                let is_null_len = value_map["column_length2"].is_null();
                if is_null_len {

                }else{
                    column_length = value_map["column_length2"].as_i64().unwrap();
                }
            }
            // 小数
            let mut numberic_dig_len =0;
            if column_db_type.eq_ignore_ascii_case("numeric") {
                //column_length = value_map["column_length"].as_i64().unwrap();
                let is_null_len = value_map["column_dig"].is_null();
                if is_null_len {

                }else{
                    numberic_dig_len = value_map["column_dig"].as_u64().unwrap();
                }

                //numberic_dig_len= value_map["column_dig"].as_u64().unwrap();
            }
            let len = column_length as u32;

            let column_type  = Self::get_type_from_db_type(column_db_type,&len).unwrap();
            let table_fld = TableFld{ 
                tablename: table_name.to_string(), 
                column:column_name.to_string(),  
                dbtype: column_db_type.to_string(),  
                rtype: column_type.to_string(), 
                length: len, 
                dig: numberic_dig_len, 
                nullable: nullable, 
                pk: primary, 
                comment: column_description.to_string(),
                dicttype: "".to_owned(),
                htmltype: "".to_owned(),
                querytype: "".to_owned(),
                prop: camel_case_prop_name.to_string(),
            };
            vec_flds.push(table_fld);

        }
        //  dbg!("vec_flds: ",&vec_flds );
        return Ok( vec_flds);

    }

    pub fn get_type_from_db_type( column_db_type: &str,dig:&u32 ) -> Result<String> {
        
        let index =  column_db_type.find("int8").unwrap_or(1000);
        if index>=0 && index<1000 {
            return Ok( "i64".to_string());
        }
        let index =  column_db_type.find("int4").unwrap_or(1000);
        if index>=0 && index<1000 {
            return Ok( "i32".to_string());
        }
        let index =  column_db_type.find("int").unwrap_or(1000);
        if index>=0 && index<1000 {
            return Ok( "i32".to_string());
        }
        let index =  column_db_type.find("float8").unwrap_or(1000);
        if index>=0 && index<1000 {
            return Ok( "f64".to_string());
        }
        let index =  column_db_type.find("float4").unwrap_or(1000);
        if index>=0 && index<1000 {
            return Ok( "f32".to_string());
        }
        // numeric 定义 一般用于 小数  金额等
        let index =  column_db_type.find("numeric").unwrap_or(1000);
        if index>=0 && index<1000 && *dig<=0 {
            return Ok( "i64".to_string());
        }else if index>=0 && index<1000 && *dig>0 {
            return Ok( "f64".to_string());
        }

        let index =  column_db_type.find("timestamp").unwrap_or(1000);
        if index>=0 && index<1000 {
            return Ok( "DateTime".to_string());
        }
        let index =  column_db_type.find("date").unwrap_or(1000);
        if index>=0 && index<1000 {
            return Ok( "DateTime".to_string());
        }
        let index =  column_db_type.find("varchar").unwrap_or(1000);
        if index>=0 && index<1000 {
            return Ok( "String".to_string());
        }
        let index =  column_db_type.find("text").unwrap_or(1000);
        if index>=0 && index<1000 {
            return Ok( "String".to_string());
        }
        let index =  column_db_type.find("uuid").unwrap_or(1000);
        if index>=0 && index<1000 {
            return Ok( "String".to_string());
        }
        let index =  column_db_type.find("char").unwrap_or(1000);
        if index>=0 && index<1000 {
            return Ok( "char".to_string());
        }

        //let result="";
        return Ok( "".to_string());
    }

}
