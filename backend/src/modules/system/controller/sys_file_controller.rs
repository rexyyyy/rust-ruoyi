use std::{
    fs::{create_dir_all, File},
    io::Write,
};

use crate::modules::system::domain::vo::{RespJson, RespVO};
use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_multipart::Multipart;
use actix_web::{post, Responder};
use async_std::stream::StreamExt;
use pinyin::Pinyin;
use std::path::PathBuf;
// 定义多部分表单结构
#[derive(MultipartForm)]
#[multipart(duplicate_field = "replace")]
struct DocumentUploadForm {
    name: Text<String>,
    category: Text<String>,
    #[multipart(limit = "2 KiB")]
    description: Text<String>,
    tags: Text<String>,
    // 文件字段，限制为50MB
    #[multipart(limit = "50MB")]
    document: TempFile,
}

const UPLOAD_DIR: &str = "./uploads/documents";
// 使用 actix_multipart::Multipart 处理文件上传 以及传入其他参数
// 处理上传的端点
#[post("/uploads")]
async fn upload_document(mut payload: Multipart) -> impl Responder {
    let create_path_result = create_dir_all(UPLOAD_DIR);
    if create_path_result.is_err() {
        let err_msg = format!("创建上传目录失败: {}", create_path_result.err().unwrap());
        return RespVO::<u64>::from_error_info(500, err_msg.as_str()).resp_json();
    }
    let mut res = RespJson::success();
    // 遍历多部分表单字段
    while let Some(field_result) = payload.next().await {
        let mut field: actix_multipart::Field = match field_result {
            Ok(f) => f,
            Err(e) => continue,
        };

        if field.name() == Some("document") {
            let headers = field.headers();
            //dbg!(&headers);

            // 获取内容处置头部
            let content_disposition = field.content_disposition();

            // 获取文件名
            let original_file_name = match content_disposition {
                Some(name) => name.get_filename(),
                None => continue, // 跳过非文件字段
            };
            let original_file_name = original_file_name.unwrap_or_default();

            let filename_pinyin: Vec<&str> =
                pinyin::to_pinyin_vec(original_file_name, Pinyin::plain);
            let mut original_file_name2 = String::new();
            for s in &filename_pinyin {
                original_file_name2.push_str(s);
            }

            //dbg!(&original_file_name2);

            let mime_type = match field.content_type() {
                Some(name) => name.to_string(),
                None => continue, // 跳过非文件字段
            };
            // 验证文件类型：仅允许白名单类型（get_extension 未命中返回 "bin"）
            let extension = get_extension(&mime_type);
            if extension == "bin" {
                return RespVO::<u64>::from_error_info(400, "不支持的文件类型").resp_json();
            }
            let file_path = PathBuf::from(UPLOAD_DIR);

            let file_id = crate::util::object_id::new_object_id(); //Uuid::new_v4().to_string();
            let unique_name = format!("{}.{}", file_id, extension);
            let file_path = format!("{}/{}", UPLOAD_DIR, unique_name);

            res.insert("file_id".to_string(), file_id.into());
            res.insert("file_path".to_string(), file_path.clone().into());
            // 保存文件内容并获取文件大小
            let file_size = match save_file(&mut field, &file_path).await {
                Ok(size) => size,
                Err(e) => {
                    let err_msg = format!("保存文件失败: {}", e);
                    return RespVO::<u64>::from_error_info(500, err_msg.as_str()).resp_json();
                }
            };
        } else {
            // 读取字段内容
            let mut field_content = Vec::new();
            while let Some(chunk) = field.next().await {
                match chunk {
                    Ok(data) => field_content.extend_from_slice(&data),
                    Err(e) => continue,
                }
            }
            // 文本字段处理（不再把内容打印到日志，避免敏感信息落盘）
            let _text_content = String::from_utf8_lossy(&field_content).to_string();
        }
    }

    res.insert("message".to_string(), "文件上传成功".into());

    return res.resp_json(); 
}

// 处理上传的端点
#[post("/uploadfrm")]
async fn upload_form(MultipartForm(form): MultipartForm<DocumentUploadForm>) -> impl Responder {
    let response = format!(
        "Title: {}, Category: {}, description: {}",
        form.name.0, form.category.0, form.description.0,
    );

    
    let doc_file = form.document;
    let sizd: usize = doc_file.size;
    let create_path_result = create_dir_all(UPLOAD_DIR);
    if create_path_result.is_err() {
        let err_msg = format!("创建上传目录失败: {}", create_path_result.err().unwrap());
        return RespVO::<u64>::from_error_info(500, err_msg.as_str()).resp_json();
    }
    let mime = doc_file
        .content_type
        .unwrap_or(actix_web::mime::APPLICATION_OCTET_STREAM);
    let mime_str = mime.essence_str();

    let allowed_types = vec![
        "application/pdf",
        "image/jpeg",
        "image/png",
        "image/gif",
        // "application/msword",
        // "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        // "application/vnd.ms-excel",
        // "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        // "application/vnd.ms-powerpoint",
        // "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    ];

    if !allowed_types.contains(&mime_str) {
        let err_msg = format!("不支持的文件类型: {}", mime_str);
        return RespVO::<u64>::from_error_info(400, err_msg.as_str()).resp_json();
    }

    let original_file_name = doc_file.file_name.unwrap_or_default();
    let extension = get_extension(mime_str);
    // 生成唯一文件名和路径

    let file_id = crate::util::object_id::new_object_id(); //Uuid::new_v4().to_string();
    let unique_name = format!("{}.{}", file_id, extension);
    let file_path = format!("{}/{}", UPLOAD_DIR, unique_name);
    let relative_path = format!("/uploads/{}", unique_name);

    // 处理上传的文件
    let file_info = response.as_str();

    // 使用 Tokio 在后台线程执行持久化操作，避免阻塞 async 运行时

    // let mut res = RespJson::success();
    //         res.insert("fileId".to_string(), file_id.into());
    //         res.insert("filePath".to_string(), relative_path.into());
    //         res.insert("fileInfo".to_string(), file_info.into());
    //         return res.resp_json();
    // 这里可以添加文件保存逻辑
    let persist_result = doc_file.file.persist(&file_path);
    match persist_result {
        Ok(_) => {
            let mut res = RespJson::success();
            res.insert("fileId".to_string(), file_id.into());
            res.insert("filePath".to_string(), relative_path.into());
            res.insert("fileInfo".to_string(), file_info.into());
            return res.resp_json();
        }
        Err(err) => {
            let err_msg = format!("保存文件失败: {}", err);
            return RespVO::<u64>::from_error_info(500, err_msg.as_str()).resp_json();
        }
    }


}


/// 保存上传的文件并返回文件大小
async fn save_file(field: &mut actix_multipart::Field, path: &str) -> std::io::Result<u64> {
    let mut file = File::create(path)?;
    let mut total_size = 0;

    // 处理每个数据块
    while let Some(chunk_result) = field.next().await {
        // 处理可能的 MultipartError
        let chunk = chunk_result.map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("读取数据块失败: {}", e))
        })?;

        // 写入文件并更新大小
        file.write_all(&chunk)?;
        total_size += chunk.len() as u64;
    }

    file.flush()?;
    Ok(total_size)
}

/// 根据 MIME 类型获取文件扩展名
fn get_extension(mime_type: &str) -> &str {
    match mime_type {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/gif" => "gif",
        "application/pdf" => "pdf",
        _ => "bin", // 不会发生（前面已验证）
    }
}
