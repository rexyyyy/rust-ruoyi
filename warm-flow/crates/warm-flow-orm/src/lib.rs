//! SeaORM 版持久层实现。对应 Java 版 `warm-flow-orm` 的 dao + `WarmServiceImpl`。
//!
//! 与 Java 版的差异：
//! - 所有写操作直接落库，不做逻辑删除（Java 的 `logicDelete` 配置在 Rust 版未启用）；
//! - 主键由内置 19 位雪花生成，调用方不传 id 时自动补齐。

pub mod entity;

use async_trait::async_trait;
use sea_orm::sea_query::Query;
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use warm_flow_core::entity::{Definition, Form, HisTask, Instance, Node, Skip, Task, User};
use warm_flow_core::error::{sea_orm_like::DbError, FlowError, Result};
use warm_flow_core::handler::SnowflakeId19;
use warm_flow_core::store::{
    DefinitionQuery, HisTaskQuery, InstanceQuery, PageData, TaskQuery, WarmStore,
};

use crate::entity::{
    flow_definition as definition, flow_form as form, flow_his_task as his_task, flow_instance as instance,
    flow_node as node, flow_skip as skip, flow_task as task, flow_user as user,
};

/// 把 `sea_orm::DbErr` 收敛成引擎错误，避免上层每处都写 map_err
trait DbResultExt<T> {
    fn db_err(self) -> Result<T>;
}

impl<T> DbResultExt<T> for std::result::Result<T, DbErr> {
    fn db_err(self) -> Result<T> {
        self.map_err(|e| FlowError::Db(DbError(e.to_string())))
    }
}

/// SeaORM 持久层
#[derive(Clone)]
pub struct SeaOrmStore {
    db: DatabaseConnection,
    id_gen: SnowflakeId19,
}

impl SeaOrmStore {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db, id_gen: SnowflakeId19::default() }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    fn next_id(&self) -> i64 {
        self.id_gen.next_id()
    }
}

// ----------------------------------------------------------------------
// 实体转换
// ----------------------------------------------------------------------

impl From<definition::Model> for Definition {
    fn from(m: definition::Model) -> Self {
        Self {
            id: Some(m.id),
            flow_code: m.flow_code,
            flow_name: m.flow_name,
            model_value: m.model_value,
            category: m.category,
            version: m.version,
            is_publish: m.is_publish,
            form_custom: m.form_custom,
            form_path: m.form_path,
            activity_status: m.activity_status,
            listener_type: m.listener_type,
            listener_path: m.listener_path,
            ext: m.ext,
            create_time: m.create_time,
            create_by: m.create_by,
            update_time: m.update_time,
            update_by: m.update_by,
            del_flag: m.del_flag,
            tenant_id: m.tenant_id,
            node_list: Vec::new(),
        }
    }
}

fn definition_active(d: &Definition) -> definition::ActiveModel {
    definition::ActiveModel {
        id: Set(d.id.unwrap_or_default()),
        flow_code: Set(d.flow_code.clone()),
        flow_name: Set(d.flow_name.clone()),
        model_value: Set(d.model_value.clone()),
        category: Set(d.category.clone()),
        version: Set(d.version.clone()),
        is_publish: Set(d.is_publish),
        form_custom: Set(d.form_custom.clone()),
        form_path: Set(d.form_path.clone()),
        activity_status: Set(d.activity_status),
        listener_type: Set(d.listener_type.clone()),
        listener_path: Set(d.listener_path.clone()),
        ext: Set(d.ext.clone()),
        create_time: Set(d.create_time),
        create_by: Set(d.create_by.clone()),
        update_time: Set(d.update_time),
        update_by: Set(d.update_by.clone()),
        del_flag: Set(d.del_flag.clone()),
        tenant_id: Set(d.tenant_id.clone()),
    }
}

impl From<node::Model> for Node {
    fn from(m: node::Model) -> Self {
        Self {
            id: Some(m.id),
            node_type: m.node_type,
            definition_id: m.definition_id,
            node_code: m.node_code,
            node_name: m.node_name,
            permission_flag: m.permission_flag,
            node_ratio: m.node_ratio,
            coordinate: m.coordinate,
            any_node_skip: m.any_node_skip,
            listener_type: m.listener_type,
            listener_path: m.listener_path,
            form_custom: m.form_custom,
            form_path: m.form_path,
            version: m.version,
            ext: m.ext,
            create_time: m.create_time,
            create_by: m.create_by,
            update_time: m.update_time,
            update_by: m.update_by,
            del_flag: m.del_flag,
            tenant_id: m.tenant_id,
            skip_list: Vec::new(),
        }
    }
}

