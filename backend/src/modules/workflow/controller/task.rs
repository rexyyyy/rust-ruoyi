//! 任务接口 `/workflow/task/**`（待办 / 已办 / 发起 / 办理）

use actix_web::{get, post, web, Responder};

use crate::modules::system::domain::vo::{PageVO, RespVO};
use crate::modules::workflow::domain::bo::{
    CompleteTaskBo, RevokeBo, StartRrocessBo, TaskOperationBo, TaskPageBo, TerminationBo,
};
use crate::modules::workflow::service::task;
use crate::web_data::CurrentUser;

/// 我的待办
#[get("/task/todoList")]
pub async fn todo_list(query: web::Query<TaskPageBo>, user: CurrentUser) -> impl Responder {
    let data = task::page_todo(user.id, &query).await;
    PageVO::from_result(&data).resp_json()
}

/// 我的已办
#[get("/task/doneList")]
pub async fn done_list(query: web::Query<TaskPageBo>, user: CurrentUser) -> impl Responder {
    let data = task::page_done(user.id, &query).await;
    PageVO::from_result(&data).resp_json()
}

/// 全部待办（管理员）
#[get("/task/allTodoList")]
pub async fn all_todo_list(query: web::Query<TaskPageBo>) -> impl Responder {
    let data = task::page_all_todo(&query).await;
    PageVO::from_result(&data).resp_json()
}

/// 全部已办（管理员）
#[get("/task/allDoneList")]
pub async fn all_done_list(query: web::Query<TaskPageBo>) -> impl Responder {
    let data = task::page_all_done(&query).await;
    PageVO::from_result(&data).resp_json()
}

/// 发起流程
#[post("/task/startWorkFlow")]
pub async fn start_work_flow(arg: web::Json<StartRrocessBo>, user: CurrentUser) -> impl Responder {
    let data = task::start(user.id, &arg.0).await;
    RespVO::from_result(&data).resp_json()
}

/// 办理任务（通过 / 退回）
#[post("/task/completeTask")]
pub async fn complete_task(arg: web::Json<CompleteTaskBo>, user: CurrentUser) -> impl Responder {
    let data = task::complete(user.id, &arg.0).await;
    RespVO::from_result(&data).resp_json()
}

/// 终止流程
#[post("/task/terminationTask")]
pub async fn termination_task(arg: web::Json<TerminationBo>, user: CurrentUser) -> impl Responder {
    let data = task::termination(user.id, &arg.0).await;
    RespVO::from_result(&data).resp_json()
}

/// 撤销流程
#[post("/task/revokeProcess")]
pub async fn revoke_process(arg: web::Json<RevokeBo>, user: CurrentUser) -> impl Responder {
    let instance_id = match arg.instance_id {
        Some(id) => id,
        None => {
            return RespVO::<String>::from_error_info(500, "流程实例 id 不能为空").resp_json();
        }
    };
    let data = task::revoke(user.id, instance_id, arg.message.as_deref()).await;
    RespVO::from_result(&data).resp_json()
}

/// 转办 / 委派 / 加签 / 减签：`{op}` 取 transfer / depute / addSignature / reductionSignature
#[post("/task/taskOperation/{op}")]
pub async fn task_operation(
    path: web::Path<String>,
    arg: web::Json<TaskOperationBo>,
    user: CurrentUser,
) -> impl Responder {
    let data = task::operate(user.id, &path.into_inner(), &arg.0).await;
    RespVO::from_result(&data).resp_json()
}

/// 取回已办任务
#[post("/task/taskBack/{task_id}")]
pub async fn task_back(path: web::Path<String>, user: CurrentUser) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => task::task_back(user.id, id).await,
        Err(_) => Err(crate::error::Error::from("任务 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

/// 实例的办理历史（详情页时间线）
#[get("/task/flowHisTaskList/{instance_id}")]
pub async fn his_by_instance(path: web::Path<String>) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => task::his_by_instance(id).await,
        Err(_) => Err(crate::error::Error::from("流程实例 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

/// 任务的办理人
#[get("/task/taskUserList/{task_id}")]
pub async fn task_user_list(path: web::Path<String>) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => task::task_users(id).await,
        Err(_) => Err(crate::error::Error::from("任务 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(todo_list)
        .service(done_list)
        .service(all_todo_list)
        .service(all_done_list)
        .service(start_work_flow)
        .service(complete_task)
        .service(termination_task)
        .service(revoke_process)
        .service(task_operation)
        .service(task_back)
        .service(his_by_instance)
        .service(task_user_list);
}
