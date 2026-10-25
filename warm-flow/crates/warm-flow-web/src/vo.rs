//! 设计器与业务接口的出入参。字段名与 Java 版 VO/DTO 保持一致（camelCase）。

use serde::{Deserialize, Serialize};

use warm_flow_core::dto::Tree;
use warm_flow_core::store::PageData;

/// 分页结果，对应 Java `FlowPage<T>`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageVo<T> {
    pub total: u64,
    pub rows: Vec<T>,
}

impl<T> PageVo<T> {
    pub fn new(rows: Vec<T>, total: u64) -> Self {
        Self { total, rows }
    }
}

impl<T> From<PageData<T>> for PageVo<T> {
    fn from(page: PageData<T>) -> Self {
        Self { total: page.total, rows: page.rows }
    }
}

/// 字典项，对应 Java `Dict`（办理人表达式下拉、节点扩展属性字典）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dict {
    pub label: Option<String>,
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub child_list: Vec<Dict>,
}

impl Dict {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self { label: Some(label.into()), value: Some(value.into()), child_list: Vec::new() }
    }
}

/// 办理人权限设置查询条件，对应 Java `HandlerQuery`
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandlerQuery {
    pub handler_code: Option<String>,
    pub handler_name: Option<String>,
    pub handler_type: Option<String>,
    pub group_id: Option<String>,
    #[serde(default)]
    pub page_num: Option<u64>,
    #[serde(default)]
    pub page_size: Option<u64>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

impl HandlerQuery {
    pub fn page_num(&self) -> u64 {
        self.page_num.unwrap_or(1).max(1)
    }

    pub fn page_size(&self) -> u64 {
        self.page_size.unwrap_or(10).clamp(1, 500)
    }
}

/// 办理人权限明细，对应 Java `HandlerAuth`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandlerAuth {
    pub storage_id: Option<String>,
    pub handler_code: Option<String>,
    pub handler_name: Option<String>,
    pub group_name: Option<String>,
    pub create_time: Option<String>,
}

/// 办理人权限设置列表结果，对应 Java `HandlerSelectVo`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandlerSelectVo {
    pub handler_auths: PageVo<HandlerAuth>,
    pub tree_selections: Vec<Tree>,
}

impl Default for PageVo<HandlerAuth> {
    fn default() -> Self {
        Self { total: 0, rows: Vec::new() }
    }
}

/// 办理人权限名称回显，对应 Java `HandlerFeedBackVo`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandlerFeedBackVo {
    pub storage_id: Option<String>,
    pub handler_name: Option<String>,
}

/// 节点扩展属性，对应 Java `NodeExt`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeExt {
    pub code: Option<String>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub r#type: i32,
    #[serde(default)]
    pub childs: Vec<NodeExtChild>,
}

/// 节点扩展属性子项，对应 Java `NodeExt.ChildNode`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeExtChild {
    pub code: Option<String>,
    pub desc: Option<String>,
    pub label: Option<String>,
    pub r#type: i32,
    pub must: bool,
    pub multiple: bool,
    pub precision: i32,
    pub step: Option<String>,
    pub min: Option<String>,
    pub date_type: Option<String>,
    pub date_format: Option<String>,
    #[serde(default)]
    pub dict: Vec<Dict>,
}

/// 监听器列表项，对应 Java `ListenerVo`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenerVo {
    pub r#type: Option<String>,
    pub path: Option<String>,
    pub description: Option<String>,
}

/// 设计器配置，对应 Java `WarmFlowVo`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarmFlowVo {
    pub token_name_list: Vec<String>,
    pub framework: String,
}

/// 表单内容保存入参，对应 Java `FlowDto` 的表单部分
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormContentDto {
    pub id: Option<i64>,
    pub form_content: Option<String>,
}

/// 开启流程入参
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartDto {
    /// 业务主键
    pub business_id: String,
    /// 流程变量，等价于 `FlowParams.variable`
    #[serde(default)]
    pub variable: serde_json::Map<String, serde_json::Value>,
    /// 完整流程参数（可选，优先级高于 `variable`）
    #[serde(default)]
    pub flow_params: Option<warm_flow_core::dto::FlowParams>,
}
