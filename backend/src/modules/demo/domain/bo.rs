//! DemoCourse 请求对象（DTO）
//! 源表：demo_course　业务：演示课程表
//! 由 codegen 于 2026-10-08 21:03:18 生成。

use serde::{Deserialize, Serialize};

use crate::modules::system::domain::PageRequest;
use crate::modules::demo::domain::entity::DemoCourse;

/// 分页查询参数
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DemoCoursePageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    pub course_code: Option<String>, // 课程编码
    pub course_name: Option<String>, // 课程名称
}

impl From<DemoCoursePageDTO> for PageRequest {
    fn from(arg: DemoCoursePageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

impl From<&DemoCoursePageDTO> for PageRequest {
    fn from(arg: &DemoCoursePageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

/// 新增参数
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DemoCourseAddDTO {
    pub course_code: Option<String>, // 课程编码
    pub course_name: Option<String>, // 课程名称
    pub teacher: Option<String>, // 授课教师
    pub credit: Option<f64>, // 学分
    pub open_status: Option<String>, // 开课状态
    pub start_date: Option<chrono::NaiveDate>, // 开课日期
    pub remark: Option<String>, // 备注
}

impl From<DemoCourseAddDTO> for DemoCourse {
    fn from(arg: DemoCourseAddDTO) -> Self {
        Self {
            course_id: crate::util::idseq::next_id(),
            course_code: arg.course_code,
            course_name: arg.course_name,
            teacher: arg.teacher,
            credit: arg.credit,
            open_status: arg.open_status,
            start_date: arg.start_date,
            remark: arg.remark,
            create_by: None,
            create_time: Some(chrono::Local::now().naive_local()),
            update_by: None,
            update_time: None,
        }
    }
}

/// 兼容前端回传的雪花 ID：字符串 / 整数 / 浮点（JS Number）/ null 四种 JSON 形态均可
fn string_or_number<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum V {
        S(String),
        I(i64),
        F(f64),
    }
    match Option::<V>::deserialize(deserializer)? {
        None => Ok(None),
        Some(V::S(s)) => s.parse().map(Some).map_err(serde::de::Error::custom),
        Some(V::I(i)) => Ok(Some(i)),
        Some(V::F(f)) => Ok(Some(f as i64)),
    }
}

/// 修改参数
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DemoCourseUpdateDTO {
    // VO 主键输出为 String（防 JS 精度丢失），前端会原样回传，
    // 因此主键同时接受字符串与数字两种 JSON 形态
    #[serde(deserialize_with = "string_or_number")]
    pub course_id: Option<i64>,
    pub course_code: Option<String>, // 课程编码
    pub course_name: Option<String>, // 课程名称
    pub teacher: Option<String>, // 授课教师
    pub credit: Option<f64>, // 学分
    pub open_status: Option<String>, // 开课状态
    pub start_date: Option<chrono::NaiveDate>, // 开课日期
    pub remark: Option<String>, // 备注
}

impl From<DemoCourseUpdateDTO> for DemoCourse {
    fn from(arg: DemoCourseUpdateDTO) -> Self {
        Self {
            course_id: arg.course_id.unwrap_or_default(),
            course_code: arg.course_code,
            course_name: arg.course_name,
            teacher: arg.teacher,
            credit: arg.credit,
            open_status: arg.open_status,
            start_date: arg.start_date,
            remark: arg.remark,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: Some(chrono::Local::now().naive_local()),
        }
    }
}
