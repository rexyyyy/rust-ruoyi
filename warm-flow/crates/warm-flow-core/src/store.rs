//! 持久层抽象。Java 版用 DAO 接口 + MyBatis/Easy-Query 多套实现，
//! Rust 版收敛为一套异步 `WarmStore`，由 `warm-flow-orm` 用 SeaORM 实现。

use async_trait::async_trait;

use crate::entity::{Definition, Form, HisTask, Instance, Node, Skip, Task, User};
use crate::error::Result;

/// 流程定义分页查询条件
#[derive(Debug, Clone, Default)]
pub struct DefinitionQuery {
    pub flow_code: Option<String>,
    pub flow_name: Option<String>,
    pub category: Option<String>,
    pub is_publish: Option<i32>,
    pub page_num: u64,
    pub page_size: u64,
}

/// 流程实例分页查询条件
#[derive(Debug, Clone, Default)]
pub struct InstanceQuery {
    pub definition_id: Option<i64>,
    pub business_id: Option<String>,
    pub flow_status: Option<String>,
    pub create_by: Option<String>,
    pub page_num: u64,
    pub page_size: u64,
}

/// 待办任务查询条件
#[derive(Debug, Clone, Default)]
pub struct TaskQuery {
    pub instance_id: Option<i64>,
    pub definition_id: Option<i64>,
    pub node_code: Option<String>,
    pub flow_status: Option<String>,
    /// 办理人过滤：命中 flow_user 中该用户的任务
    pub handler: Option<String>,
    pub page_num: u64,
    pub page_size: u64,
}

/// 历史任务查询条件
#[derive(Debug, Clone, Default)]
pub struct HisTaskQuery {
    pub instance_id: Option<i64>,
    pub definition_id: Option<i64>,
    pub task_id: Option<i64>,
    pub approver: Option<String>,
    pub flow_status: Option<String>,
    pub page_num: u64,
    pub page_size: u64,
}

/// 分页结果
#[derive(Debug, Clone)]
pub struct PageData<T> {
    pub rows: Vec<T>,
    pub total: u64,
}

/// 工作流持久层。
///
/// 所有 `insert_*` 方法都会在返回前把生成的主键回填到入参（`&mut`），
/// 这与 Java 版 `save()` 之后实体携带 id 的行为一致。
#[async_trait]
pub trait WarmStore: Send + Sync {
    // ---------------- 流程定义 ----------------
    async fn insert_definition(&self, definition: &mut Definition) -> Result<()>;
    async fn update_definition(&self, definition: &Definition) -> Result<()>;
    async fn delete_definitions(&self, ids: &[i64]) -> Result<()>;
    async fn get_definition(&self, id: i64) -> Result<Option<Definition>>;
    async fn find_definition(&self, flow_code: &str, is_publish: Option<i32>) -> Result<Option<Definition>>;
    async fn list_definitions_by_code(&self, flow_code: &str) -> Result<Vec<Definition>>;
    async fn list_definitions(&self, query: &DefinitionQuery) -> Result<PageData<Definition>>;
    async fn update_publish_status(&self, ids: &[i64], status: i32) -> Result<()>;

    // ---------------- 节点 ----------------
    async fn insert_nodes(&self, nodes: &mut [Node]) -> Result<()>;
    async fn delete_nodes_by_definition(&self, definition_ids: &[i64]) -> Result<()>;
    async fn list_nodes(&self, definition_id: i64) -> Result<Vec<Node>>;
    async fn get_node(&self, definition_id: i64, node_code: &str) -> Result<Option<Node>>;
    async fn get_node_by_id(&self, id: i64) -> Result<Option<Node>>;
    async fn find_node_by_type(&self, definition_id: i64, node_type: i32) -> Result<Option<Node>>;
    async fn list_nodes_by_codes(&self, definition_id: i64, node_codes: &[String]) -> Result<Vec<Node>>;

    // ---------------- 跳转 ----------------
    async fn insert_skips(&self, skips: &mut [Skip]) -> Result<()>;
    async fn delete_skips_by_definition(&self, definition_ids: &[i64]) -> Result<()>;
    async fn list_skips(&self, definition_id: i64) -> Result<Vec<Skip>>;

    // ---------------- 流程实例 ----------------
    async fn insert_instance(&self, instance: &mut Instance) -> Result<()>;
    async fn update_instance(&self, instance: &Instance) -> Result<()>;
    async fn get_instance(&self, id: i64) -> Result<Option<Instance>>;
    async fn list_instances_by_definition(&self, definition_id: i64) -> Result<Vec<Instance>>;
    async fn list_instances_by_definitions(&self, definition_ids: &[i64]) -> Result<Vec<Instance>>;
    async fn list_instances(&self, query: &InstanceQuery) -> Result<PageData<Instance>>;
    async fn delete_instances(&self, ids: &[i64]) -> Result<()>;

    // ---------------- 待办任务 ----------------
    async fn insert_tasks(&self, tasks: &mut [Task]) -> Result<()>;
    async fn delete_tasks(&self, ids: &[i64]) -> Result<()>;
    async fn delete_tasks_by_instances(&self, instance_ids: &[i64]) -> Result<()>;
    async fn get_task(&self, id: i64) -> Result<Option<Task>>;
    async fn list_tasks_by_instance(&self, instance_id: i64) -> Result<Vec<Task>>;
    async fn list_tasks_by_instance_and_nodes(&self, instance_id: i64, node_codes: &[String]) -> Result<Vec<Task>>;
    async fn list_tasks(&self, query: &TaskQuery) -> Result<PageData<Task>>;

    // ---------------- 历史任务 ----------------
    async fn insert_his_tasks(&self, his_tasks: &mut [HisTask]) -> Result<()>;
    async fn get_his_task(&self, id: i64) -> Result<Option<HisTask>>;
    async fn list_his_tasks_by_instance(&self, instance_id: i64) -> Result<Vec<HisTask>>;
    async fn list_his_tasks_by_task(&self, task_id: i64) -> Result<Vec<HisTask>>;
    async fn list_his_tasks(&self, query: &HisTaskQuery) -> Result<PageData<HisTask>>;
    async fn delete_his_tasks_by_instances(&self, instance_ids: &[i64]) -> Result<()>;

    // ---------------- 流程用户 ----------------
    async fn insert_users(&self, users: &mut [User]) -> Result<()>;
    async fn delete_users_by_tasks(&self, task_ids: &[i64]) -> Result<()>;
    async fn delete_user(&self, id: i64) -> Result<()>;
    async fn list_users_by_task(&self, task_id: i64) -> Result<Vec<User>>;
    async fn list_users_by_task_and_types(&self, task_id: i64, types: &[String]) -> Result<Vec<User>>;
    async fn find_user(&self, task_id: i64, processed_by: &str, user_type: &str) -> Result<Option<User>>;
    async fn list_users_by_tasks_and_types(&self, task_ids: &[i64], types: &[String]) -> Result<Vec<User>>;

    // ---------------- 表单 ----------------
    async fn get_form(&self, id: i64) -> Result<Option<Form>>;
    async fn list_forms(&self, is_publish: Option<i32>) -> Result<Vec<Form>>;
    async fn insert_form(&self, form: &mut Form) -> Result<()>;
    async fn update_form(&self, form: &Form) -> Result<()>;
}
