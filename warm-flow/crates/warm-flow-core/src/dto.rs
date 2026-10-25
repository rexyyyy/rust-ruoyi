//! 传输对象。对应 Java 版 `org.dromara.warm.flow.core.dto`。
//!
//! `DefJson` / `NodeJson` / `SkipJson` 直接与 warm-flow 设计器交换数据，
//! 因此字段名保持前端约定的 camelCase。

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::entity::{Definition, Instance, Node, Skip};
use crate::enums::ModelEnum;

/// 兼容 Java 设计器导出的 JSON：`nodeType` 既可能是数字 `0`，也可能是字符串 `"0"`。
///
/// warm-flow 官方导出文件（如 RuoYi 示例的 leave1.json）统一用字符串，
/// 本设计器输出数字，两种都要能吃下。
fn de_i32_lenient<'de, D>(de: D) -> std::result::Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum Raw {
        Num(i64),
        Text(String),
    }
    match Raw::deserialize(de)? {
        Raw::Num(n) => Ok(n as i32),
        Raw::Text(s) => s.trim().parse::<i32>().map_err(serde::de::Error::custom),
    }
}

/// 工作流内置参数，对应 Java `FlowParams`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowParams {
    /// 流程编码（开启流程时必填）
    pub flow_code: Option<String>,
    /// 当前办理人唯一标识（用户 id）
    pub handler: String,
    /// 指定跳转节点编码（任意跳转 / 退回指定节点）
    pub node_code: Option<String>,
    /// 当前用户权限标识，与节点预设办理人求交集判断权限
    pub permission_flag: Vec<String>,
    /// PASS / REJECT / NONE
    pub skip_type: Option<String>,
    /// 审批意见
    pub message: Option<String>,
    /// 流程变量
    pub variable: Map<String, Value>,
    /// 自定义流程状态
    pub flow_status: Option<String>,
    /// 自定义历史任务状态
    pub his_status: Option<String>,
    /// 实例激活状态
    pub activity_status: Option<i32>,
    /// 协作方式
    pub cooperate_type: Option<i32>,
    /// 业务扩展字段
    pub ext: Option<String>,
    /// 历史任务业务扩展字段
    pub his_task_ext: Option<String>,
    /// 加签 / 转办 / 委托的处理人
    pub add_handlers: Vec<String>,
    /// 减签 / 委托回收的处理人
    pub reduction_handlers: Vec<String>,
    /// 忽略权限校验
    pub ignore: bool,
    /// 忽略委派处理
    pub ignore_depute: bool,
    /// 忽略会签票签处理
    pub ignore_cooperate: bool,
    /// 指定下个任务的办理人
    pub next_handler: Vec<String>,
    /// 下个任务处理人是否追加
    pub next_handler_append: bool,
}

impl FlowParams {
    pub fn build() -> Self {
        Self::default()
    }

    pub fn flow_code(mut self, v: impl Into<String>) -> Self {
        self.flow_code = Some(v.into());
        self
    }

    pub fn handler(mut self, v: impl Into<String>) -> Self {
        self.handler = v.into();
        self
    }

    pub fn node_code(mut self, v: impl Into<String>) -> Self {
        self.node_code = Some(v.into());
        self
    }

    pub fn permission_flag(mut self, v: Vec<String>) -> Self {
        self.permission_flag = v;
        self
    }

    pub fn skip_type(mut self, v: impl Into<String>) -> Self {
        self.skip_type = Some(v.into());
        self
    }

    pub fn message(mut self, v: impl Into<String>) -> Self {
        self.message = Some(v.into());
        self
    }

    pub fn variable(mut self, v: Map<String, Value>) -> Self {
        self.variable = v;
        self
    }

    pub fn flow_status(mut self, v: impl Into<String>) -> Self {
        self.flow_status = Some(v.into());
        self
    }

    pub fn his_status(mut self, v: impl Into<String>) -> Self {
        self.his_status = Some(v.into());
        self
    }

    pub fn activity_status(mut self, v: i32) -> Self {
        self.activity_status = Some(v);
        self
    }

    pub fn cooperate_type(mut self, v: i32) -> Self {
        self.cooperate_type = Some(v);
        self
    }

    pub fn ext(mut self, v: impl Into<String>) -> Self {
        self.ext = Some(v.into());
        self
    }

    pub fn his_task_ext(mut self, v: impl Into<String>) -> Self {
        self.his_task_ext = Some(v.into());
        self
    }

    pub fn add_handlers(mut self, v: Vec<String>) -> Self {
        self.add_handlers = v;
        self
    }