fn node_active(n: &Node, id: i64) -> node::ActiveModel {
    node::ActiveModel {
        id: Set(id),
        node_type: Set(n.node_type),
        definition_id: Set(n.definition_id),
        node_code: Set(n.node_code.clone()),
        node_name: Set(n.node_name.clone()),
        permission_flag: Set(n.permission_flag.clone()),
        node_ratio: Set(n.node_ratio.clone()),
        coordinate: Set(n.coordinate.clone()),
        any_node_skip: Set(n.any_node_skip.clone()),
        listener_type: Set(n.listener_type.clone()),
        listener_path: Set(n.listener_path.clone()),
        form_custom: Set(n.form_custom.clone()),
        form_path: Set(n.form_path.clone()),
        version: Set(n.version.clone()),
        create_time: Set(n.create_time),
        create_by: Set(n.create_by.clone()),
        update_time: Set(n.update_time),
        update_by: Set(n.update_by.clone()),
        ext: Set(n.ext.clone()),
        del_flag: Set(n.del_flag.clone()),
        tenant_id: Set(n.tenant_id.clone()),
    }
}

impl From<skip::Model> for Skip {
    fn from(m: skip::Model) -> Self {
        Self {
            id: Some(m.id),
            definition_id: m.definition_id,
            now_node_code: m.now_node_code,
            now_node_type: m.now_node_type,
            next_node_code: m.next_node_code,
            next_node_type: m.next_node_type,
            skip_name: m.skip_name,
            skip_type: m.skip_type,
            skip_condition: m.skip_condition,
            coordinate: m.coordinate,
            create_time: m.create_time,
            create_by: m.create_by,
            update_time: m.update_time,
            update_by: m.update_by,
            del_flag: m.del_flag,
            tenant_id: m.tenant_id,
        }
    }
}

fn skip_active(s: &Skip, id: i64) -> skip::ActiveModel {
    skip::ActiveModel {
        id: Set(id),
        definition_id: Set(s.definition_id),
        now_node_code: Set(s.now_node_code.clone()),
        now_node_type: Set(s.now_node_type),
        next_node_code: Set(s.next_node_code.clone()),
        next_node_type: Set(s.next_node_type),
        skip_name: Set(s.skip_name.clone()),
        skip_type: Set(s.skip_type.clone()),
        skip_condition: Set(s.skip_condition.clone()),
        coordinate: Set(s.coordinate.clone()),
        create_time: Set(s.create_time),
        create_by: Set(s.create_by.clone()),
        update_time: Set(s.update_time),
        update_by: Set(s.update_by.clone()),
        del_flag: Set(s.del_flag.clone()),
        tenant_id: Set(s.tenant_id.clone()),
    }
}

impl From<instance::Model> for Instance {
    fn from(m: instance::Model) -> Self {
        Self {
            id: Some(m.id),
            definition_id: m.definition_id,
            business_id: m.business_id,
            node_type: m.node_type,
            node_code: m.node_code,
            node_name: m.node_name,
            variable: m.variable,
            flow_status: m.flow_status,
            activity_status: m.activity_status,
            def_json: m.def_json,
            create_time: m.create_time,
            create_by: m.create_by,
            update_time: m.update_time,
            update_by: m.update_by,
            ext: m.ext,
            del_flag: m.del_flag,
            tenant_id: m.tenant_id,
        }
    }
}

fn instance_active(i: &Instance) -> instance::ActiveModel {
    instance::ActiveModel {
        id: Set(i.id.unwrap_or_default()),
        definition_id: Set(i.definition_id),
        business_id: Set(i.business_id.clone()),
        node_type: Set(i.node_type),
        node_code: Set(i.node_code.clone()),
        node_name: Set(i.node_name.clone()),
        variable: Set(i.variable.clone()),
        flow_status: Set(i.flow_status.clone()),
        activity_status: Set(i.activity_status),
        def_json: Set(i.def_json.clone()),
        create_time: Set(i.create_time),
        create_by: Set(i.create_by.clone()),
        update_time: Set(i.update_time),
        update_by: Set(i.update_by.clone()),
        ext: Set(i.ext.clone()),
        del_flag: Set(i.del_flag.clone()),
        tenant_id: Set(i.tenant_id.clone()),
    }
}

