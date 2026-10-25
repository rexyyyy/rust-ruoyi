//! 节点流转算法。对应 Java `NodeServiceImpl`。
//!
//! 这一层全是纯计算：输入节点 / 跳转 / 流程变量，输出下一个节点集合，
//! 同时把途径的节点与连线写进 [`PathWayData`]，供流程图渲染使用。

use std::collections::{HashMap, HashSet};

use serde_json::{Map, Value};

use super::FlowEngine;
use crate::constant::{exception, flow as flow_cons};
use crate::dto::{FlowCombine, PathWayData};
use crate::entity::{Node, Skip};
use crate::enums::{NodeType, SkipType};
use crate::error::{FlowError, Result};

/// 便捷扩展：设计器 JSON / 流程定义都能转成 [`FlowCombine`]
pub trait FlowCombineExt {
    fn to_flow_combine(&self) -> FlowCombine;
}

/// 取开始节点
pub fn start_node(combine: &FlowCombine) -> Result<Node> {
    combine
        .all_nodes
        .iter()
        .find(|n| NodeType::is_start(n.node_type))
        .cloned()
        .ok_or_else(|| FlowError::biz(exception::LOST_START_NODE))
}

/// 取结束节点
pub fn end_node(combine: &FlowCombine) -> Result<Node> {
    combine
        .all_nodes
        .iter()
        .find(|n| NodeType::is_end(n.node_type))
        .cloned()
        .ok_or_else(|| FlowError::biz(exception::NULL_NODE_CODE))
}

/// 取某节点的全部出线
pub fn skips_of<'a>(combine: &'a FlowCombine, node_code: &str) -> Vec<&'a Skip> {
    combine.all_skips.iter().filter(|s| s.now_node_code == node_code).collect()
}

/// 按跳转类型挑一条出线：跳转类型为空视为通用线（对应 Java `getSkipByCheck`）
pub fn skip_by_check(combine: &FlowCombine, node_code: &str, skip_type: &str) -> Result<Skip> {
    skips_of(combine, node_code)
        .into_iter()
        .find(|s| {
            s.skip_type
                .as_deref()
                .map(|t| t.is_empty() || t.eq_ignore_ascii_case(skip_type))
                .unwrap_or(true)
        })
        .cloned()
        .ok_or_else(|| FlowError::biz(exception::NULL_SKIP_TYPE))
}

/// 计算下一个节点（对应 Java `NodeServiceImpl#getNextNode`）
///
/// `any_node_code` 为任意跳转目标；未指定时按跳转类型找出线。
/// 途径节点与连线会累加到 `path_way`。
pub fn get_next_node(
    now_node: &Node,
    any_node_code: Option<&str>,
    skip_type: &str,
    path_way: &mut PathWayData,
    combine: &FlowCombine,
) -> Result<Node> {
    if now_node.node_code.is_empty() {
        return Err(FlowError::biz(exception::LOST_NODE_CODE));
    }
    if skip_type.is_empty() {
        return Err(FlowError::biz(exception::NULL_CONDITION_VALUE));
    }
    path_way.path_way_nodes.push(now_node.clone());

    // 指定了跳转节点，或节点配置了任意跳转且当前是退回
    let target_code = any_node_code.filter(|c| !c.is_empty()).or_else(|| {
        now_node
            .any_node_skip
            .as_deref()
            .filter(|c| !c.is_empty() && SkipType::is_reject(skip_type))
    });
    if let Some(code) = target_code {
        let node = combine
            .node(code)
            .cloned()
            .ok_or_else(|| FlowError::biz(exception::NULL_NODE_CODE))?;
        // 生成待办时必须落到真实节点，网关没有办理人
        if NodeType::is_gateway(node.node_type) {
            return Err(FlowError::biz(exception::TAR_NOT_GATEWAY));
        }
        return Ok(node);
    }

    if skips_of(combine, &now_node.node_code).is_empty() {
        return Err(FlowError::biz(exception::NULL_DEST_NODE));
    }
    let next_skip = skip_by_check(combine, &now_node.node_code, skip_type)?;
    let next_node = combine
        .node(&next_skip.next_node_code)
        .cloned()
        .ok_or_else(|| FlowError::biz(exception::NULL_NODE_CODE))?;
    if NodeType::is_start(next_node.node_type) {
        return Err(FlowError::biz(exception::FIRST_FORBID_BACK));
    }
    path_way.path_way_nodes.push(next_node.clone());
    path_way.path_way_skips.push(next_skip);
    Ok(next_node)
}

