use std::time::Duration;
use actix_web::{get, HttpRequest, post, put, Responder, web};
use actix_multipart::form::{tempfile::TempFile, MultipartForm};

use crate::modules::system::domain::vo::{RespJson, RespVO};
use crate::modules::system::service::CONTEXT;
use crate::modules::system::domain::bo::user::{PasswordUpdateDTO, UserUpdateDTO};
use crate::util::password_encoder::PasswordEncoder;

/*
* 用户自身的操作
*/

//的用户信息
#[get("/user/profile")]
pub async fn profile(req: HttpRequest) -> impl Responder {
    let user_cahce = match CONTEXT.sys_user_service.get_user_cache(&req).await {
        Ok(u) => u,
        Err(e) => {
            // 登录态失效（强退 / Redis 缓存缺失）：必须返回 401 让前端重新登录，
            // 不能 unwrap() —— 否则 actix worker panic，连接被无响应关闭。
            return RespVO::<String>::from_error_info(401, &format!("Unauthorized for:{}", e)).resp_json();
        }
    };
    let mut res = RespJson::success_info("操作成功");
    // RuoYi-Vue3（plus-ui）从 data 中取复合对象：data.user / data.roleGroup / data.postGroup
    let post_group = CONTEXT
        .sys_user_service
        .finds_post_names_by_user_id(&user_cahce.id.to_string())
        .await
        .unwrap_or_default()
        .join(",");
    let data = serde_json::json!({
        "user": user_cahce.user.unwrap(),
        "roleGroup": user_cahce.roles.clone().into_iter()
            .map(|r| r.role_name.unwrap_or_default())
            .collect::<Vec<_>>()
            .join(","),
        "postGroup": post_group
    });
    res.insert("data".to_string(), data);
    return res.resp_json();
}

//用户自行修改用户信息
#[put("/user/profile")]
pub async fn update_profile(req: HttpRequest, mut arg: web::Json<UserUpdateDTO>) -> impl Responder {
    let mut user_cache = match CONTEXT.sys_user_service.get_user_cache(&req).await {
        Ok(u) => u,
        Err(e) => {
            return RespVO::<String>::from_error_info(401, &format!("Unauthorized for:{}", e)).resp_json();
        }
    };
    let clone = arg.0.clone();
    arg.0.user_id = Some(user_cache.id.to_string());
    let vo = CONTEXT.sys_user_service.update(arg.0).await.unwrap();
    if vo > 0 {
        let mut user = user_cache.user.clone().unwrap();
        user.phonenumber = clone.phonenumber;
        user.email = clone.email;
        user_cache.user = user.into();
        CONTEXT.cache_service.set_string_ex(
            &user_cache.token_key,
            &user_cache.to_string(),
            Some(Duration::from_secs(
                CONTEXT.config.token_expired_min * 60
            )),
        ).await;
    }
    return RespVO::from_result(&Ok(vo)).resp_json();
}

//用户自行修改密码
#[put("/user/profile/updatePwd")]
pub async fn update_pwd(req: HttpRequest, arg: web::Query<PasswordUpdateDTO>) -> impl Responder {
    let user_cache = match CONTEXT.sys_user_service.get_user_cache(&req).await {
        Ok(u) => u,
        Err(e) => {
            return RespVO::<String>::from_error_info(401, &format!("Unauthorized for:{}", e)).resp_json();
        }
    };
    let user = match CONTEXT.sys_user_service.find(&user_cache.id.to_string()).await {
        Ok(Some(u)) => u,
        Ok(None) => return RespVO::<String>::from_error_info(500, "用户不存在").resp_json(),
        Err(e) => return RespVO::<String>::from_error_info(500, &e.to_string()).resp_json(),
    };
    let new_password = &arg.new_password;
    let old_password = &arg.old_password;
    if new_password.eq(old_password) {
        return RespVO::<u64>::from_error_info(500, "新密码不能与旧密码相同").resp_json();
    }


    if !PasswordEncoder::verify(user.password.as_ref().unwrap(), &old_password.clone().unwrap()) {
        return RespVO::<u64>::from_error_info(500, "修改密码失败，旧密码错误").resp_json();
    }
    let vo = CONTEXT.sys_user_service.update_password(
        &PasswordEncoder::encode(new_password.as_ref().unwrap()), &user_cache.id.to_string()).await;
    return RespVO::from_result(&vo).resp_json();
}

// 用户头像上传（对应Spring POST /system/user/profile/avatar）
#[post("/user/profile/avatar")]
pub async fn avatar(req: HttpRequest, MultipartForm(form): MultipartForm<AvatarUploadForm>) -> impl Responder {
    let user_cache = CONTEXT.sys_user_service.get_user_cache(&req).await;
    if user_cache.is_err() {
        return RespVO::<u64>::from_error_info(401, "请重新登录！").resp_json();
    }
    let user_cache = user_cache.unwrap();
    let doc_file = &form.avatarfile;
    let mime = doc_file.content_type.clone().unwrap_or(actix_web::mime::APPLICATION_OCTET_STREAM);
    let mime_str = mime.essence_str().to_string();
    let extension = match mime_str.as_str() {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/gif" => "gif",
        "image/bmp" => "bmp",
        _ => "jpg",
    };
    if std::fs::create_dir_all("./uploads/avatar").is_err() {
        return RespVO::<u64>::from_error_info(500, "创建上传目录失败").resp_json();
    }
    let file_id = crate::util::object_id::new_object_id();
    let unique_name = format!("{}.{}", file_id, extension);
    let file_path = format!("./uploads/avatar/{}", unique_name);
    if std::fs::copy(doc_file.file.path(), &file_path).is_err() {
        return RespVO::<u64>::from_error_info(500, "保存头像文件失败").resp_json();
    }
    let img_url = format!("/profile/avatar/{}", unique_name);
    let _ = CONTEXT.sys_user_service.update_avatar(&user_cache.id.to_string(), &img_url).await;
    let mut res = RespJson::success();
    // RuoYi-Vue3（plus-ui）从 data 中取：data.imgUrl
    res.insert("data".to_string(), serde_json::json!({ "imgUrl": img_url }));
    res.resp_json()
}

// 头像静态文件访问（对应Spring /profile/** 静态资源）
pub async fn serve_avatar(path: web::Path<String>) -> actix_web::Result<actix_files::NamedFile> {
    let file_name = path.into_inner();
    // 防目录穿越：头像文件名由 object_id + 扩展名生成，只应含 [A-Za-z0-9._-]，
    // 禁止任何路径分隔符 / 上跳序列，防止读取 /uploads/avatar 之外的任意文件。
    let safe = !file_name.is_empty()
        && file_name.len() <= 128
        && !file_name.contains("..")
        && file_name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-');
    if !safe {
        return Err(actix_web::error::ErrorNotFound("invalid file name"));
    }
    Ok(actix_files::NamedFile::open(format!("./uploads/avatar/{}", file_name))?)
}

#[derive(Debug, MultipartForm)]
struct AvatarUploadForm {
    #[multipart(limit = "10MB")]
    avatarfile: TempFile,
}