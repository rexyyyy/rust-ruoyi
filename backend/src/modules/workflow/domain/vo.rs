//! 工作流出参（VO）。
//!
//! 约定：所有主键输出为 String —— 雪花 id 是 19 位，超出 JS 安全整数范围，
//! 直接给数字会被前端精度截断，回传时就找不到了。

use serde::{Deserialize, Serialize};
use warm_flow_core::entity::{Definition, HisTask, Instance, Task};

use super::entity::{FlowCategory, TestLeave};

// ======================================================================
// 流程分类
// ======================================================================

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CategoryVO {
    pub category_id: String,
    pub parent_id: String,
    pub ancestors: Option<String>,
    pub category_name: String,
    pub order_num: Option<i32>,
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 树形接口用
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<CategoryVO>,
}

impl From<FlowCategory> for CategoryVO {
    fn from(arg: FlowCategory) -> Self {
        Self {
            category_id: arg.category_id.to_string(),
            parent_id: arg.parent_id.to_string(),
            ancestors: arg.ancestors,
            category_name: arg.category_name,
            order_num: arg.order_num,
            create_time: arg.create_time,
            children: Vec::new(),
        }
    }
}

// ======================================================================
// 流程定义
// ======================================================================

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionVO {
    pub id: String,
    pub flow_code: String,
    pub flow_name: String,
    pub model_value: String,
    pub category: Option<String>,
    /// 分类名称（由分类表补齐，便于列表直接展示）
    pub category_name: Option<String>,
    pub version: String,
    /// 0 未发布 1 已发布 9 已失效
    pub is_publish: i32,
    /// 1 激活 0 挂起
    pub activity_status: i32,
    pub form_custom: Option<String>,
    pub form_path: Option<String>,
    pub create_by: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<chrono::NaiveDateTime>,
}

impl From<Definition> for DefinitionVO {
    fn from(arg: Definition) -> Self {
        Self {
            id: arg.id.unwrap_or_default().to_string(),
            flow_code: arg.flow_code,
            flow_name: arg.flow_name,
            model_value: arg.model_value,
            category: arg.category,
            category_name: None,
            version: arg.version,
            is_publish: arg.is_publish,
            activity_status: arg.activity_status,
            form_custom: arg.form_custom,
            form_path: arg.form_path,
            create_by: arg.create_by,
            create_time: arg.create_time,
            update_by: arg.update_by,
            update_time: arg.update_time,
        }
    }
}

// ======================================================================
// 流程实例
// ======================================================================

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstanceVO {
    pub id: String,
    pub definition_id: String,
    pub flow_name: Option<String>,
    pub flow_code: Option<String>,
    pub business_id: String,
    /// 0 开始 1 中间 2 结束 3 互斥 4 并行 5 包容
    pub node_type: i32,
    pub node_code: String,
    pub node_name: Option<String>,
    pub flow_status: String,
    pub activity_status: i32,
    pub create_by: Option<String>,
    /// 发起人昵称（引擎里 create_by 存的是用户 id，列表页要给人看名字）
    pub create_by_name: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_time: Option<chrono::NaiveDateTime>,
}

impl From<Instance> for InstanceVO {
    fn from(arg: Instance) -> Self {
        Self {
            id: arg.id.unwrap_or_default().to_string(),
            definition_id: arg.definition_id.to_string(),
            flow_name: None,
            flow_code: None,
            business_id: arg.business_id,
            node_type: arg.node_type,
            node_code: arg.node_code,
            node_name: arg.node_name,
            flow_status: arg.flow_status,
            activity_status: arg.activity_status,
            create_by: arg.create_by,
            create_by_name: None,
            create_time: arg.create_time,
            update_time: arg.update_time,
        }
    }
}