/// 网关条件判定（对应 Java `getNextByCheckGateway`）
///
/// - 互斥网关：取第一条条件命中的连线，无条件的连线兜底；
/// - 并行网关：全部连线都走；
/// - 包容网关：无条件的连线都走，有条件的按表达式过滤。
pub fn get_next_by_check_gateway(
    variable: &Map<String, Value>,
    next_node: &Node,
    path_way: &mut PathWayData,
    combine: &FlowCombine,
) -> Result<Vec<Node>> {
    if !NodeType::is_gateway(next_node.node_type) {
        // 非网关节点就是最终落点，不做「途径」记录
        path_way.path_way_nodes.retain(|n| n.node_code != next_node.node_code);
        if NodeType::is_start(next_node.node_type) {
            return Err(FlowError::biz(exception::START_NODE_NOT_ALLOW_JUMP));
        }
        return Ok(vec![next_node.clone()]);
    }

    let gateway_skips = skips_of(combine, &next_node.node_code);
    if gateway_skips.is_empty() {
        return Err(FlowError::biz(exception::NULL_DEST_NODE));
    }

    let chosen: Vec<Skip> = if NodeType::is_gateway_serial(next_node.node_type) {
        let mut picked: Option<Skip> = None;
        for skip in &gateway_skips {
            match skip.skip_condition.as_deref() {
                Some(cond) if !cond.is_empty() => {
                    if crate::expression::eval_condition(cond, variable)? {
                        picked = Some((*skip).clone());
                        break;
                    }
                }
                _ => picked = Some((*skip).clone()),
            }
        }
        picked.into_iter().collect()
    } else if NodeType::is_gateway_inclusive(next_node.node_type) {
        let mut kept = Vec::new();
        for skip in &gateway_skips {
            match skip.skip_condition.as_deref() {
                Some(cond) if !cond.is_empty() => {
                    if crate::expression::eval_condition(cond, variable)? {
                        kept.push((*skip).clone());
                    }
                }
                _ => kept.push((*skip).clone()),
            }
        }
        kept
    } else {
        gateway_skips.iter().map(|s| (*s).clone()).collect()
    };

    if chosen.is_empty() {
        return Err(FlowError::biz(exception::NULL_CONDITION_VALUE_NODE));
    }

    let next_codes: HashSet<&str> = chosen.iter().map(|s| s.next_node_code.as_str()).collect();
    let next_nodes: Vec<Node> = combine
        .all_nodes
        .iter()
        .filter(|n| next_codes.contains(n.node_code.as_str()))
        .cloned()
        .collect();
    if next_nodes.is_empty() {
        return Err(FlowError::biz(exception::NOT_NODE_DATA));
    }
    path_way.path_way_nodes.extend(next_nodes.iter().cloned());
    path_way.path_way_skips.extend(chosen.iter().cloned());

    let mut result = Vec::new();
    for node in &next_nodes {
        result.extend(get_next_by_check_gateway(variable, node, path_way, combine)?);
    }
    Ok(result)
}

/// 计算下一批节点（对应 Java `getNextNodeList`）
pub fn get_next_node_list(
    now_node: &Node,
    any_node_code: Option<&str>,
    skip_type: &str,
    variable: &Map<String, Value>,
    path_way: &mut PathWayData,
    combine: &FlowCombine,
) -> Result<Vec<Node>> {
    let next_node = get_next_node(now_node, any_node_code, skip_type, path_way, combine)?;
    get_next_by_check_gateway(variable, &next_node, path_way, combine)
}

/// 前驱 / 后继节点链（对应 Java `prefixOrSuffixNodes`）
///
/// 只沿 `PASS` 连线走，并剔除网关节点；`direction` 取 `previous` 或 `suffix`。
pub fn prefix_or_suffix_nodes(now_node_code: &str, direction: &str, combine: &FlowCombine) -> Vec<Node> {
    let is_previous = flow_cons::PREVIOUS.eq_ignore_ascii_case(direction);
    let mut skip_map: HashMap<String, Vec<&Skip>> = HashMap::new();
    for skip in &combine.all_skips {
        if !SkipType::is_pass(skip.skip_type.as_deref().unwrap_or("")) {
            continue;
        }
        let key = if is_previous {
            skip.next_node_code.clone()
        } else {
            skip.now_node_code.clone()
        };
        skip_map.entry(key).or_default().push(skip);
    }

    let mut visited: HashSet<String> = HashSet::new();
    let mut codes: Vec<String> = Vec::new();
    collect_codes(&skip_map, now_node_code, is_previous, &mut visited, &mut codes);

    let mut nodes: Vec<Node> = codes
        .iter()
        .filter_map(|code| combine.node(code))
        .filter(|n| !NodeType::is_gateway(n.node_type))
        .cloned()
        .collect();
    // 与 Java 一致：先反转、按编码去重、再反转，保留最后出现的顺序
    nodes.reverse();
    let mut seen = HashSet::new();
    nodes.retain(|n| seen.insert(n.node_code.clone()));
    nodes.reverse();
    nodes
}