impl From<task::Model> for Task {
    fn from(m: task::Model) -> Self {
        Self {
            id: Some(m.id),
            definition_id: m.definition_id,
            instance_id: m.instance_id,
            node_code: m.node_code,
            node_name: m.node_name,
            node_type: m.node_type,
            flow_status: m.flow_status,
            form_custom: m.form_custom,
            form_path: m.form_path,
            create_time: m.create_time,
            create_by: m.create_by,
            update_time: m.update_time,
            update_by: m.update_by,
            del_flag: m.del_flag,
            tenant_id: m.tenant_id,
            permission_list: Vec::new(),
            user_list: Vec::new(),
        }
    }
}

fn task_active(t: &Task, id: i64) -> task::ActiveModel {
    task::ActiveModel {
        id: Set(id),
        definition_id: Set(t.definition_id),
        instance_id: Set(t.instance_id),
        node_code: Set(t.node_code.clone()),
        node_name: Set(t.node_name.clone()),
        node_type: Set(t.node_type),
        flow_status: Set(t.flow_status.clone()),
        form_custom: Set(t.form_custom.clone()),
        form_path: Set(t.form_path.clone()),
        create_time: Set(t.create_time),
        create_by: Set(t.create_by.clone()),
        update_time: Set(t.update_time),
        update_by: Set(t.update_by.clone()),
        del_flag: Set(t.del_flag.clone()),
        tenant_id: Set(t.tenant_id.clone()),
    }
}

impl From<his_task::Model> for HisTask {
    fn from(m: his_task::Model) -> Self {
        Self {
            id: Some(m.id),
            definition_id: m.definition_id,
            instance_id: m.instance_id,
            task_id: m.task_id,
            node_code: m.node_code,
            node_name: m.node_name,
            node_type: m.node_type,
            target_node_code: m.target_node_code,
            target_node_name: m.target_node_name,
            approver: m.approver,
            cooperate_type: m.cooperate_type,
            collaborator: m.collaborator,
            skip_type: m.skip_type,
            flow_status: m.flow_status,
            form_custom: m.form_custom,
            form_path: m.form_path,
            message: m.message,
            variable: m.variable,
            ext: m.ext,
            create_time: m.create_time,
            update_time: m.update_time,
            del_flag: m.del_flag,
            tenant_id: m.tenant_id,
        }
    }
}

fn his_task_active(h: &HisTask, id: i64) -> his_task::ActiveModel {
    his_task::ActiveModel {
        id: Set(id),
        definition_id: Set(h.definition_id),
        instance_id: Set(h.instance_id),
        task_id: Set(h.task_id),
        node_code: Set(h.node_code.clone()),
        node_name: Set(h.node_name.clone()),
        node_type: Set(h.node_type),
        target_node_code: Set(h.target_node_code.clone()),
        target_node_name: Set(h.target_node_name.clone()),
        approver: Set(h.approver.clone()),
        cooperate_type: Set(h.cooperate_type),
        collaborator: Set(h.collaborator.clone()),
        skip_type: Set(h.skip_type.clone()),
        flow_status: Set(h.flow_status.clone()),
        form_custom: Set(h.form_custom.clone()),
        form_path: Set(h.form_path.clone()),
        message: Set(h.message.clone()),
        variable: Set(h.variable.clone()),
        ext: Set(h.ext.clone()),
        create_time: Set(h.create_time),
        update_time: Set(h.update_time),
        del_flag: Set(h.del_flag.clone()),
        tenant_id: Set(h.tenant_id.clone()),
    }
}

impl From<user::Model> for User {
    fn from(m: user::Model) -> Self {
        Self {
            id: Some(m.id),
            r#type: m.r#type,
            processed_by: m.processed_by,
            associated: m.associated,
            create_time: m.create_time,
            create_by: m.create_by,
            update_time: m.update_time,
            update_by: m.update_by,
            del_flag: m.del_flag,
            tenant_id: m.tenant_id,
        }
    }
}

fn user_active(u: &User, id: i64) -> user::ActiveModel {
    user::ActiveModel {
        id: Set(id),
        r#type: Set(u.r#type.clone()),
        processed_by: Set(u.processed_by.clone()),
        associated: Set(u.associated),
        create_time: Set(u.create_time),
        create_by: Set(u.create_by.clone()),
        update_time: Set(u.update_time),
        update_by: Set(u.update_by.clone()),
        del_flag: Set(u.del_flag.clone()),
        tenant_id: Set(u.tenant_id.clone()),
    }
}

