use actix_web::{delete, get, HttpRequest, post, put, Responder, web};

use crate::config::global_variables::{ADMIN_NAME, DEL_FLAG_NORMAL, STATUS_NORMAL};
use crate::modules::system::domain::bo::user::{ UserAddDTO, UserPageDTO, UserRoleAuthQueryDTO, UserUpdateDTO};
use crate::modules::system::domain::SysUser;
use crate::modules::system::domain::vo::{ PageVO, RespJson, RespVO};
use crate::modules::system::service::CONTEXT;
use futures_util::future::BoxFuture;
use crate::web_data::CurrentUser;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use crate::modules::system::domain::bo::user::UserIdsQueryDTO;

#[get("/user/list")]
pub async fn page(arg: web::Query<UserPageDTO>) -> impl Responder {
    let vo = CONTEXT.sys_user_service.page(&arg.0).await;
    return PageVO::from_result(&vo).resp_json();
}

//前端打开用户添加时，查询岗位和角色列表
#[get("/user/")]
pub async fn before_add(arg: web::Query<UserPageDTO>) -> impl Responder {
    let mut res = RespJson::success_info("操作成功");
    // RuoYi-Vue3（plus-ui）从 data 中取复合对象：data.posts / data.roles
    let data = serde_json::json!({
        "roles": CONTEXT.sys_role_service.finds_all().await.unwrap(),
        "posts": CONTEXT.sys_post_service.finds_all().await.unwrap()
    });
    res.insert("data".to_string(), data);
    res.resp_json()
}

#[post("/user")]
pub async fn add(current_user: CurrentUser, arg: web::Json<UserAddDTO>) -> impl Responder {
    let role_ids = arg.role_ids.clone().unwrap();
    let mut data = SysUser::from(arg.0);
    let user_id = data.user_id.clone();
    data.create_by = Some(current_user.id);
    let rows_affected = CONTEXT.sys_user_service.add(data).await.unwrap();
    if rows_affected > 0 {
        if role_ids.len() > 0 {
            CONTEXT.sys_user_role_service.add_user_roles(&user_id.to_string(), &role_ids);
        }
        //todo 增加post
    }
    CONTEXT.sys_operlog_service.record("用户管理", 1, "POST /system/user", &current_user.name, "/system/user", if rows_affected > 0 { 0 } else { 1 }, if rows_affected > 0 { "" } else { "新增用户失败" }).await;

    return RespVO::from_result(&Ok(rows_affected)).resp_json();
}


//用户编辑，需要查询post和role列表
#[get("/user/{user_id}")]
pub async fn detail(user_id: web::Path<String>) -> impl Responder {
    let user_id = user_id.into_inner();
    // 防御：id 非数字或用户不存在时不能 panic（会把整个 worker 连接打断），返回标准错误响应
    let user = match CONTEXT.sys_user_service.detail(&user_id).await {
        Ok(u) => u,
        Err(e) => {
            return RespVO::<u64>::from_error_info(500, &format!("查询用户失败: {}", e)).resp_json();
        }
    };

    let mut res = RespJson::success_info("操作成功");

    // plus-ui 编辑对话框（handleUpdate）会读 data.user.roles 并与 data.roles 合并去重，
    // 不回填会被序列化为 null 导致 "... is not iterable" 报错
    let roles = CONTEXT.sys_role_service.finds_roles_by_user_id(&user_id).await.unwrap();
    let role_ids: Vec<String> = roles.iter().map(|r| r.role_id.clone().unwrap()).collect();
    let mut user = user;
    user.roles = Some(roles);

    let post_ids = CONTEXT.sys_user_service.finds_post_ids_by_user_id(&user_id).await.unwrap_or_default();

    // RuoYi-Vue3（plus-ui）从 data 中取复合对象：data.user / data.posts / data.roles / data.postIds / data.roleIds
    let data = serde_json::json!({
        "user": user,
        "posts": CONTEXT.sys_post_service.finds_all().await.unwrap(),
        "roles": CONTEXT.sys_role_service.finds_all().await.unwrap(),
        "postIds": post_ids,
        "roleIds": role_ids
    });
    res.insert("data".to_string(), data);

    return res.resp_json();
}