fn collect_codes(
    skip_map: &HashMap<String, Vec<&Skip>>,
    node_code: &str,
    is_previous: bool,
    visited: &mut HashSet<String>,
    result: &mut Vec<String>,
) {
    if !visited.insert(node_code.to_string()) {
        return;
    }
    let Some(skips) = skip_map.get(node_code) else {
        return;
    };
    for skip in skips {
        let next_code = if is_previous {
            skip.now_node_code.clone()
        } else {
            skip.next_node_code.clone()
        };
        if !result.contains(&next_code) {
            result.push(next_code.clone());
        }
        collect_codes(skip_map, &next_code, is_previous, visited, result);
    }
}

/// 解析节点 ext（`[{"code":"x","value":"y"}]`）为 map，对应 Java `getExt`
pub fn get_ext(node: &Node) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let Some(raw) = node.ext.as_deref().filter(|s| !s.is_empty()) else {
        return map;
    };
    let Ok(list) = serde_json::from_str::<Vec<Value>>(raw) else {
        return map;
    };
    for item in list {
        let code = item.get("code").and_then(Value::as_str).unwrap_or("");
        let value = item.get("value").and_then(Value::as_str).unwrap_or("");
        if !code.is_empty() && !value.is_empty() {
            map.insert(code.to_string(), value.to_string());
        }
    }
    map
}

impl FlowEngine {
    /// 流程定义的全部节点与连线
    pub async fn flow_combine_no_def(&self, definition_id: i64) -> Result<FlowCombine> {
        let all_nodes = self.store.list_nodes(definition_id).await?;
        let all_skips = self.store.list_skips(definition_id).await?;
        Ok(FlowCombine { all_nodes, all_skips, ..Default::default() })
    }

    /// 流程定义 + 全部节点 + 全部连线
    pub async fn flow_combine(&self, definition: &crate::entity::Definition) -> Result<FlowCombine> {
        let mut combine = self.flow_combine_no_def(definition.id.unwrap_or_default()).await?;
        combine.definition = definition.clone();
        Ok(combine)
    }

    /// 前驱节点（不含网关）
    pub async fn previous_node_list(&self, definition_id: i64, now_node_code: &str) -> Result<Vec<Node>> {
        let combine = self.flow_combine_no_def(definition_id).await?;
        Ok(prefix_or_suffix_nodes(now_node_code, flow_cons::PREVIOUS, &combine))
    }

    /// 后继节点（不含网关）
    pub async fn suffix_node_list(&self, definition_id: i64, now_node_code: &str) -> Result<Vec<Node>> {
        let combine = self.flow_combine_no_def(definition_id).await?;
        Ok(prefix_or_suffix_nodes(now_node_code, flow_cons::SUFFIX, &combine))
    }

    /// 开始节点
    pub async fn get_start_node(&self, definition_id: i64) -> Result<Node> {
        self.store
            .find_node_by_type(definition_id, NodeType::START.0)
            .await?
            .ok_or_else(|| FlowError::biz(exception::LOST_START_NODE))
    }

    /// 结束节点
    pub async fn get_end_node(&self, definition_id: i64) -> Result<Node> {
        self.store
            .find_node_by_type(definition_id, NodeType::END.0)
            .await?
            .ok_or_else(|| FlowError::biz(exception::NULL_NODE_CODE))
    }

    /// 第一个真实节点（开始节点的后继），用于开启流程
    pub async fn get_first_between_node(
        &self,
        definition_id: i64,
        variable: &Map<String, Value>,
    ) -> Result<Vec<Node>> {
        let combine = self.flow_combine_no_def(definition_id).await?;
        let start = start_node(&combine)?;
        let mut path_way = PathWayData::default();
        get_next_node_list(&start, None, SkipType::PASS, variable, &mut path_way, &combine)
    }
}