impl From<form::Model> for Form {
    fn from(m: form::Model) -> Self {
        Self {
            id: Some(m.id),
            form_code: m.form_code,
            form_name: m.form_name,
            version: m.version,
            is_publish: m.is_publish,
            form_type: m.form_type,
            form_path: m.form_path,
            form_content: m.form_content,
            ext: m.ext,
            create_time: m.create_time,
            update_time: m.update_time,
            del_flag: m.del_flag,
            tenant_id: m.tenant_id,
        }
    }
}

fn form_active(f: &Form) -> form::ActiveModel {
    form::ActiveModel {
        id: Set(f.id.unwrap_or_default()),
        form_code: Set(f.form_code.clone()),
        form_name: Set(f.form_name.clone()),
        version: Set(f.version.clone()),
        is_publish: Set(f.is_publish),
        form_type: Set(f.form_type),
        form_path: Set(f.form_path.clone()),
        form_content: Set(f.form_content.clone()),
        ext: Set(f.ext.clone()),
        create_time: Set(f.create_time),
        update_time: Set(f.update_time),
        del_flag: Set(f.del_flag.clone()),
        tenant_id: Set(f.tenant_id.clone()),
    }
}

/// 页码换算：引擎按 1 起始传，SeaORM 按 0 起始取
fn page_offset(page_num: u64, page_size: u64) -> (u64, u64) {
    let size = if page_size == 0 { 10 } else { page_size.min(500) };
    let num = page_num.saturating_sub(1);
    (num, size)
}

#[async_trait]
impl WarmStore for SeaOrmStore {
    // ---------------- 流程定义 ----------------

    async fn insert_definition(&self, definition: &mut Definition) -> Result<()> {
        let id = definition.id.unwrap_or_else(|| self.next_id());
        definition.id = Some(id);
        definition::Entity::insert(definition_active(definition))
            .exec(&self.db)
            .await
            .db_err()?;
        Ok(())
    }

    async fn update_definition(&self, definition: &Definition) -> Result<()> {
        definition::Entity::update(definition_active(definition))
            .exec(&self.db)
            .await
            .db_err()?;
        Ok(())
    }