#[put("/user")]
pub async fn update(current_user: CurrentUser, arg: web::Json<UserUpdateDTO>) -> impl Responder {
    let vo = CONTEXT.sys_user_service.update(arg.0).await;
    let ok = vo.is_ok();
    CONTEXT.sys_operlog_service.record("用户管理", 2, "PUT /system/user", &current_user.name, "/system/user", if ok { 0 } else { 1 }, if ok { "" } else { "修改用户失败" }).await;
    return RespVO::from_result(&vo).resp_json();
}

//删除用户（对应Spring DELETE /system/user/{userIds}）
#[delete("/user/{user_ids}")]
pub async fn remove(current_user: CurrentUser, user_ids: web::Path<String>) -> impl Responder {
    let user_ids = user_ids.into_inner();
    // 支持批量删除（逗号分隔）
    let ids: Vec<String> = user_ids.split(",").map(|s| s.to_string()).collect();
    let mut affected = 0u64;
    for id in ids {
        let vo = CONTEXT.sys_user_service.remove(&id).await;
        match vo {
            Ok(n) => affected += n,
            Err(e) => return RespVO::from_error_result(500, &Err::<u64, _>(e)).resp_json(),
        }
    }
    CONTEXT.sys_operlog_service.record("用户管理", 3, "DELETE /system/user/{userIds}", &current_user.name, "/system/user", if affected > 0 { 0 } else { 1 }, "").await;
    return RespVO::from_result(&Ok(affected)).resp_json();
}


#[get("/user/deptTree")]
pub async fn get_dept_tree(current_user: CurrentUser) -> impl Responder {
    let dept_tree = CONTEXT.sys_dept_service.get_dept_tree(&current_user.name).await;
    return RespVO::from_result(&dept_tree).resp_json();
}

#[put("/user/authRole")]
pub async fn set_auth_roles(arg: web::Query<UserRoleAuthQueryDTO>) -> impl Responder {
    let s = arg.role_ids.clone().unwrap_or_default();
    let role_ids: Vec<String> = s.split(",").map(|s| s.to_string()).collect();
    CONTEXT.sys_user_role_service.reset_through_user_id(&arg.user_id.clone().unwrap_or_default(), &role_ids).await;
    RespVO::<u64>::from_success_info("更新成功！").resp_json()
}

#[get("/user/authRole/{user_id}")]
pub async fn get_auth_roles(current_user: CurrentUser, user_id: web::Path<String>) -> impl Responder {
    let user_id = user_id.into_inner();
    let mut user = CONTEXT.sys_user_service.detail(&user_id).await.unwrap();
    user.roles = Some(CONTEXT.sys_role_service.finds_roles_by_user_id(&user_id).await.unwrap());
    let roles = CONTEXT.sys_role_service.finds_roles_by_user_id(&user_id).await.unwrap();
    let filter_roles = match current_user.name.eq(ADMIN_NAME) {
        true => {
            roles
        }
        false => {
            roles.into_iter().filter(|r| r.admin).collect::<Vec<_>>()
        }
    };

    let mut res = RespJson::success_info("操作成功");
    // RuoYi-Vue3（plus-ui）从 data 中取复合对象：data.user / data.roles
    let data = serde_json::json!({
        "user": user,
        "roles": filter_roles
    });
    res.insert("data".to_string(), data);
    return res.resp_json();
}


// AjaxResult ajax = AjaxResult.success();
// SysUser user = userService.selectUserById(userId);
// List<SysRole> roles = roleService.selectRolesByUserId(userId);
// ajax.put("user", user);
// ajax.put("roles", SysUser.isAdmin(userId) ? roles : roles.stream().filter(r -> !r.isAdmin()).collect(Collectors.toList()));
// return ajax;
// }


#[put("/user/resetPwd")]
pub async fn reset_pwd(arg: web::Json<UserUpdateDTO>) -> impl Responder {
    let user_id = arg.0.user_id.unwrap();
    let user = CONTEXT.sys_user_service.find(&user_id).await.unwrap();
    if user.is_none() {
        return RespVO::<u64>::from_error_info(500, "找不到此用户").resp_json();
    }
    let user = user.unwrap();
    if user.user_name.eq(&Some(ADMIN_NAME.to_string())) {
        return RespVO::<u64>::from_error_info(500, "不允许操作超级管理员用户").resp_json();
    }
    //todo 查看datascope  userService.checkUserDataScope(user.getUserId());

    //重置密码（默认密码已在service层处理）
    let password = arg.0.password.clone().unwrap_or_default();
    // bcrypt 哈希下沉到阻塞线程池（默认 cost 下单次 200ms+ CPU）
    let encoded = crate::util::password_encoder::PasswordEncoder::encode_async(&password).await;
    let vo = CONTEXT.sys_user_service.update_password(&encoded, &user_id).await;
    RespVO::from_result(&vo).resp_json()
}

