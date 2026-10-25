//! 流程定义的维护。对应 Java `DefServiceImpl` + `FlowConfigUtil`。
//!
//! 设计器保存时提交的是整份 JSON（定义 + 节点 + 连线），
//! 所以这里的写操作统一走「先校验、再整体覆盖」的策略。

use std::collections::{HashMap, HashSet};

use super::node::{end_node, start_node};
use super::FlowEngine;
use crate::constant::exception;
use crate::dto::{DefJson, FlowCombine};
use crate::entity::{Definition, Node, Skip};
use crate::enums::{ActivityStatus, NodeType, PublishStatus, SkipType};
use crate::error::{FlowError, Result};

impl FlowEngine {
    /// 流程定义 + 节点 + 连线，节点上挂好各自的出线（对应 Java `getAllDataDefinition`）
    pub async fn get_all_data_definition(&self, id: i64) -> Result<Definition> {
        let mut definition = self
            .store
            .get_definition(id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_DEF))?;
        let mut nodes = self.store.list_nodes(id).await?;
        let skips = self.store.list_skips(id).await?;
        let mut skip_map: HashMap<String, Vec<Skip>> = HashMap::new();
        for skip in skips {
            skip_map.entry(skip.now_node_code.clone()).or_default().push(skip);
        }
        for node in nodes.iter_mut() {
            node.skip_list = skip_map.remove(&node.node_code).unwrap_or_default();
        }
        definition.node_list = nodes;
        Ok(definition)
    }

    /// 设计器读取：流程定义 JSON（对应 Java `queryDesign`）
    pub async fn query_design(&self, id: i64) -> Result<DefJson> {
        let definition = self.get_all_data_definition(id).await?;
        Ok(DefJson::from_definition(&definition))
    }

    /// 导出流程定义 JSON（导出时不带发布状态）
    pub async fn export_json(&self, id: i64) -> Result<String> {
        let mut def_json = self.query_design(id).await?;
        def_json.is_publish = None;
        serde_json::to_string(&def_json).map_err(|e| FlowError::biz(format!("流程定义序列化失败: {e}")))
    }

    /// 保存设计器 JSON（对应 Java `saveDef`）
    ///
    /// `only_node_skip` 为真时只覆盖节点与连线，不更新定义本身。
    pub async fn save_def(&self, def_json: &DefJson, only_node_skip: bool) -> Result<Definition> {
        let mut combine = def_json.to_combine();
        let definition_id = combine.definition.id;
        let is_new = definition_id.is_none();

        if is_new {
            combine.definition.version = self.get_new_version(&combine.definition).await?;
            combine.definition.id = Some(self.next_id());
        }
        self.check_flow_legal(&mut combine)?;

        let definition = combine.definition.clone();
        let id = definition.id.unwrap_or_default();

        if is_new {
            let mut def = definition.clone();
            def.create_time = Some(self.now());
            def.create_by = self.operator();
            def.update_time = Some(self.now());
            def.update_by = self.operator();
            self.store.insert_definition(&mut def).await?;
        } else {
            if !only_node_skip {
                let mut def = definition.clone();
                def.id = Some(id);
                def.update_time = Some(self.now());
                def.update_by = self.operator();
                self.store.update_definition(&def).await?;
            }
            self.store.delete_nodes_by_definition(&[id]).await?;
            self.store.delete_skips_by_definition(&[id]).await?;
        }

        let mut nodes = combine.all_nodes;
        for node in nodes.iter_mut() {
            node.definition_id = id;
            node.version = Some(definition.version.clone());
            if node.node_ratio.as_deref().map(|v| v.is_empty()).unwrap_or(true) {
                node.node_ratio = Some("0".to_string());
            }
            node.create_time = node.create_time.or_else(|| Some(self.now()));
            node.create_by = node.create_by.clone().or_else(|| self.operator());
            node.skip_list.clear();
        }
        let mut skips = combine.all_skips;
        for skip in skips.iter_mut() {
            skip.definition_id = id;
            skip.create_time = skip.create_time.or_else(|| Some(self.now()));
            skip.create_by = skip.create_by.clone().or_else(|| self.operator());
        }
        self.store.insert_nodes(&mut nodes).await?;
        self.store.insert_skips(&mut skips).await?;

        Ok(def_json.to_definition())
    }

    /// 导入设计器 JSON（对应 Java `importJson`），总是生成新版本
    pub async fn import_json(&self, json: &str) -> Result<Definition> {
        let def_json: DefJson =
            serde_json::from_str(json).map_err(|e| FlowError::biz(format!("流程定义解析失败: {e}")))?;
        self.import_def(&def_json).await
    }

    /// 导入流程定义：清空 id / 发布状态后重新入库
    pub async fn import_def(&self, def_json: &DefJson) -> Result<Definition> {
        let mut def_json = def_json.clone();
        def_json.id = None;
        def_json.is_publish = None;
        self.save_def(&def_json, false).await
    }

    /// 直接把已结构化的定义 + 节点 + 连线落库（对应 Java `insertFlow`）
    pub async fn insert_flow(
        &self,
        mut definition: Definition,
        mut nodes: Vec<Node>,
        mut skips: Vec<Skip>,
    ) -> Result<Definition> {
        definition.version = self.get_new_version(&definition).await?;
        if definition.id.is_none() {
            definition.id = Some(self.next_id());
        }
        definition.create_time = Some(self.now());
        definition.update_time = Some(self.now());
        let id = definition.id.unwrap_or_default();
        self.store.insert_definition(&mut definition).await?;
        for node in nodes.iter_mut() {
            node.definition_id = id;
            node.version = Some(definition.version.clone());
        }
        for skip in skips.iter_mut() {
            skip.definition_id = id;
        }
        self.store.insert_nodes(&mut nodes).await?;
        self.store.insert_skips(&mut skips).await?;
        Ok(definition)
    }

    /// 发布：同一 flowCode 下其他版本按是否已被使用降级为失效 / 未发布
    pub async fn publish(&self, id: i64) -> Result<()> {
        let nodes = self.store.list_nodes(id).await?;
        if nodes.is_empty() {
            return Err(FlowError::biz(exception::NOT_DRAW_FLOW_ERROR));
        }
        let definition = self
            .store
            .get_definition(id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_DEF))?;
        let definitions = self.store.list_definitions_by_code(&definition.flow_code).await?;
        let other_ids: Vec<i64> = definitions
            .iter()
            .filter(|d| d.id != Some(id) && d.is_publish == PublishStatus::PUBLISHED.0)
            .filter_map(|d| d.id)
            .collect();

        if !other_ids.is_empty() {
            let instances = self.store.list_instances_by_definitions(&other_ids).await?;
            let used: HashSet<i64> = instances.iter().map(|i| i.definition_id).collect();
            if !used.is_empty() {
                let used_ids: Vec<i64> = used.iter().copied().collect();
                self.store.update_publish_status(&used_ids, PublishStatus::EXPIRED.0).await?;
            }
            let unused_ids: Vec<i64> = other_ids.iter().copied().filter(|i| !used.contains(i)).collect();
            if !unused_ids.is_empty() {
                self.store.update_publish_status(&unused_ids, PublishStatus::UNPUBLISHED.0).await?;
            }
        }

        self.store.update_publish_status(&[id], PublishStatus::PUBLISHED.0).await
    }

    /// 取消发布：已被使用过则不允许
    pub async fn un_publish(&self, id: i64) -> Result<()> {
        let instances = self.store.list_instances_by_definition(id).await?;
        if !instances.is_empty() {
            return Err(FlowError::biz(exception::EXIST_START_TASK));
        }
        self.store.update_publish_status(&[id], PublishStatus::UNPUBLISHED.0).await
    }

    /// 复制流程定义（生成新版本、新 id，节点与连线一并复制）
    pub async fn copy_def(&self, id: i64) -> Result<Definition> {
        let source = self.get_all_data_definition(id).await?;
        let mut definition = Definition {
            id: None,
            is_publish: PublishStatus::UNPUBLISHED.0,
            activity_status: ActivityStatus::ACTIVITY.0,
            create_time: Some(self.now()),
            create_by: self.operator(),
            update_time: Some(self.now()),
            update_by: self.operator(),
            node_list: Vec::new(),
            ..source.clone()
        };
        definition.version = self.get_new_version(&definition).await?;
        definition.id = Some(self.next_id());
        let new_id = definition.id.unwrap_or_default();

        let mut nodes: Vec<Node> = source.node_list.clone();
        let mut skips = Vec::new();
        for node in nodes.iter_mut() {
            node.id = Some(self.next_id());
            node.definition_id = new_id;
            node.version = Some(definition.version.clone());
            for skip in node.skip_list.iter_mut() {
                skip.id = Some(self.next_id());
                skip.definition_id = new_id;
            }
            skips.extend(node.skip_list.clone());
            node.skip_list.clear();
        }
        self.store.insert_definition(&mut definition).await?;
        self.store.insert_nodes(&mut nodes).await?;
        self.store.insert_skips(&mut skips).await?;
        Ok(definition)
    }

    /// 激活流程定义
    pub async fn active_def(&self, id: i64) -> Result<()> {
        let mut definition = self
            .store
            .get_definition(id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_DEF))?;
        if ActivityStatus::is_activity(definition.activity_status) {
            return Err(FlowError::biz(exception::DEFINITION_ALREADY_ACTIVITY));
        }
        definition.activity_status = ActivityStatus::ACTIVITY.0;
        definition.update_time = Some(self.now());
        definition.update_by = self.operator();
        self.store.update_definition(&definition).await
    }

    /// 挂起流程定义
    pub async fn un_active_def(&self, id: i64) -> Result<()> {
        let mut definition = self
            .store
            .get_definition(id)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_DEF))?;
        if ActivityStatus::is_suspended(definition.activity_status) {
            return Err(FlowError::biz(exception::DEFINITION_ALREADY_SUSPENDED));
        }
        definition.activity_status = ActivityStatus::SUSPENDED.0;
        definition.update_time = Some(self.now());
        definition.update_by = self.operator();
        self.store.update_definition(&definition).await
    }

    /// 删除流程定义：已有实例的定义不允许删除
    pub async fn remove_def(&self, ids: &[i64]) -> Result<()> {
        for id in ids {
            let instances = self.store.list_instances_by_definition(*id).await?;
            if !instances.is_empty() {
                return Err(FlowError::biz(exception::EXIST_START_TASK));
            }
        }
        self.store.delete_nodes_by_definition(ids).await?;
        self.store.delete_skips_by_definition(ids).await?;
        self.store.delete_definitions(ids).await
    }

    /// 计算下一个版本号：纯数字取最大值 +1，否则在最新非数字版本后加 `_1`
    pub async fn get_new_version(&self, definition: &Definition) -> Result<String> {
        let definitions = self.store.list_definitions_by_code(&definition.flow_code).await?;
        let mut highest = 0i32;
        let mut latest_non_numeric: Option<(i64, String)> = None;
        for other in &definitions {
            if other.flow_code != definition.flow_code {
                continue;
            }
            match other.version.parse::<i32>() {
                Ok(v) if v > highest => highest = v,
                Ok(_) => {}
                Err(_) => {
                    let ts = other
                        .create_time
                        .map(|t| t.and_utc().timestamp_millis())
                        .unwrap_or(i64::MIN);
                    if latest_non_numeric.as_ref().map(|(old, _)| ts > *old).unwrap_or(true) {
                        latest_non_numeric = Some((ts, other.version.clone()));
                    }
                }
            }
        }
        Ok(if highest > 0 {
            (highest + 1).to_string()
        } else if let Some((_, version)) = latest_non_numeric {
            format!("{version}_1")
        } else {
            "1".to_string()
        })
    }

    /// 已发布的流程定义
    pub async fn get_publish_by_flow_code(&self, flow_code: &str) -> Result<Definition> {
        self.store
            .find_definition(flow_code, Some(PublishStatus::PUBLISHED.0))
            .await?
            .ok_or_else(|| FlowError::biz(exception::NOT_FOUND_DEF))
    }

    /// 设计器 JSON 结构校验 + 补全 definitionId / version / 节点类型
    pub fn structure_flow(&self, def_json: &DefJson) -> Result<FlowCombine> {
        let mut combine = def_json.to_combine();
        let definition_id = match combine.definition.id {
            Some(id) => id,
            None => {
                let id = self.next_id();
                combine.definition.id = Some(id);
                id
            }
        };
        let flow_name = combine.definition.flow_name.clone();
        if combine.definition.flow_code.is_empty() {
            return Err(FlowError::biz(format!("【{flow_name}】流程flowCode为空!")));
        }
        combine.definition.is_publish = PublishStatus::UNPUBLISHED.0;
        combine.definition.update_time = Some(self.now());

        let version = combine.definition.version.clone();
        let mut start_num = 0;
        let mut node_codes = HashSet::new();
        for node in combine.all_nodes.iter_mut() {
            node.definition_id = definition_id;
            init_node_and_condition(node, definition_id, &version)?;
            start_num = check_start_and_same(node, start_num, &flow_name, &mut node_codes)?;
        }
        let type_map: HashMap<String, i32> = combine
            .all_nodes
            .iter()
            .map(|n| (n.node_code.clone(), n.node_type))
            .collect();
        for skip in combine.all_skips.iter_mut() {
            skip.next_node_type = type_map.get(&skip.next_node_code).copied();
        }
        if start_num == 0 {
            return Err(FlowError::biz(format!("[{flow_name}]{}", exception::LOST_START_NODE)));
        }
        check_skip_node(&combine.all_skips)?;
        validate_exist_dest_node(&combine.all_skips, &node_codes)?;
        Ok(combine)
    }

    /// 保存前的合法性校验（对应 Java `checkFlowLegal`）
    pub fn check_flow_legal(&self, combine: &mut FlowCombine) -> Result<()> {
        let flow_name = combine.definition.flow_name.clone();
        if combine.definition.flow_code.is_empty() {
            return Err(FlowError::biz(format!("【{flow_name}】流程flowCode为空!")));
        }
        let definition_id = combine.definition.id.unwrap_or_default();
        let version = combine.definition.version.clone();

        let mut skip_map: HashMap<String, Vec<Skip>> = HashMap::new();
        for skip in &combine.all_skips {
            skip_map.entry(skip.now_node_code.clone()).or_default().push(skip.clone());
        }
        let mut start_num = 0;
        let mut node_codes = HashSet::new();
        for node in combine.all_nodes.iter_mut() {
            node.skip_list = skip_map.remove(&node.node_code).unwrap_or_default();
            init_node_and_condition(node, definition_id, &version)?;
            start_num = check_start_and_same(node, start_num, &flow_name, &mut node_codes)?;
        }
        if !skip_map.is_empty() {
            return Err(FlowError::biz(format!("[{flow_name}]{}", exception::FLOW_HAVE_USELESS_SKIP)));
        }
        if start_num == 0 {
            return Err(FlowError::biz(format!("[{flow_name}]{}", exception::LOST_START_NODE)));
        }
        let type_map: HashMap<String, i32> = combine
            .all_nodes
            .iter()
            .map(|n| (n.node_code.clone(), n.node_type))
            .collect();
        for skip in combine.all_skips.iter_mut() {
            skip.next_node_type = type_map.get(&skip.next_node_code).copied();
        }
        check_skip_node(&combine.all_skips)?;
        validate_exist_dest_node(&combine.all_skips, &node_codes)?;
        Ok(())
    }

    /// 校验定义中存在唯一的开始节点与结束节点
    pub fn check_start_end(&self, combine: &FlowCombine) -> Result<()> {
        start_node(combine)?;
        end_node(combine)?;
        Ok(())
    }
}