    pub fn reduction_handlers(mut self, v: Vec<String>) -> Self {
        self.reduction_handlers = v;
        self
    }

    pub fn ignore(mut self, v: bool) -> Self {
        self.ignore = v;
        self
    }

    pub fn ignore_depute(mut self, v: bool) -> Self {
        self.ignore_depute = v;
        self
    }

    pub fn ignore_cooperate(mut self, v: bool) -> Self {
        self.ignore_cooperate = v;
        self
    }

    pub fn next_handler(mut self, v: Vec<String>) -> Self {
        self.next_handler = v;
        self
    }

    pub fn next_handler_append(mut self, v: bool) -> Self {
        self.next_handler_append = v;
        self
    }

    /// 表单数据写入流程变量（key = formData）
    pub fn form_data(mut self, form_data: Map<String, Value>) -> Self {
        self.variable.insert(crate::constant::flow::FORM_DATA.to_string(), Value::Object(form_data));
        self
    }

    /// 流程变量 JSON 字符串
    pub fn variable_str(&self) -> String {
        serde_json::to_string(&self.variable).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn skip_type_or_empty(&self) -> &str {
        self.skip_type.as_deref().unwrap_or("")
    }
}

/// 设计器节点，对应 Java `NodeJson`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeJson {
    #[serde(deserialize_with = "de_i32_lenient")]
    pub node_type: i32,
    pub node_code: String,
    pub node_name: Option<String>,
    pub version: Option<String>,
    pub permission_flag: Option<String>,
    pub node_ratio: Option<String>,
    pub coordinate: Option<String>,
    pub any_node_skip: Option<String>,
    pub listener_type: Option<String>,
    pub listener_path: Option<String>,
    pub form_custom: Option<String>,
    pub form_path: Option<String>,
    pub ext: Option<String>,
    /// 办理状态：0 未办理 1 待办理 2 已办理
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext_map: Option<Map<String, Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_content: Option<Value>,
    #[serde(default)]
    pub skip_list: Vec<SkipJson>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
}

impl NodeJson {
    pub fn to_node(&self, definition_id: i64) -> Node {
        Node {
            node_type: self.node_type,
            definition_id,
            node_code: self.node_code.clone(),
            node_name: self.node_name.clone(),
            permission_flag: self.permission_flag.clone(),
            node_ratio: Some(self.node_ratio.clone().unwrap_or_else(|| "0".to_string())),
            coordinate: self.coordinate.clone(),
            any_node_skip: self.any_node_skip.clone(),
            listener_type: self.listener_type.clone(),
            listener_path: self.listener_path.clone(),
            form_custom: self.form_custom.clone(),
            form_path: self.form_path.clone(),
            ext: self.ext.clone(),
            create_by: self.create_by.clone(),
            update_by: self.update_by.clone(),
            version: self.version.clone(),
            skip_list: self.skip_list.iter().map(|s| s.to_skip(definition_id)).collect(),
            ..Default::default()
        }
    }

    pub fn from_node(node: &Node) -> Self {
        Self {
            node_type: node.node_type,
            node_code: node.node_code.clone(),
            node_name: node.node_name.clone(),
            version: node.version.clone(),
            permission_flag: node.permission_flag.clone(),
            node_ratio: node.node_ratio.clone(),
            coordinate: node.coordinate.clone(),
            any_node_skip: node.any_node_skip.clone(),
            listener_type: node.listener_type.clone(),
            listener_path: node.listener_path.clone(),
            form_custom: node.form_custom.clone(),
            form_path: node.form_path.clone(),
            ext: node.ext.clone(),
            status: None,
            ext_map: None,
            prompt_content: None,
            skip_list: node.skip_list.iter().map(SkipJson::from_skip).collect(),
            create_by: node.create_by.clone(),
            update_by: node.update_by.clone(),
        }
    }
}

/// 设计器连线，对应 Java `SkipJson`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkipJson {
    pub now_node_code: String,
    pub next_node_code: String,
    pub skip_name: Option<String>,
    pub skip_type: Option<String>,
    pub skip_condition: Option<String>,
    pub coordinate: Option<String>,
    /// 办理状态：0 未办理 1 待办理 2 已办理
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext_map: Option<Map<String, Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_content: Option<Value>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
}

impl SkipJson {
    pub fn to_skip(&self, definition_id: i64) -> Skip {
        Skip {
            definition_id,
            now_node_code: self.now_node_code.clone(),
            next_node_code: self.next_node_code.clone(),
            skip_name: self.skip_name.clone(),
            skip_type: self.skip_type.clone(),
            skip_condition: self.skip_condition.clone(),
            coordinate: self.coordinate.clone(),
            create_by: self.create_by.clone(),
            update_by: self.update_by.clone(),
            ..Default::default()
        }
    }

