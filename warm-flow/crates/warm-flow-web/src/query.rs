//! 分页与业务查询参数。字段名沿用前端习惯的 `pageNum` / `pageSize`。

use serde::Deserialize;

use warm_flow_core::store::{DefinitionQuery, HisTaskQuery, InstanceQuery, TaskQuery};

const DEFAULT_PAGE_SIZE: u64 = 10;
const MAX_PAGE_SIZE: u64 = 500;

/// 分页参数
#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageQuery {
    #[serde(default)]
    pub page_num: Option<u64>,
    #[serde(default)]
    pub page_size: Option<u64>,
}

impl PageQuery {
    pub fn page_num(&self) -> u64 {
        self.page_num.unwrap_or(1).max(1)
    }

    pub fn page_size(&self) -> u64 {
        self.page_size.unwrap_or(DEFAULT_PAGE_SIZE).clamp(1, MAX_PAGE_SIZE)
    }
}

/// 流程定义分页查询
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionPageQuery {
    pub flow_code: Option<String>,
    pub flow_name: Option<String>,
    pub category: Option<String>,
    pub is_publish: Option<i32>,
    #[serde(flatten)]
    pub page: PageQuery,
}

impl From<&DefinitionPageQuery> for DefinitionQuery {
    fn from(q: &DefinitionPageQuery) -> Self {
        Self {
            flow_code: q.flow_code.clone(),
            flow_name: q.flow_name.clone(),
            category: q.category.clone(),
            is_publish: q.is_publish,
            page_num: q.page.page_num(),
            page_size: q.page.page_size(),
        }
    }
}

/// 流程实例分页查询
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstancePageQuery {
    pub definition_id: Option<i64>,
    pub business_id: Option<String>,
    pub flow_status: Option<String>,
    pub create_by: Option<String>,
    #[serde(flatten)]
    pub page: PageQuery,
}

impl From<&InstancePageQuery> for InstanceQuery {
    fn from(q: &InstancePageQuery) -> Self {
        Self {
            definition_id: q.definition_id,
            business_id: q.business_id.clone(),
            flow_status: q.flow_status.clone(),
            create_by: q.create_by.clone(),
            page_num: q.page.page_num(),
            page_size: q.page.page_size(),
        }
    }
}

/// 待办任务分页查询
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskPageQuery {
    pub instance_id: Option<i64>,
    pub definition_id: Option<i64>,
    pub node_code: Option<String>,
    pub flow_status: Option<String>,
    /// 按办理人过滤（命中 flow_user）
    pub handler: Option<String>,
    #[serde(flatten)]
    pub page: PageQuery,
}

impl From<&TaskPageQuery> for TaskQuery {
    fn from(q: &TaskPageQuery) -> Self {
        Self {
            instance_id: q.instance_id,
            definition_id: q.definition_id,
            node_code: q.node_code.clone(),
            flow_status: q.flow_status.clone(),
            handler: q.handler.clone(),
            page_num: q.page.page_num(),
            page_size: q.page.page_size(),
        }
    }
}

/// 历史任务分页查询
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HisTaskPageQuery {
    pub instance_id: Option<i64>,
    pub definition_id: Option<i64>,
    pub task_id: Option<i64>,
    pub approver: Option<String>,
    pub flow_status: Option<String>,
    #[serde(flatten)]
    pub page: PageQuery,
}

impl From<&HisTaskPageQuery> for HisTaskQuery {
    fn from(q: &HisTaskPageQuery) -> Self {
        Self {
            instance_id: q.instance_id,
            definition_id: q.definition_id,
            task_id: q.task_id,
            approver: q.approver.clone(),
            flow_status: q.flow_status.clone(),
            page_num: q.page.page_num(),
            page_size: q.page.page_size(),
        }
    }
}

/// 任意跳转参数（`passAtWill` / `rejectAtWill`）
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtWillQuery {
    /// 目标节点编码
    pub node_code: String,
    /// 是否忽略权限校验
    #[serde(default)]
    pub ignore: bool,
}
