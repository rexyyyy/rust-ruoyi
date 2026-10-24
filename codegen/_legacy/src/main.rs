
use codegen::codegen::backcode::backcode;
use codegen::codegen::CONTEXT;
use codegen::codegen::tableinfo::TableInfo ;
use codegen::util::idseq::{self, IdSeq};

#[allow(unused_variables)]
#[allow(unused_doc_comments)]
/// use tokio,because Rbatis specifies the runtime-tokio
#[tokio::main]
async fn main(){

    //连接数据库
    CONTEXT.lock().unwrap().init_pool().await;
    let table_info = TableInfo{};
    //TableInfo::get_pg_tables() .await;
    // let table_name  = "sys_menu";
    // let module_name  = "system";
    // let function_name  = "菜单";

    // 
    let module_name  = "mall";
    let module_name_cn  = "商城";
    let table_name  = "oms_regorder";
    let function_name  = "注册申请";
    let path_save  = "d:/code";

    let menu_path = module_name;

    let sub_menu_cn="注册申请管理"; //  二级菜单 汉字



    let pathResult = TableInfo::generate_code(module_name,path_save,table_name,function_name).await;

    
    // 模块如果存在,判断是否新建
    // 模块 mall 中文名商城 菜单名 mall， 
    // mall菜单曾加二级菜单 订单管理 订单管理 英文名 order
    // 最终生成  sql  用户决定是否运行该 sql
    // 
    let _ = backcode::make_new_module_menu(path_save,module_name,module_name_cn,&menu_path,sub_menu_cn ,table_name).await;



    //println!("生成完毕{}",pathResult.unwrap_or_default());
    // 测试生成业务流水号 函数
    //let new_id = backcode::get_table_maxid("id","oms_order").await;
    // 测试生成定制雪花算法 函数
    /* 
    let mut idnew = IdSeq::next_id();
    for i in 0..200 {
        dbg!(&idnew);
        idnew = IdSeq::next_id();
    }
    */

}