    pub fn from_skip(skip: &Skip) -> Self {
        Self {
            now_node_code: skip.now_node_code.clone(),
            next_node_code: skip.next_node_code.clone(),
            skip_name: skip.skip_name.clone(),
            skip_type: skip.skip_type.clone(),
            skip_condition: skip.skip_condition.clone(),
            coordinate: skip.coordinate.clone(),
            status: None,
            ext_map: None,
            prompt_content: None,
            create_by: skip.create_by.clone(),
            update_by: skip.update_by.clone(),
        }
    }

    /// 连线唯一键：起点:类型:条件:终点（与 Java `ChartServiceImpl#getSkipKey` 一致）
    pub fn key(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.now_node_code,
            self.skip_type.clone().unwrap_or_default(),
            self.skip_condition.clone().unwrap_or_default(),
            self.next_node_code
        )
    }
}

/// 流程定义 JSON（设计器存盘格式），对应 Java `DefJson`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefJson {
    pub id: Option<i64>,
    pub flow_code: Option<String>,
    pub flow_name: Option<String>,
    pub model_value: Option<String>,
    pub category: Option<String>,
    pub version: Option<String>,
    pub is_publish: Option<i32>,
    pub form_custom: Option<String>,
    pub form_path: Option<String>,
    pub listener_type: Option<String>,
    pub listener_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<Instance>,
    pub ext: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext_map: Option<Map<String, Value>>,
    #[serde(default)]
    pub node_list: Vec<NodeJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chart_status_color: Option<Vec<String>>,
    pub top_text: Option<String>,
    pub top_text_show: Option<bool>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_list: Option<Vec<Tree>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_path_list: Option<Vec<Tree>>,
}

impl Default for DefJson {
    fn default() -> Self {
        Self {
            id: None,
            flow_code: None,
            flow_name: None,
            model_value: Some(ModelEnum::CLASSICS.to_string()),
            category: None,
            version: None,
            is_publish: None,
            form_custom: Some(crate::enums::FormCustomEnum::N.to_string()),
            form_path: None,
            listener_type: None,
            listener_path: None,
            instance: None,
            ext: None,
            ext_map: None,
            node_list: Vec::new(),
            chart_status_color: None,
            top_text: None,
            top_text_show: Some(true),
            create_by: None,
            update_by: None,
            category_list: None,
            form_path_list: None,
        }
    }
}

impl DefJson {
    pub fn model(&self) -> String {
        match self.model_value.as_deref() {
            Some(v) if !v.is_empty() => v.to_string(),
            _ => ModelEnum::CLASSICS.to_string(),
        }
    }

    /// 由数据库实体组装设计器 JSON（对应 Java `DefJson.copyDef(Definition)`）
    pub fn from_definition(definition: &Definition) -> Self {
        let node_list = definition
            .node_list
            .iter()
            .map(|node| {
                let mut node_json = NodeJson::from_node(node);
                node_json.skip_list = node.skip_list.iter().map(SkipJson::from_skip).collect();
                node_json
            })
            .collect();

        Self {
            id: definition.id,
            flow_code: Some(definition.flow_code.clone()),
            flow_name: Some(definition.flow_name.clone()),
            model_value: Some(definition.model_value.clone()),
            category: definition.category.clone(),
            version: Some(definition.version.clone()),
            is_publish: Some(definition.is_publish),
            form_custom: definition.form_custom.clone(),
            form_path: definition.form_path.clone(),
            listener_type: definition.listener_type.clone(),
            listener_path: definition.listener_path.clone(),
            instance: None,
            ext: definition.ext.clone(),
            ext_map: None,
            node_list,
            chart_status_color: None,
            top_text: None,
            top_text_show: Some(true),
            create_by: definition.create_by.clone(),
            update_by: definition.update_by.clone(),
            category_list: None,
            form_path_list: None,
        }
    }

    /// 转换为领域实体（对应 Java `DefJson.copyDef(DefJson)`）
    pub fn to_definition(&self) -> Definition {
        let definition_id = self.id.unwrap_or_default();
        let node_list: Vec<Node> = self.node_list.iter().map(|n| n.to_node(definition_id)).collect();
        Definition {
            id: self.id,
            flow_code: self.flow_code.clone().unwrap_or_default(),
            flow_name: self.flow_name.clone().unwrap_or_default(),
            model_value: self.model(),
            category: self.category.clone(),
            version: self.version.clone().unwrap_or_default(),
            is_publish: self.is_publish.unwrap_or(0),
            form_custom: self.form_custom.clone(),
            form_path: self.form_path.clone(),
            activity_status: crate::enums::ActivityStatus::ACTIVITY.0,
            listener_type: self.listener_type.clone(),
            listener_path: self.listener_path.clone(),
            ext: self.ext.clone(),
            create_by: self.create_by.clone(),
            update_by: self.update_by.clone(),
            node_list,
            ..Default::default()
        }
    }