/// 补全节点与其出线的公共字段，并做单节点内的重复校验。
pub fn init_node_and_condition(node: &mut Node, definition_id: i64, version: &str) -> Result<()> {
    let node_name = node.node_name.clone().unwrap_or_default();
    let node_type = node.node_type;
    if !NodeType::is_end(node_type) && node.skip_list.is_empty() {
        return Err(FlowError::biz(exception::MUST_SKIP));
    }
    if node.node_code.is_empty() {
        return Err(FlowError::biz(format!("[{node_name}]{}", exception::LOST_NODE_CODE)));
    }
    node.version = Some(version.to_string());
    node.definition_id = definition_id;

    let mut between_set: HashSet<String> = HashSet::new();
    let mut gateway_set: HashSet<String> = HashSet::new();
    let mut skip_num = 0;
    for skip in node.skip_list.iter_mut() {
        if NodeType::is_start(node_type) {
            skip_num += 1;
            if skip_num > 1 {
                return Err(FlowError::biz(format!("[{node_name}]{}", exception::MUL_START_SKIP)));
            }
        }
        if skip.next_node_code.is_empty() {
            return Err(FlowError::biz(format!("【{node_name}】{}", exception::LOST_DEST_NODE)));
        }
        skip.definition_id = definition_id;
        skip.now_node_type = Some(node_type);

        let duplicate = if NodeType::is_gateway_serial(node_type) {
            let target = format!("{}:{}", skip.skip_condition.clone().unwrap_or_default(), skip.next_node_code);
            let ok = gateway_set.insert(target);
            if !ok {
                return Err(FlowError::biz(format!("[{node_name}]{}", exception::SAME_CONDITION_NODE)));
            }
            continue;
        } else if NodeType::is_gateway_parallel(node_type) {
            let ok = gateway_set.insert(skip.next_node_code.clone());
            if !ok {
                return Err(FlowError::biz(format!("[{node_name}]{}", exception::SAME_DEST_NODE)));
            }
            continue;
        } else {
            format!("{}:{}", skip.skip_type.clone().unwrap_or_default(), skip.next_node_code)
        };
        if !between_set.insert(duplicate) {
            return Err(FlowError::biz(format!("[{node_name}]{}", exception::SAME_CONDITION_VALUE)));
        }
    }
    Ok(())
}

