//! 领域实体。字段与 Java 版 `entity` 包一一对应，表结构见 `sql/*/warm-flow-all.sql`。
//!
//! 与 Java 版的差异：
//! - Java 用 `interface + setter 链式调用`，Rust 用 `struct` + `Default`；
//! - `variable` / `def_json` / `ext` 在库里是字符串，这里保持 `String`，并额外提供
//!   `*_json` / `*_map` 便捷方法做 JSON 转换；
//! - `node_list` / `skip_list` / `user_list` / `permission_list` 是内存态字段，
//!   不落库（标了 `#[serde(default)]`，前端可忽略）。

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::enums::{ActivityStatus, PublishStatus};

/// 公共审计字段：对应 Java `RootEntity`
pub trait RootEntity {
    fn id(&self) -> Option<i64>;
    fn set_id(&mut self, id: i64);
}

/// 流程定义 flow_definition
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    pub id: Option<i64>,
    pub flow_code: String,
    pub flow_name: String,
    /// CLASSICS 经典模型 / MIMIC 仿钉钉模型
    #[serde(default)]
    pub model_value: String,
    pub category: Option<String>,
    pub version: String,
    pub is_publish: i32,
    pub form_custom: Option<String>,
    pub form_path: Option<String>,
    pub activity_status: i32,
    pub listener_type: Option<String>,
    pub listener_path: Option<String>,
    pub ext: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub create_by: Option<String>,
    pub update_time: Option<NaiveDateTime>,
    pub update_by: Option<String>,
    pub del_flag: Option<String>,
    pub tenant_id: Option<String>,

    /// 内存态：节点集合，导入导出设计器 JSON 时使用
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub node_list: Vec<Node>,
}

impl Definition {
    pub fn new(flow_code: impl Into<String>, flow_name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            flow_code: flow_code.into(),
            flow_name: flow_name.into(),
            model_value: crate::enums::ModelEnum::CLASSICS.to_string(),
            version: version.into(),
            is_publish: PublishStatus::UNPUBLISHED.0,
            activity_status: ActivityStatus::ACTIVITY.0,
            form_custom: Some(crate::enums::FormCustomEnum::N.to_string()),
            ..Default::default()
        }
    }

    pub fn is_activity(&self) -> bool {
        ActivityStatus::is_activity(self.activity_status)
    }
}

impl RootEntity for Definition {
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }
}

/// 流程节点 flow_node
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: Option<i64>,
    /// 0 开始 1 中间 2 结束 3 互斥网关 4 并行网关 5 包容网关
    pub node_type: i32,
    pub definition_id: i64,
    pub node_code: String,
    pub node_name: Option<String>,
    /// 权限标识，多个用 `@@` 分隔
    pub permission_flag: Option<String>,
    /// 签署比例：0 或签、100 会签、0~100 票签
    pub node_ratio: Option<String>,
    pub coordinate: Option<String>,
    /// 任意结点跳转（退回时直达节点）
    pub any_node_skip: Option<String>,
    pub listener_type: Option<String>,
    pub listener_path: Option<String>,
    pub form_custom: Option<String>,
    pub form_path: Option<String>,
    pub version: Option<String>,
    pub ext: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub create_by: Option<String>,
    pub update_time: Option<NaiveDateTime>,
    pub update_by: Option<String>,
    pub del_flag: Option<String>,
    pub tenant_id: Option<String>,

    /// 内存态：当前节点的跳转集合
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skip_list: Vec<Skip>,
}

impl Node {
    pub fn new(node_type: i32, node_code: impl Into<String>, node_name: impl Into<String>) -> Self {
        Self {
            node_type,
            node_code: node_code.into(),
            node_name: Some(node_name.into()),
            node_ratio: Some("0".to_string()),
            form_custom: Some(crate::enums::FormCustomEnum::N.to_string()),
            ..Default::default()
        }
    }

    pub fn ratio(&self) -> String {
        self.node_ratio.clone().unwrap_or_else(|| "0".to_string())
    }

    /// 节点自定义表单优先，其次流程定义表单（对应 Java `TaskServiceImpl#addTask`）
    pub fn form_of(&self, definition: &Definition) -> (Option<String>, Option<String>) {
        if self.form_custom.as_deref().map(|v| !v.is_empty()).unwrap_or(false)
            && self.form_path.as_deref().map(|v| !v.is_empty()).unwrap_or(false)
        {
            (self.form_custom.clone(), self.form_path.clone())
        } else {
            (definition.form_custom.clone(), definition.form_path.clone())
        }
    }
}

impl RootEntity for Node {
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }
}

/// 节点跳转 flow_skip
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skip {
    pub id: Option<i64>,
    pub definition_id: i64,
    pub now_node_code: String,
    pub now_node_type: Option<i32>,
    pub next_node_code: String,
    pub next_node_type: Option<i32>,
    pub skip_name: Option<String>,
    /// PASS / REJECT
    pub skip_type: Option<String>,
    /// 跳转条件，如 `eq@@flag|4`
    pub skip_condition: Option<String>,
    pub coordinate: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub create_by: Option<String>,
    pub update_time: Option<NaiveDateTime>,
    pub update_by: Option<String>,
    pub del_flag: Option<String>,
    pub tenant_id: Option<String>,
}

