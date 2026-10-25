//! 纯函数层的冒烟测试：条件表达式、流程图三原色、状态判断、树构建。
//! 这些是流程流转的判定依据，不依赖数据库，适合做回归基线。

use serde_json::{json, Map, Value};

use warm_flow_core::chart;
use warm_flow_core::dto::Tree;
use warm_flow_core::enums::{ChartStatus, FlowStatus, NodeType, SkipType};
use warm_flow_core::expression::eval_condition;

fn vars(pairs: &[(&str, Value)]) -> Map<String, Value> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.clone())).collect()
}

#[test]
fn condition_operators() {
    let v = vars(&[("flag", json!(4)), ("name", json!("张三")), ("amount", json!(1200))]);

    assert!(eval_condition("eq@@flag|4", &v).unwrap());
    assert!(!eval_condition("eq@@flag|5", &v).unwrap());
    assert!(eval_condition("ne@@flag|5", &v).unwrap());
    assert!(eval_condition("gt@@flag|3", &v).unwrap());
    assert!(eval_condition("ge@@flag|4", &v).unwrap());
    assert!(eval_condition("lt@@flag|5", &v).unwrap());
    assert!(eval_condition("le@@flag|4", &v).unwrap());
    assert!(eval_condition("like@@name|张", &v).unwrap());
    assert!(eval_condition("notLike@@name|李", &v).unwrap());
    // default@@ 前缀只是复用普通条件表达式
    assert!(eval_condition("default@@gt@@amount|1000", &v).unwrap());
    // 缺省运算符按 eq 处理
    assert!(eval_condition("flag|4", &v).unwrap());
}

#[test]
fn condition_rejects_invalid_expression() {
    let v = vars(&[("flag", json!(4))]);
    assert!(eval_condition("", &v).is_err());
    // 缺少 | 分隔符
    assert!(eval_condition("eq@@flag", &v).is_err());
    // spel 未实现
    assert!(eval_condition("spel@@flag|4", &v).is_err());
}

#[test]
fn condition_uses_missing_variable_as_error() {
    let v = Map::new();
    // 与 Java AbstractConditionStrategy#preEval 一致：变量缺失直接报错，而不是当作空串比较
    let err = eval_condition("eq@@flag|4", &v).unwrap_err();
    assert!(matches!(err, warm_flow_core::error::FlowError::Biz(_)));

    // 变量存在但值为 null 时，按空串参与比较
    let mut v = Map::new();
    v.insert("flag".to_string(), Value::Null);
    assert!(!eval_condition("eq@@flag|4", &v).unwrap());
}

#[test]
fn chart_rgb_keeps_three_colors() {
    let rgb = chart::chart_rgb(ChartStatus::DEFAULT_RGB);
    assert_eq!(rgb.len(), 3);
    assert_eq!(rgb[0], "107,114,128");
    assert_eq!(rgb[1], "245,158,11");
    assert_eq!(rgb[2], "56,161,105");
}

#[test]
fn status_helpers() {
    assert!(SkipType::is_reject(SkipType::REJECT));
    assert!(!SkipType::is_reject(SkipType::PASS));
    assert!(NodeType::is_end(NodeType::END.key()));
    assert!(!NodeType::is_end(NodeType::BETWEEN.key()));
    assert!(FlowStatus::is_finished(FlowStatus::FINISHED));
    assert!(!FlowStatus::is_finished(FlowStatus::APPROVAL));
}

#[test]
fn build_tree_from_flat_list() {
    let mut root = Tree::leaf("1", "一级");
    root.parent_id = Some("0".to_string());
    let mut child = Tree::leaf("2", "二级");
    child.parent_id = Some("1".to_string());

    let tree = Tree::build_tree(vec![root, child]);
    assert_eq!(tree.len(), 1);
    assert_eq!(tree[0].children.len(), 1);
    assert_eq!(tree[0].children[0].name.as_deref(), Some("二级"));
}