// ======================================================================
// 待办任务
// ======================================================================

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TaskVO {
    pub id: String,
    pub instance_id: String,
    pub definition_id: String,
    pub flow_name: Option<String>,
    pub flow_code: Option<String>,
    pub business_id: Option<String>,
    pub node_code: String,
    pub node_name: Option<String>,
    pub node_type: i32,
    pub flow_status: String,
    pub form_custom: Option<String>,
    pub form_path: Option<String>,
    pub create_by: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
}

impl From<Task> for TaskVO {
    fn from(arg: Task) -> Self {
        Self {
            id: arg.id.unwrap_or_default().to_string(),
            instance_id: arg.instance_id.to_string(),
            definition_id: arg.definition_id.to_string(),
            flow_name: None,
            flow_code: None,
            business_id: None,
            node_code: arg.node_code,
            node_name: arg.node_name,
            node_type: arg.node_type,
            flow_status: arg.flow_status,
            form_custom: arg.form_custom,
            form_path: arg.form_path,
            create_by: arg.create_by,
            create_time: arg.create_time,
        }
    }
}

// ======================================================================
// 已办任务
// ======================================================================

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct HisTaskVO {
    pub id: String,
    pub instance_id: String,
    pub definition_id: String,
    pub flow_name: Option<String>,
    pub business_id: Option<String>,
    pub node_code: Option<String>,
    pub node_name: Option<String>,
    pub target_node_code: Option<String>,
    pub target_node_name: Option<String>,
    pub approver: Option<String>,
    /// 办理人昵称（approver 存用户 id，保留原值供程序逻辑使用，昵称供展示）
    pub approver_name: Option<String>,
    /// 1 审批 2 转办 3 委派 4 会签 5 票签 6 加签 7 减签
    pub cooperate_type: i32,
    /// PASS / REJECT / NONE
    pub skip_type: String,
    pub flow_status: String,
    pub message: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_time: Option<chrono::NaiveDateTime>,
}

impl From<HisTask> for HisTaskVO {
    fn from(arg: HisTask) -> Self {
        Self {
            id: arg.id.unwrap_or_default().to_string(),
            instance_id: arg.instance_id.to_string(),
            definition_id: arg.definition_id.to_string(),
            flow_name: None,
            business_id: None,
            node_code: arg.node_code,
            node_name: arg.node_name,
            target_node_code: arg.target_node_code,
            target_node_name: arg.target_node_name,
            approver: arg.approver,
            approver_name: None,
            cooperate_type: arg.cooperate_type,
            skip_type: arg.skip_type,
            flow_status: arg.flow_status,
            message: arg.message,
            create_time: arg.create_time,
            update_time: arg.update_time,
        }
    }
}

// ======================================================================
// 请假单
// ======================================================================

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaveVO {
    pub id: String,
    pub apply_code: Option<String>,
    pub leave_type: String,
    pub start_date: Option<chrono::NaiveDateTime>,
    pub end_date: Option<chrono::NaiveDateTime>,
    pub leave_days: i32,
    pub remark: Option<String>,
    /// 业务状态：draft / waiting / finish / cancel / back / termination / invalid
    pub status: Option<String>,
    /// 实际走的流程编码（同一业务表可挂多个流程，列表页反查流程实例得到）
    pub flow_code: Option<String>,
    pub create_by: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
}

impl From<TestLeave> for LeaveVO {
    fn from(arg: TestLeave) -> Self {
        Self {
            id: arg.id.to_string(),
            apply_code: arg.apply_code,
            leave_type: arg.leave_type,
            start_date: Some(arg.start_date),
            end_date: Some(arg.end_date),
            leave_days: arg.leave_days,
            remark: arg.remark,
            status: arg.status,
            flow_code: None,
            create_by: arg.create_by.map(|v| v.to_string()),
            create_time: arg.create_time,
        }
    }
}

// ======================================================================
// 启动流程返回
// ======================================================================

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartReturnVO {
    /// 流程实例 id
    pub process_instance_id: String,
    /// 首个人工节点产生的待办 id
    pub task_id: Option<String>,
    /// 业务编号（请假单回填 applyCode）
    pub business_code: Option<String>,
}
