use crate::modules::system::domain::bo::dept::{DeptAddDTO, DeptUpdateDTO, DeptQueryDTO};
use crate::modules::system::domain::SysDept;
use crate::modules::system::domain::vo::{ RespVO};
use crate::modules::system::service::CONTEXT;
use actix_web::{get, post, put, delete, web, Responder};
use crate::config::global_variables::STATUS_NORMAL;
use crate::modules::system::domain::vo::SysDeptVO;
use crate::web_data::CurrentUser;

#[get("/dept/list")]
pub async fn page(query: web::Query<DeptQueryDTO>) -> impl Responder {
    let data = CONTEXT.sys_dept_service.all(&query).await;
    RespVO::from_result(&data).resp_json()
}


#[get("/dept/{dept_id}")]
pub async fn detail(dept_id: web::Path<String>) -> impl Responder {
    let dept_id = dept_id.into_inner();
    let dept_vo = CONTEXT.sys_dept_service.detail(&dept_id).await;
    RespVO::from_result(&dept_vo).resp_json()
}


#[post("/dept")]
pub async fn add(current_user: CurrentUser, arg: web::Json<DeptAddDTO>) -> impl Responder {
    let mut data = SysDept::from(arg.0);
    data.create_by = Some(current_user.id);
    if data.status.is_none() {
        data.status = Some(STATUS_NORMAL.to_string());
    }
    let data = CONTEXT.sys_dept_service.add(&data).await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "添加失败！".to_string()).resp_json()
}

#[put("/dept")]
pub async fn update(current_user: CurrentUser, arg: web::Json<DeptUpdateDTO>) -> impl Responder {
    let mut data = SysDept::from(arg.0);
    data.update_by = Some(current_user.id);
    let data = CONTEXT.sys_dept_service.update(data).await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "更新失败！".to_string()).resp_json()
}

#[delete("/dept/{dept_id}")]
pub async fn remove(dept_id: web::Path<String>) -> impl Responder {
    let dept_id = dept_id.into_inner();
    let data = CONTEXT
        .sys_dept_service
        .remove(&dept_id)
        .await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "删除失败！".to_string()).resp_json()
}

// 部门下拉框（对应Spring GET /system/dept/optionselect）
#[get("/dept/optionselect")]
pub async fn optionselect() -> impl Responder {
    let data = CONTEXT.sys_dept_service.all(&DeptQueryDTO { dept_name: None, status: None }).await;
    RespVO::from_result(&data).resp_json()
}

// 查询排除自身及子部门的部门列表（对应Spring GET /system/dept/list/exclude/{deptId}）
#[get("/dept/list/exclude/{dept_id}")]
pub async fn list_exclude(dept_id: web::Path<String>) -> impl Responder {
    let dept_id = dept_id.into_inner();
    let all = CONTEXT.sys_dept_service.all(&DeptQueryDTO { dept_name: None, status: None }).await.unwrap_or_default();
    let mut exclude: Vec<String> = vec![dept_id.clone()];
    let mut changed = true;
    while changed {
        changed = false;
        for d in &all {
            if let Some(pid) = &d.parent_id {
                let did = d.dept_id.clone().unwrap_or_default();
                if exclude.contains(pid) && !exclude.contains(&did) {
                    exclude.push(did);
                    changed = true;
                }
            }
        }
    }
    let data: Vec<SysDeptVO> = all.into_iter().filter(|d| !exclude.contains(&d.dept_id.clone().unwrap_or_default())).collect();
    RespVO::from_result(&Ok(data)).resp_json()
}