impl Skip {
    pub fn new(now_node_code: impl Into<String>, next_node_code: impl Into<String>, skip_type: impl Into<String>) -> Self {
        Self {
            now_node_code: now_node_code.into(),
            next_node_code: next_node_code.into(),
            skip_type: Some(skip_type.into()),
            ..Default::default()
        }
    }
}

/// 流程实例 flow_instance
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub id: Option<i64>,
    pub definition_id: i64,
    pub business_id: String,
    pub node_type: i32,
    pub node_code: String,
    pub node_name: Option<String>,
    /// 流程变量 JSON 字符串
    pub variable: Option<String>,
    pub flow_status: String,
    pub activity_status: i32,
    /// 流程图元数据 JSON（带节点/连线办理状态）
    pub def_json: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub create_by: Option<String>,
    pub update_time: Option<NaiveDateTime>,
    pub update_by: Option<String>,
    pub ext: Option<String>,
    pub del_flag: Option<String>,
    pub tenant_id: Option<String>,
}

impl Instance {
    /// 流程变量（JSON 对象）
    pub fn variable_map(&self) -> serde_json::Map<String, Value> {
        self.variable
            .as_deref()
            .and_then(|s| serde_json::from_str::<Value>(s).ok())
            .and_then(|v| v.as_object().cloned())
            .unwrap_or_default()
    }

    pub fn set_variable_map(&mut self, map: &serde_json::Map<String, Value>) {
        self.variable = serde_json::to_string(map).ok();
    }

    pub fn is_activity(&self) -> bool {
        ActivityStatus::is_activity(self.activity_status)
    }
}

/// 待办任务 flow_task
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: Option<i64>,
    pub definition_id: i64,
    pub instance_id: i64,
    pub node_code: String,
    pub node_name: Option<String>,
    pub node_type: i32,
    pub flow_status: String,
    pub form_custom: Option<String>,
    pub form_path: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub create_by: Option<String>,
    pub update_time: Option<NaiveDateTime>,
    pub update_by: Option<String>,
    pub del_flag: Option<String>,
    pub tenant_id: Option<String>,

    /// 内存态：当前任务的办理人权限（来自 flow_user）
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permission_list: Vec<String>,
    /// 内存态：当前任务的办理人明细
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_list: Vec<User>,
}

impl Task {
    pub fn new(instance_id: i64, definition_id: i64, node_code: impl Into<String>) -> Self {
        Self {
            instance_id,
            definition_id,
            node_code: node_code.into(),
            ..Default::default()
        }
    }
}

/// 历史任务 flow_his_task
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HisTask {
    pub id: Option<i64>,
    pub definition_id: i64,
    pub instance_id: i64,
    pub task_id: i64,
    pub node_code: Option<String>,
    pub node_name: Option<String>,
    pub node_type: Option<i32>,
    pub target_node_code: Option<String>,
    pub target_node_name: Option<String>,
    pub approver: Option<String>,
    /// 1 审批 2 转办 3 委派 4 会签 5 票签 6 加签 7 减签
    pub cooperate_type: i32,
    pub collaborator: Option<String>,
    /// PASS / REJECT / NONE
    pub skip_type: String,
    pub flow_status: String,
    pub form_custom: Option<String>,
    pub form_path: Option<String>,
    pub message: Option<String>,
    pub variable: Option<String>,
    pub ext: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub del_flag: Option<String>,
    pub tenant_id: Option<String>,
}

impl HisTask {
    pub fn variable_map(&self) -> serde_json::Map<String, Value> {
        self.variable
            .as_deref()
            .and_then(|s| serde_json::from_str::<Value>(s).ok())
            .and_then(|v| v.as_object().cloned())
            .unwrap_or_default()
    }
}

/// 流程用户权限 flow_user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<i64>,
    /// 1 审批人 2 转办人 3 委托人
    pub r#type: String,
    pub processed_by: Option<String>,
    /// 关联的任务 id
    pub associated: i64,
    pub create_time: Option<NaiveDateTime>,
    pub create_by: Option<String>,
    pub update_time: Option<NaiveDateTime>,
    pub update_by: Option<String>,
    pub del_flag: Option<String>,
    pub tenant_id: Option<String>,
}

impl User {
    pub fn new(associated: i64, processed_by: impl Into<String>, user_type: impl Into<String>) -> Self {
        Self {
            associated,
            processed_by: Some(processed_by.into()),
            r#type: user_type.into(),
            ..Default::default()
        }
    }
}

/// 流程表单 flow_form
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub id: Option<i64>,
    pub form_code: String,
    pub form_name: String,
    pub version: String,
    pub is_publish: i32,
    /// 0 内置表单（存 form_content） 1 外挂表单（存 form_path）
    pub form_type: Option<i32>,
    pub form_path: Option<String>,
    pub form_content: Option<String>,
    pub ext: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub del_flag: Option<String>,
    pub tenant_id: Option<String>,
}
