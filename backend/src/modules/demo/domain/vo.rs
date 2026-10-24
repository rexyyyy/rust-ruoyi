//! DemoCourse 返回对象（VO）
//! 源表：demo_course　业务：演示课程表
//! 由 codegen 于 2026-10-08 21:03:18 生成。
//!
//! VO 与前端 TS 类型（`types.ts`）一一对应：`rename_all = "camelCase"`
//! 保证 JSON 字段名就是 Vue 侧的属性名。

use serde::{Deserialize, Serialize};

use crate::modules::demo::domain::entity::DemoCourse;

#[allow(non_snake_case)] // VO 字段即前端 camelCase 属性名，有意为之
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DemoCourseVO {
    // 主键统一输出成字符串：PG bigint 的精度超过 JS Number 安全范围，
    // 前端直接塞进 URL / 表单若经过 JSON 数字会有精度丢失风险。
    pub courseId: Option<String>,
    pub courseCode: Option<String>, // 课程编码
    pub courseName: Option<String>, // 课程名称
    pub teacher: Option<String>, // 授课教师
    pub credit: Option<f64>, // 学分
    pub openStatus: Option<String>, // 开课状态
    pub startDate: Option<chrono::NaiveDate>, // 开课日期
    pub remark: Option<String>, // 备注
    pub createBy: Option<i64>, // 创建者
    pub createTime: Option<chrono::NaiveDateTime>, // 创建时间
    pub updateBy: Option<i64>, // 更新者
    pub updateTime: Option<chrono::NaiveDateTime>, // 更新时间
}

impl From<DemoCourse> for DemoCourseVO {
    fn from(arg: DemoCourse) -> Self {
        Self {
            courseId: Some(arg.course_id.to_string()),
            courseCode: arg.course_code,
            courseName: arg.course_name,
            teacher: arg.teacher,
            credit: arg.credit,
            openStatus: arg.open_status,
            startDate: arg.start_date,
            remark: arg.remark,
            createBy: arg.create_by,
            createTime: arg.create_time,
            updateBy: arg.update_by,
            updateTime: arg.update_time,
        }
    }
}