    async fn delete_definitions(&self, ids: &[i64]) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }
        definition::Entity::delete_many()
            .filter(definition::Column::Id.is_in(ids.to_vec()))
            .exec(&self.db)
            .await
            .db_err()?;
        Ok(())
    }

    async fn get_definition(&self, id: i64) -> Result<Option<Definition>> {
        Ok(definition::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .db_err()?
            .map(Definition::from))
    }

    async fn find_definition(&self, flow_code: &str, is_publish: Option<i32>) -> Result<Option<Definition>> {
        let mut query = definition::Entity::find().filter(definition::Column::FlowCode.eq(flow_code));
        if let Some(status) = is_publish {
            query = query.filter(definition::Column::IsPublish.eq(status));
        }
        Ok(query
            .order_by_desc(definition::Column::Id)
            .one(&self.db)
            .await
            .db_err()?
            .map(Definition::from))
    }

    async fn list_definitions_by_code(&self, flow_code: &str) -> Result<Vec<Definition>> {
        Ok(definition::Entity::find()
            .filter(definition::Column::FlowCode.eq(flow_code))
            .order_by_asc(definition::Column::Id)
            .all(&self.db)
            .await
            .db_err()?
            .into_iter()
            .map(Definition::from)
            .collect())
    }

    async fn list_definitions(&self, query: &DefinitionQuery) -> Result<PageData<Definition>> {
        let mut find = definition::Entity::find();
        if let Some(code) = query.flow_code.as_deref().filter(|s| !s.is_empty()) {
            find = find.filter(definition::Column::FlowCode.like(format!("%{code}%")));
        }
        if let Some(name) = query.flow_name.as_deref().filter(|s| !s.is_empty()) {
            find = find.filter(definition::Column::FlowName.like(format!("%{name}%")));
        }
        if let Some(category) = query.category.as_deref().filter(|s| !s.is_empty()) {
            find = find.filter(definition::Column::Category.eq(category));
        }
        if let Some(status) = query.is_publish {
            find = find.filter(definition::Column::IsPublish.eq(status));
        }
        let (num, size) = page_offset(query.page_num, query.page_size);
        let paginator = find.order_by_desc(definition::Column::Id).paginate(&self.db, size);
        let total = paginator.num_items().await.db_err()?;
        let rows = paginator.fetch_page(num).await.db_err()?;
        Ok(PageData { rows: rows.into_iter().map(Definition::from).collect(), total })
    }

    async fn update_publish_status(&self, ids: &[i64], status: i32) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }
        definition::Entity::update_many()
            .col_expr(definition::Column::IsPublish, sea_orm::sea_query::Expr::value(status))
            .filter(definition::Column::Id.is_in(ids.to_vec()))
            .exec(&self.db)
            .await
            .db_err()?;
        Ok(())
    }

    // ---------------- 节点 ----------------

    async fn insert_nodes(&self, nodes: &mut [Node]) -> Result<()> {
        if nodes.is_empty() {
            return Ok(());
        }
        let mut models = Vec::with_capacity(nodes.len());
        for node in nodes.iter_mut() {
            let id = node.id.unwrap_or_else(|| self.next_id());
            node.id = Some(id);
            models.push(node_active(node, id));
        }
        node::Entity::insert_many(models).exec(&self.db).await.db_err()?;
        Ok(())
    }

    async fn delete_nodes_by_definition(&self, definition_ids: &[i64]) -> Result<()> {
        if definition_ids.is_empty() {
            return Ok(());
        }
        node::Entity::delete_many()
            .filter(node::Column::DefinitionId.is_in(definition_ids.to_vec()))
            .exec(&self.db)
            .await
            .db_err()?;
        Ok(())
    }

    async fn list_nodes(&self, definition_id: i64) -> Result<Vec<Node>> {
        Ok(node::Entity::find()
            .filter(node::Column::DefinitionId.eq(definition_id))
            .order_by_asc(node::Column::Id)
            .all(&self.db)
            .await
            .db_err()?
            .into_iter()
            .map(Node::from)
            .collect())
    }

    async fn get_node(&self, definition_id: i64, node_code: &str) -> Result<Option<Node>> {
        Ok(node::Entity::find()
            .filter(node::Column::DefinitionId.eq(definition_id))
            .filter(node::Column::NodeCode.eq(node_code))
            .one(&self.db)
            .await
            .db_err()?
            .map(Node::from))
    }

    async fn get_node_by_id(&self, id: i64) -> Result<Option<Node>> {
        Ok(node::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .db_err()?
            .map(Node::from))
    }

    async fn find_node_by_type(&self, definition_id: i64, node_type: i32) -> Result<Option<Node>> {
        Ok(node::Entity::find()
            .filter(node::Column::DefinitionId.eq(definition_id))
            .filter(node::Column::NodeType.eq(node_type))
            .order_by_asc(node::Column::Id)
            .one(&self.db)
            .await
            .db_err()?
            .map(Node::from))
    }

    async fn list_nodes_by_codes(&self, definition_id: i64, node_codes: &[String]) -> Result<Vec<Node>> {
        if node_codes.is_empty() {
            return Ok(Vec::new());
        }
        Ok(node::Entity::find()
            .filter(node::Column::DefinitionId.eq(definition_id))
            .filter(node::Column::NodeCode.is_in(node_codes.to_vec()))
            .all(&self.db)
            .await
            .db_err()?
            .into_iter()
            .map(Node::from)
            .collect())
    }

    // ---------------- 跳转 ----------------

    async fn insert_skips(&self, skips: &mut [Skip]) -> Result<()> {
        if skips.is_empty() {
            return Ok(());
        }
        let mut models = Vec::with_capacity(skips.len());
        for skip in skips.iter_mut() {
            let id = skip.id.unwrap_or_else(|| self.next_id());
            skip.id = Some(id);
            models.push(skip_active(skip, id));
        }
        skip::Entity::insert_many(models).exec(&self.db).await.db_err()?;
        Ok(())
    }

    async fn delete_skips_by_definition(&self, definition_ids: &[i64]) -> Result<()> {
        if definition_ids.is_empty() {
            return Ok(());
        }
        skip::Entity::delete_many()
            .filter(skip::Column::DefinitionId.is_in(definition_ids.to_vec()))
            .exec(&self.db)
            .await
            .db_err()?;
        Ok(())
    }

    async fn list_skips(&self, definition_id: i64) -> Result<Vec<Skip>> {
        Ok(skip::Entity::find()
            .filter(skip::Column::DefinitionId.eq(definition_id))
            .order_by_asc(skip::Column::Id)
            .all(&self.db)
            .await
            .db_err()?
            .into_iter()
            .map(Skip::from)
            .collect())
    }

    // ---------------- 流程实例 ----------------

    async fn insert_instance(&self, instance: &mut Instance) -> Result<()> {
        let id = instance.id.unwrap_or_else(|| self.next_id());
        instance.id = Some(id);
        instance::Entity::insert(instance_active(instance))
            .exec(&self.db)
            .await
            .db_err()?;
        Ok(())
    }

    async fn update_instance(&self, instance: &Instance) -> Result<()> {
        instance::Entity::update(instance_active(instance))
            .exec(&self.db)
            .await
            .db_err()?;
        Ok(())
    }

    async fn get_instance(&self, id: i64) -> Result<Option<Instance>> {
        Ok(instance::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .db_err()?
            .map(Instance::from))
    }

    async fn list_instances_by_definition(&self, definition_id: i64) -> Result<Vec<Instance>> {
        Ok(instance::Entity::find()
            .filter(instance::Column::DefinitionId.eq(definition_id))
            .all(&self.db)
            .await
            .db_err()?
            .into_iter()
            .map(Instance::from)
            .collect())
    }

    async fn list_instances_by_definitions(&self, definition_ids: &[i64]) -> Result<Vec<Instance>> {
        if definition_ids.is_empty() {
            return Ok(Vec::new());
        }
        Ok(instance::Entity::find()
            .filter(instance::Column::DefinitionId.is_in(definition_ids.to_vec()))
            .all(&self.db)
            .await
            .db_err()?
            .into_iter()
            .map(Instance::from)
            .collect())
    }

    async fn list_instances(&self, query: &InstanceQuery) -> Result<PageData<Instance>> {
        let mut find = instance::Entity::find();
        if let Some(id) = query.definition_id {
            find = find.filter(instance::Column::DefinitionId.eq(id));
        }
        if let Some(business_id) = query.business_id.as_deref().filter(|s| !s.is_empty()) {
            find = find.filter(instance::Column::BusinessId.eq(business_id));
        }
        if let Some(status) = query.flow_status.as_deref().filter(|s| !s.is_empty()) {
            find = find.filter(instance::Column::FlowStatus.eq(status));
        }
        if let Some(create_by) = query.create_by.as_deref().filter(|s| !s.is_empty()) {
            find = find.filter(instance::Column::CreateBy.eq(create_by));
        }
        let (num, size) = page_offset(query.page_num, query.page_size);
        let paginator = find.order_by_desc(instance::Column::Id).paginate(&self.db, size);
        let total = paginator.num_items().await.db_err()?;
        let rows = paginator.fetch_page(num).await.db_err()?;
        Ok(PageData { rows: rows.into_iter().map(Instance::from).collect(), total })
    }

    async fn delete_instances(&self, ids: &[i64]) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }
        instance::Entity::delete_many()
            .filter(instance::Column::Id.is_in(ids.to_vec()))
            .exec(&self.db)
            .await
            .db_err()?;
        Ok(())
    }

    // ---------------- 待办任务 ----------------

    async fn insert_tasks(&self, tasks: &mut [Task]) -> Result<()> {
        if tasks.is_empty() {
            return Ok(());
        }
        let mut models = Vec::with_capacity(tasks.len());
        for task in tasks.iter_mut() {
            let id = task.id.unwrap_or_else(|| self.next_id());
            task.id = Some(id);
            models.push(task_active(task, id));
        }
        task::Entity::insert_many(models).exec(&self.db).await.db_err()?;
        Ok(())
    }

    async fn delete_tasks(&self, ids: &[i64]) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }
        task::Entity::delete_many()
            .filter(task::Column::Id.is_in(ids.to_vec()))
            .exec(&self.db)
            .await
            .db_err()?;
        Ok(())
    }

    async fn delete_tasks_by_instances(&self, instance_ids: &[i64]) -> Result<()> {
        if instance_ids.is_empty() {
            return Ok(());
        }
        task::Entity::delete_many()
            .filter(task::Column::InstanceId.is_in(instance_ids.to_vec()))
            .exec(&self.db)
            .await
            .db_err()?;
        Ok(())
    }

    async fn get_task(&self, id: i64) -> Result<Option<Task>> {
        Ok(task::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .db_err()?
            .map(Task::from))
    }

    async fn list_tasks_by_instance(&self, instance_id: i64) -> Result<Vec<Task>> {
        Ok(task::Entity::find()
            .filter(task::Column::InstanceId.eq(instance_id))
            .order_by_asc(task::Column::Id)
            .all(&self.db)
            .await
            .db_err()?
            .into_iter()
            .map(Task::from)
            .collect())
    }

    async fn list_tasks_by_instance_and_nodes(&self, instance_id: i64, node_codes: &[String]) -> Result<Vec<Task>> {
        if node_codes.is_empty() {
            return Ok(Vec::new());
        }
        Ok(task::Entity::find()
            .filter(task::Column::InstanceId.eq(instance_id))
            .filter(task::Column::NodeCode.is_in(node_codes.to_vec()))
            .order_by_asc(task::Column::Id)
            .all(&self.db)
            .await
            .db_err()?
            .into_iter()
            .map(Task::from)
            .collect())
    }

    async fn list_tasks(&self, query: &TaskQuery) -> Result<PageData<Task>> {
        let mut find = task::Entity::find();
        if let Some(id) = query.instance_id {
            find = find.filter(task::Column::InstanceId.eq(id));
        }
        if let Some(id) = query.definition_id {
            find = find.filter(task::Column::DefinitionId.eq(id));
        }
        if let Some(code) = query.node_code.as_deref().filter(|s| !s.is_empty()) {
            find = find.filter(task::Column::NodeCode.eq(code));
        }
        if let Some(status) = query.flow_status.as_deref().filter(|s| !s.is_empty()) {
            find = find.filter(task::Column::FlowStatus.eq(status));
        }
        if let Some(handler) = query.handler.as_deref().filter(|s| !s.is_empty()) {
            let sub = Query::select()
                .column(user::Column::Associated)
                .from(user::Entity)
                .and_where(user::Column::ProcessedBy.eq(handler))
                .to_owned();
            find = find.filter(task::Column::Id.in_subquery(sub));
        }
        let (num, size) = page_offset(query.page_num, query.page_size);
        let paginator = find.order_by_desc(task::Column::Id).paginate(&self.db, size);
        let total = paginator.num_items().await.db_err()?;
        let rows = paginator.fetch_page(num).await.db_err()?;
        Ok(PageData { rows: rows.into_iter().map(Task::from).collect(), total })
    }

    // ---------------- 历史任务 ----------------

    async fn insert_his_tasks(&self, his_tasks: &mut [HisTask]) -> Result<()> {
        if his_tasks.is_empty() {
            return Ok(());
        }
        let mut models = Vec::with_capacity(his_tasks.len());
        for his in his_tasks.iter_mut() {
            let id = his.id.unwrap_or_else(|| self.next_id());
            his.id = Some(id);
            models.push(his_task_active(his, id));
        }
        his_task::Entity::insert_many(models).exec(&self.db).await.db_err()?;
        Ok(())
    }

    async fn get_his_task(&self, id: i64) -> Result<Option<HisTask>> {
        Ok(his_task::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .db_err()?
            .map(HisTask::from))
    }

    async fn list_his_tasks_by_instance(&self, instance_id: i64) -> Result<Vec<HisTask>> {
        Ok(his_task::Entity::find()
            .filter(his_task::Column::InstanceId.eq(instance_id))
            .order_by_asc(his_task::Column::Id)
            .all(&self.db)
            .await
            .db_err()?
            .into_iter()
            .map(HisTask::from)
            .collect())
    }

    async fn list_his_tasks_by_task(&self, task_id: i64) -> Result<Vec<HisTask>> {
        Ok(his_task::Entity::find()
            .filter(his_task::Column::TaskId.eq(task_id))
            .order_by_asc(his_task::Column::Id)
            .all(&self.db)
            .await
            .db_err()?
            .into_iter()
            .map(HisTask::from)
            .collect())
    }

    async fn list_his_tasks(&self, query: &HisTaskQuery) -> Result<PageData<HisTask>> {
        let mut find = his_task::Entity::find();
        if let Some(id) = query.instance_id {
            find = find.filter(his_task::Column::InstanceId.eq(id));
        }
        if let Some(id) = query.definition_id {
            find = find.filter(his_task::Column::DefinitionId.eq(id));
        }
        if let Some(id) = query.task_id {
            find = find.filter(his_task::Column::TaskId.eq(id));
        }
        if let Some(approver) = query.approver.as_deref().filter(|s| !s.is_empty()) {
            find = find.filter(his_task::Column::Approver.eq(approver));
        }
        if let Some(status) = query.flow_status.as_deref().filter(|s| !s.is_empty()) {
            find = find.filter(his_task::Column::FlowStatus.eq(status));
        }
        let (num, size) = page_offset(query.page_num, query.page_size);
        let paginator = find.order_by_desc(his_task::Column::Id).paginate(&self.db, size);
        let total = paginator.num_items().await.db_err()?;
        let rows = paginator.fetch_page(num).await.db_err()?;
        Ok(PageData { rows: rows.into_iter().map(HisTask::from).collect(), total })
    }

    async fn delete_his_tasks_by_instances(&self, instance_ids: &[i64]) -> Result<()> {
        if instance_ids.is_empty() {
            return Ok(());
        }
        his_task::Entity::delete_many()
            .filter(his_task::Column::InstanceId.is_in(instance_ids.to_vec()))
            .exec(&self.db)
            .await
            .db_err()?;
        Ok(())
    }

    // ---------------- 流程用户 ----------------

    async fn insert_users(&self, users: &mut [User]) -> Result<()> {
        if users.is_empty() {
            return Ok(());
        }
        let mut models = Vec::with_capacity(users.len());
        for user in users.iter_mut() {
            let id = user.id.unwrap_or_else(|| self.next_id());
            user.id = Some(id);
            models.push(user_active(user, id));
        }
        user::Entity::insert_many(models).exec(&self.db).await.db_err()?;
        Ok(())
    }

    async fn delete_users_by_tasks(&self, task_ids: &[i64]) -> Result<()> {
        if task_ids.is_empty() {
            return Ok(());
        }
        user::Entity::delete_many()
            .filter(user::Column::Associated.is_in(task_ids.to_vec()))
            .exec(&self.db)
            .await
            .db_err()?;
        Ok(())
    }

    async fn delete_user(&self, id: i64) -> Result<()> {
        user::Entity::delete_by_id(id).exec(&self.db).await.db_err()?;
        Ok(())
    }

    async fn list_users_by_task(&self, task_id: i64) -> Result<Vec<User>> {
        Ok(user::Entity::find()
            .filter(user::Column::Associated.eq(task_id))
            .order_by_asc(user::Column::Id)
            .all(&self.db)
            .await
            .db_err()?
            .into_iter()
            .map(User::from)
            .collect())
    }

    async fn list_users_by_task_and_types(&self, task_id: i64, types: &[String]) -> Result<Vec<User>> {
        if types.is_empty() {
            return self.list_users_by_task(task_id).await;
        }
        Ok(user::Entity::find()
            .filter(user::Column::Associated.eq(task_id))
            .filter(user::Column::Type.is_in(types.to_vec()))
            .order_by_asc(user::Column::Id)
            .all(&self.db)
            .await
            .db_err()?
            .into_iter()
            .map(User::from)
            .collect())
    }

    async fn find_user(&self, task_id: i64, processed_by: &str, user_type: &str) -> Result<Option<User>> {
        Ok(user::Entity::find()
            .filter(user::Column::Associated.eq(task_id))
            .filter(user::Column::ProcessedBy.eq(processed_by))
            .filter(user::Column::Type.eq(user_type))
            .one(&self.db)
            .await
            .db_err()?
            .map(User::from))
    }

    async fn list_users_by_tasks_and_types(&self, task_ids: &[i64], types: &[String]) -> Result<Vec<User>> {
        if task_ids.is_empty() || types.is_empty() {
            return Ok(Vec::new());
        }
        Ok(user::Entity::find()
            .filter(user::Column::Associated.is_in(task_ids.to_vec()))
            .filter(user::Column::Type.is_in(types.to_vec()))
            .order_by_asc(user::Column::Id)
            .all(&self.db)
            .await
            .db_err()?
            .into_iter()
            .map(User::from)
            .collect())
    }

    // ---------------- 表单 ----------------

    async fn get_form(&self, id: i64) -> Result<Option<Form>> {
        Ok(form::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .db_err()?
            .map(Form::from))
    }

    async fn list_forms(&self, is_publish: Option<i32>) -> Result<Vec<Form>> {
        let mut find = form::Entity::find();
        if let Some(status) = is_publish {
            find = find.filter(form::Column::IsPublish.eq(status));
        }
        Ok(find
            .order_by_desc(form::Column::Id)
            .all(&self.db)
            .await
            .db_err()?
            .into_iter()
            .map(Form::from)
            .collect())
    }

    async fn insert_form(&self, form: &mut Form) -> Result<()> {
        let id = form.id.unwrap_or_else(|| self.next_id());
        form.id = Some(id);
        form::Entity::insert(form_active(form)).exec(&self.db).await.db_err()?;
        Ok(())
    }

    async fn update_form(&self, form: &Form) -> Result<()> {
        form::Entity::update(form_active(form)).exec(&self.db).await.db_err()?;
        Ok(())
    }
}
