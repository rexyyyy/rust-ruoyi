//! 流程图元数据。对应 Java `ChartServiceImpl`。
//!
//! 只做纯数据计算：把「途径节点 / 途径连线 / 目标节点」映射成设计器 JSON 中
//! 每个节点的办理状态（0 未办理、1 待办理、2 已办理），落库后前端直接渲染。

use std::collections::HashMap;

use crate::dto::{DefJson, NodeJson, PathWayData, SkipJson};
use crate::enums::{ChartStatus, NodeType, SkipType};

/// 开启流程时初始化流程图：途径节点/连线标记已办理，目标节点标记待办理（结束节点直接已办理）
pub fn apply_start_metadata(def_json: &mut DefJson, path_way: &PathWayData) {
    // 1. 全部节点、连线先置为未办理
    let mut node_map: HashMap<String, usize> = HashMap::new();
    for (idx, node) in def_json.node_list.iter_mut().enumerate() {
        node.status = Some(ChartStatus::NOT_DONE.0);
        node_map.insert(node.node_code.clone(), idx);
    }
    let mut skip_map: HashMap<String, (usize, usize)> = HashMap::new();
    for (node_idx, node) in def_json.node_list.iter_mut().enumerate() {
        for (skip_idx, skip) in node.skip_list.iter_mut().enumerate() {
            skip.status = Some(ChartStatus::NOT_DONE.0);
            skip_map.insert(skip.key(), (node_idx, skip_idx));
        }
    }

    // 2. 途径节点、连线置为已办理
    for node in &path_way.path_way_nodes {
        if let Some(&idx) = node_map.get(&node.node_code) {
            def_json.node_list[idx].status = Some(ChartStatus::DONE.0);
        }
    }
    for skip in &path_way.path_way_skips {
        let key = skip_key(skip);
        if let Some(&(node_idx, skip_idx)) = skip_map.get(&key) {
            def_json.node_list[node_idx].skip_list[skip_idx].status = Some(ChartStatus::DONE.0);
        }
    }

    // 3. 目标节点：结束节点已办理，其余待办理
    for node in &path_way.target_nodes {
        if let Some(&idx) = node_map.get(&node.node_code) {
            def_json.node_list[idx].status = Some(if NodeType::is_end(node.node_type) {
                ChartStatus::DONE.0
            } else {
                ChartStatus::TO_DO.0
            });
        }
    }
}

/// 审批过程中刷新流程图状态，对应 Java `ChartServiceImpl#skipMetadata`
pub fn apply_skip_metadata(def_json: &mut DefJson, path_way: &PathWayData) {
    let skip_type = path_way.skip_type.clone().unwrap_or_default();
    let is_pass = SkipType::is_pass(&skip_type);
    let is_reject = SkipType::is_reject(&skip_type);

    let mut node_map: HashMap<String, usize> = HashMap::new();
    for (idx, node) in def_json.node_list.iter().enumerate() {
        node_map.insert(node.node_code.clone(), idx);
    }
    let mut skip_map: HashMap<String, (usize, usize)> = HashMap::new();
    for (node_idx, node) in def_json.node_list.iter().enumerate() {
        for (skip_idx, skip) in node.skip_list.iter().enumerate() {
            skip_map.insert(skip.key(), (node_idx, skip_idx));
        }
    }

    // 途径节点与连线
    for node in &path_way.path_way_nodes {
        if let Some(&idx) = node_map.get(&node.node_code) {
            if is_pass {
                def_json.node_list[idx].status = Some(ChartStatus::DONE.0);
            } else if is_reject {
                def_json.node_list[idx].status = Some(ChartStatus::NOT_DONE.0);
            }
        }
    }
    for skip in &path_way.path_way_skips {
        let key = skip_key(skip);
        if let Some(&(node_idx, skip_idx)) = skip_map.get(&key) {
            if is_pass {
                def_json.node_list[node_idx].skip_list[skip_idx].status = Some(ChartStatus::DONE.0);
            } else if is_reject {
                def_json.node_list[node_idx].skip_list[skip_idx].status = Some(ChartStatus::NOT_DONE.0);
            }
        }
    }

    // 目标节点
    for node in &path_way.target_nodes {
        if let Some(&idx) = node_map.get(&node.node_code) {
            def_json.node_list[idx].status = Some(if NodeType::is_end(node.node_type) {
                ChartStatus::DONE.0
            } else {
                ChartStatus::TO_DO.0
            });
        }
    }

    // 退回时，目标节点之后的所有后继节点/连线都要重置为未办理
    if is_reject {
        let mut next_skip_map: HashMap<String, Vec<String>> = HashMap::new();
        for node in &def_json.node_list {
            for skip in &node.skip_list {
                if !SkipType::is_reject(skip.skip_type.as_deref().unwrap_or("")) {
                    next_skip_map
                        .entry(skip.now_node_code.clone())
                        .or_default()
                        .push(skip.next_node_code.clone());
                }
            }
        }
        for node in &path_way.target_nodes {
            reject_reset(&node.node_code, &next_skip_map, &mut def_json.node_list);
        }
    }

    // 流程结束时，把仍在待办理的节点清成未办理
    if path_way.target_nodes.iter().any(|n| NodeType::is_end(n.node_type)) {
        for node in def_json.node_list.iter_mut() {
            if node.status.map(ChartStatus::is_to_do).unwrap_or(false) {
                node.status = Some(ChartStatus::NOT_DONE.0);
            }
        }
    }
}

/// 流程图三原色，返回 `[r,g,b]` 字符串数组
pub fn chart_rgb(rgb: [[u8; 3]; 3]) -> Vec<String> {
    rgb.iter().map(|c| format!("{},{},{}", c[0], c[1], c[2])).collect()
}

/// 解析实例上的 defJson，失败时返回空结构（避免审批被历史脏数据阻塞）
pub fn parse_def_json(raw: Option<&str>) -> DefJson {
    raw.and_then(|s| serde_json::from_str::<DefJson>(s).ok()).unwrap_or_default()
}

/// 把设计器 JSON 序列化后写回实例
pub fn dump_def_json(def_json: &DefJson) -> String {
    serde_json::to_string(def_json).unwrap_or_else(|_| "{}".to_string())
}

fn skip_key(skip: &crate::entity::Skip) -> String {
    format!(
        "{}:{}:{}:{}",
        skip.now_node_code,
        skip.skip_type.clone().unwrap_or_default(),
        skip.skip_condition.clone().unwrap_or_default(),
        skip.next_node_code
    )
}

fn reject_reset(node_code: &str, next_skip_map: &HashMap<String, Vec<String>>, nodes: &mut [NodeJson]) {
    let Some(next_codes) = next_skip_map.get(node_code) else {
        return;
    };
    let next_codes = next_codes.clone();
    for next_code in next_codes {
        if let Some(node) = nodes.iter_mut().find(|n| n.node_code == next_code) {
            if !node.status.map(ChartStatus::is_not_done).unwrap_or(false) {
                node.status = Some(ChartStatus::NOT_DONE.0);
                reject_reset(&next_code, next_skip_map, nodes);
            }
        }
    }
}

/// 便捷方法：给节点/连线打状态（供后续业务扩展使用）
pub fn mark_status(node: &mut NodeJson, status: ChartStatus) {
    node.status = Some(status.0);
}

/// 便捷方法：给连线打状态
pub fn mark_skip_status(skip: &mut SkipJson, status: ChartStatus) {
    skip.status = Some(status.0);
}