//获取当前登录用户信息（对应Spring /system/user/getInfo）
// 逻辑提取为内部函数，供 /user/getInfo 与 RuoYi 标准前端顶层别名 /getInfo 复用
pub(crate) async fn build_get_info(req: HttpRequest) -> impl Responder {
    let user_cache = CONTEXT.sys_user_service.get_user_cache(&req).await;
    if user_cache.is_err() {
        return RespVO::<u64>::from_error_info(401, "请重新登录！").resp_json();
    }
    let user_cache = user_cache.unwrap();
    // plus-ui 5.x user store 消费 res.data.{user,roles,permissions}（R<InfoVo> 包 data 层）
    let mut res = RespJson::success();
    let mut data = serde_json::Map::new();
    data.insert("user".to_string(), serde_json::json!(&user_cache.user));
    data.insert(
        "roles".to_string(),
        serde_json::json!(user_cache.roles.iter().map(|r| r.role_key.clone()).collect::<Vec<_>>()),
    );
    data.insert("permissions".to_string(), serde_json::json!(&user_cache.permissions));
    res.insert("data".to_string(), serde_json::Value::Object(data));
    res.resp_json()
}

#[get("/user/getInfo")]
pub async fn get_info(req: HttpRequest) -> impl Responder {
    build_get_info(req).await
}

// RuoYi 标准前端（ruoyi-vue3）顶层路径别名：/dev-api/getInfo
#[get("/getInfo")]
pub async fn get_info_alias(req: HttpRequest) -> impl Responder {
    build_get_info(req).await
}

//更改用户当前状态
#[put("/user/changeStatus")]
pub async fn change_status(req: HttpRequest, arg: web::Json<UserUpdateDTO>) -> impl Responder {
    let user_id = arg.0.user_id.unwrap();
    let user = CONTEXT.sys_user_service.find(&user_id).await.unwrap();
    if user.is_none() {
        return RespVO::<u64>::from_error_info(500, "找不到此用户").resp_json();
    }
    let user = user.unwrap();
    if user.user_name.eq(&Some(ADMIN_NAME.to_string())) {
        return RespVO::<u64>::from_error_info(500, "不允许操作超级管理员用户").resp_json();
    }
    let vo = CONTEXT.sys_user_service.update_status(
        arg.0.status.as_deref().unwrap_or_default(), &user_id).await;
    return RespVO::from_result(&vo).resp_json();
}

// 根据用户ID列表查询用户（对应Spring GET /system/user/optionselect）
#[get("/user/optionselect")]
pub async fn optionselect(user_ids: web::Query<UserIdsQueryDTO>) -> impl Responder {
    let ids: Vec<String> = user_ids.user_ids.clone().unwrap_or_default()
        .split(",").filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
    let data = CONTEXT.sys_user_service.finds_by_ids(&ids).await;
    RespVO::from_result(&data).resp_json()
}

// 根据部门ID查询用户（对应Spring GET /system/user/list/dept/{deptId}）
#[get("/user/list/dept/{dept_id}")]
pub async fn list_by_dept(dept_id: web::Path<String>) -> impl Responder {
    let dept_id = dept_id.into_inner();
    let data = CONTEXT.sys_user_service.finds_by_dept_id(&dept_id).await;
    RespVO::from_result(&data).resp_json()
}