    /// 拆成 "定义 + 全部节点 + 全部跳转"（对应 Java `DefJson.copyCombine`）
    pub fn to_combine(&self) -> FlowCombine {
        let definition = self.to_definition();
        let mut all_nodes = Vec::new();
        let mut all_skips = Vec::new();
        for node in &definition.node_list {
            all_nodes.push(node.clone());
            all_skips.extend(node.skip_list.clone());
        }
        FlowCombine { definition, all_nodes, all_skips }
    }
}

/// 流程定义 + 全部节点 + 全部跳转，对应 Java `FlowCombine`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FlowCombine {
    pub definition: Definition,
    pub all_nodes: Vec<Node>,
    pub all_skips: Vec<Skip>,
}

impl FlowCombine {
    /// 根据节点编码索引节点
    pub fn node_map(&self) -> std::collections::HashMap<String, &Node> {
        self.all_nodes.iter().map(|n| (n.node_code.clone(), n)).collect()
    }

    /// 按节点编码取节点
    pub fn node(&self, node_code: &str) -> Option<&Node> {
        self.all_nodes.iter().find(|n| n.node_code == node_code)
    }

    /// 取某节点的出线
    pub fn skips_of(&self, node_code: &str) -> Vec<&Skip> {
        self.all_skips.iter().filter(|s| s.now_node_code == node_code).collect()
    }
}

/// 办理过程途径数据，用于渲染流程图，对应 Java `PathWayData`
#[derive(Debug, Clone, Default)]
pub struct PathWayData {
    pub def_id: Option<i64>,
    pub ins_id: Option<i64>,
    pub skip_type: Option<String>,
    /// 目标节点
    pub target_nodes: Vec<Node>,
    /// 已途径节点（渲染为已办理）
    pub path_way_nodes: Vec<Node>,
    /// 已途径连线
    pub path_way_skips: Vec<Skip>,
}

impl PathWayData {
    pub fn new(ins_id: Option<i64>, skip_type: Option<String>) -> Self {
        Self { ins_id, skip_type, ..Default::default() }
    }
}

/// 表单数据载体，对应 Java `FlowDto`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowDto {
    pub id: Option<i64>,
    pub form: Option<crate::entity::Form>,
    pub form_content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// 树节点（流程分类、表单路径），对应 Java `Tree`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tree {
    pub id: Option<String>,
    pub name: Option<String>,
    /// 父级 id，用于把扁平列表拼成树
    pub parent_id: Option<String>,
    pub label: Option<String>,
    pub value: Option<String>,
    #[serde(default)]
    pub children: Vec<Tree>,
}

impl Tree {
    pub fn leaf(id: impl Into<String>, name: impl Into<String>) -> Self {
        let id = id.into();
        let name = name.into();
        Self {
            label: Some(name.clone()),
            value: Some(id.clone()),
            id: Some(id),
            name: Some(name),
            ..Default::default()
        }
    }

    /// 把带 `parent_id` 的扁平列表组装成树（对应 Java `TreeUtil.buildTree`）
    pub fn build_tree(list: Vec<Tree>) -> Vec<Tree> {
        // 先按 parent_id 分组，避免 O(n^2) 查找
        let mut children_of: std::collections::HashMap<Option<String>, Vec<Tree>> =
            std::collections::HashMap::new();
        for mut item in list {
            // 约定 "0"/"-1"/空串 与 None 一样表示根节点（Java 侧常用 "0" 作为根）
            let parent = item.parent_id.take().filter(|p| {
                let p = p.trim();
                !p.is_empty() && p != "0" && p != "-1"
            });
            children_of.entry(parent).or_default().push(item);
        }
        Self::attach(None, &mut children_of)
    }

    fn attach(
        parent: Option<String>,
        children_of: &mut std::collections::HashMap<Option<String>, Vec<Tree>>,
    ) -> Vec<Tree> {
        let mut nodes = children_of.remove(&parent).unwrap_or_default();
        for node in nodes.iter_mut() {
            let key = node.id.clone();
            node.children = Self::attach(key, children_of);
        }
        nodes
    }
}
