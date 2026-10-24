//! job 模块请求 DTO（字段命名对齐 RuoYi 前端 camelCase）。
use serde::Deserialize;

/// 任务分页查询（参数名对齐 RuoYi：`pageNum` / `pageSize`，其余 camelCase）。
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobPageDTO {
    #[serde(rename(deserialize = "pageNum"), default)]
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub job_name: Option<String>,
    pub job_group: Option<String>,
    pub status: Option<String>,
    pub invoke_target: Option<String>,
}

/// 执行日志分页查询。
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobLogPageDTO {
    #[serde(rename(deserialize = "pageNum"), default)]
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub job_name: Option<String>,
    pub status: Option<String>,
}

/// 新增任务。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobAddDTO {
    pub job_name: Option<String>,
    pub job_group: Option<String>,
    pub invoke_target: Option<String>,
    pub cron_expression: Option<String>,
    pub misfire_policy: Option<String>,
    pub concurrent: Option<String>,
    pub status: Option<String>,
    pub create_by: Option<String>,
    pub remark: Option<String>,
}

/// 修改任务（必须带 `jobId`）。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobUpdateDTO {
    pub job_id: i64,
    pub job_name: Option<String>,
    pub job_group: Option<String>,
    pub invoke_target: Option<String>,
    pub cron_expression: Option<String>,
    pub misfire_policy: Option<String>,
    pub concurrent: Option<String>,
    pub status: Option<String>,
    pub update_by: Option<String>,
    pub remark: Option<String>,
}

/// 暂停 / 恢复（changeStatus）。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatusDTO {
    pub job_id: i64,
    pub status: String,
}

/// 立即执行一次（run）。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobRunDTO {
    pub job_id: i64,
}