// 数据导出（对应Spring POST /user/export）——分批拉取，避免全量进内存
#[post("/user/export")]
pub async fn export(query: web::Query<UserPageDTO>) -> impl Responder {
    let base = query.0;
    let headers = ["用户编号", "用户名称", "用户昵称", "手机号码", "状态", "创建时间"];
    let fetch = |page_no: u64| -> BoxFuture<'static, Result<Vec<Vec<String>>, String>> {
        let mut dto = base.clone();
        dto.page_no = Some(page_no);
        dto.page_size = Some(500);
        Box::pin(async move {
            let pg = CONTEXT.sys_user_service.page(&dto).await.map_err(|e| e.to_string())?;
            Ok(pg.records.into_iter().map(|user| vec![
                user.user_id.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                user.user_name.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                user.nick_name.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                user.phonenumber.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                user.status.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                user.create_time.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            ]).collect())
        })
    };
    match crate::util::excel::build_xlsx_streaming("导出数据", &headers, 500, fetch).await {
        Ok(bytes) => crate::util::excel::excel_response(bytes, "user.xlsx"),
        Err(e) => RespVO::<String>::from_error_info(500, &e).resp_json(),
    }
}


// 用户导入模板（对应Spring POST /system/user/importTemplate）
#[post("/user/importTemplate")]
pub async fn import_template() -> impl Responder {
    let headers = ["用户名称", "用户昵称", "手机号码", "用户邮箱", "用户性别", "岗位编号", "角色标识"];
    let body: Vec<Vec<String>> = vec![];
    match crate::util::excel::build_xlsx("用户数据", &headers, &body) {
        Ok(bytes) => crate::util::excel::excel_response(bytes, "user_import_template.xlsx"),
        Err(e) => RespVO::<String>::from_error_info(500, &e).resp_json(),
    }
}

// 用户导入（对应Spring POST /system/user/importData，multipart字段file）
#[derive(Debug, MultipartForm)]
struct ImportUserForm {
    #[multipart(limit = "10MB")]
    file: TempFile,
}

#[post("/user/importData")]
pub async fn import_data(current_user: CurrentUser, MultipartForm(mut form): MultipartForm<ImportUserForm>) -> impl Responder {
    let mut buf = Vec::new();
    if let Err(e) = std::io::Read::read_to_end(&mut form.file.file, &mut buf) {
        return RespVO::<String>::from_error_info(500, &e.to_string()).resp_json();
    }
    let rows = match crate::util::excel::parse_xlsx(&buf) {
        Ok(r) => r,
        Err(e) => return RespVO::<String>::from_error_info(500, &e).resp_json(),
    };
    let init_password = crate::get_config_value!("sys.user.initPassword");
    let mut success_num = 0usize;
    let mut failure_num = 0usize;
    let mut failure_list: Vec<String> = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        if i == 0 { continue; } // 跳过表头
        if row.is_empty() || row[0].trim().is_empty() { continue; }
        let user_name = row[0].trim().to_string();
        let nick_name = row.get(1).cloned().unwrap_or_default().trim().to_string();
        let phonenumber = row.get(2).cloned().unwrap_or_default().trim().to_string();
        if CONTEXT.sys_user_service.find_by_account(&user_name).await.unwrap_or(None).is_some() {
            failure_num += 1;
            failure_list.push(format!("{}: 用户已存在", user_name));
            continue;
        }
        let user = SysUser {
            user_id: crate::util::idseq::next_id(),
            dept_id: Some(103),
            user_name: Some(user_name.clone()),
            nick_name: Some(nick_name),
            email: row.get(3).cloned().unwrap_or_default().trim().to_string().into(),
            phonenumber: Some(phonenumber),
            sex: Some("0".to_string()),
            avatar: None,
            password: Some(crate::util::password_encoder::PasswordEncoder::encode_async(&init_password).await),
            status: Some(STATUS_NORMAL.to_string()),
            del_flag: Some(DEL_FLAG_NORMAL.to_string()),
            login_ip: None,
            login_date: None,
            create_by: Some(current_user.id),
            create_time: Some(chrono::Local::now().naive_local()),
            update_by: None,
            update_time: None,
            remark: None,
        };
        match CONTEXT.sys_user_service.add(user).await {
            Ok(n) if n > 0 => success_num += 1,
            _ => { failure_num += 1; failure_list.push(format!("{}: 导入失败", user_name)); }
        }
    }
    let mut res = RespJson::success_info("导入完成");
    res.insert("data".to_string(), serde_json::json!({
        "successNum": success_num,
        "failureNum": failure_num,
        "successList": [],
        "failureList": failure_list,
    }));
    res.resp_json()
}