/// 统计开始节点个数，并校验节点编码唯一
pub fn check_start_and_same(
    node: &Node,
    start_num: i32,
    flow_name: &str,
    node_codes: &mut HashSet<String>,
) -> Result<i32> {
    let mut start_num = start_num;
    if NodeType::is_start(node.node_type) {
        start_num += 1;
        if start_num > 1 {
            return Err(FlowError::biz(format!("[{flow_name}]{}", exception::MUL_START_NODE)));
        }
    }
    if !node_codes.insert(node.node_code.clone()) {
        return Err(FlowError::biz(format!("【{flow_name}】{}", exception::SAME_NODE_CODE)));
    }
    Ok(start_num)
}

/// 校验：不能同时通过或退回到多个中间节点（必须先走网关）
pub fn check_skip_node(all_skips: &[Skip]) -> Result<()> {
    let mut grouped: HashMap<&str, Vec<&Skip>> = HashMap::new();
    for skip in all_skips {
        grouped.entry(skip.now_node_code.as_str()).or_default().push(skip);
    }
    for skips in grouped.values() {
        let mut pass_num = 0;
        let mut reject_num = 0;
        for skip in skips {
            let now_type = skip.now_node_type.unwrap_or(-1);
            let next_type = skip.next_node_type.unwrap_or(-1);
            if NodeType::is_between(now_type) && NodeType::is_between(next_type) {
                if SkipType::is_pass(skip.skip_type.as_deref().unwrap_or("")) {
                    pass_num += 1;
                } else {
                    reject_num += 1;
                }
            }
        }
        if pass_num > 1 || reject_num > 1 {
            return Err(FlowError::biz(exception::MUL_SKIP_BETWEEN));
        }
    }
    Ok(())
}

/// 校验连线的目标节点都存在
pub fn validate_exist_dest_node(all_skips: &[Skip], node_codes: &HashSet<String>) -> Result<()> {
    for skip in all_skips {
        if !node_codes.contains(&skip.next_node_code) {
            return Err(FlowError::biz(format!("【{}】{}", skip.next_node_code, exception::NULL_NODE_CODE)));
        }
    }
    Ok(())
}
