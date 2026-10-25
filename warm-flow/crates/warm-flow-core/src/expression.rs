//! 表达式求值。对应 Java `ExpressionUtil` + `condition/*` + `DefaultHandlerStrategy`。
//!
//! 支持范围：
//! - 条件表达式：`eq@@flag|4`、`gt@@days|3`、`like@@name|张` 等 8 种；
//! - 办理人表达式：`${handler}`、`${approverList}`（变量可为标量或数组）；
//! - 票签表达式：`default@@...` 复用条件表达式解析，`spel` / `snel` 前缀返回未支持错误。
//!
//! 与 Java 版的差异：Java 允许业务方注册自定义策略（SPI），Rust 版保留了
//! [`register_condition`] 扩展点，但没有引入运行期反射。

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::sync::Arc;

use crate::constant::{exception, flow};
use crate::enums::ConditionType;
use crate::error::{FlowError, Result};
use crate::utils::math;

/// 条件表达式处理器
pub type ConditionFn = Arc<dyn Fn(&str, &str) -> Result<bool> + Send + Sync>;

static CONDITION_REGISTRY: Lazy<RwLock<HashMap<String, ConditionFn>>> = Lazy::new(|| {
    let mut map: HashMap<String, ConditionFn> = HashMap::new();
    map.insert(ConditionType::EQ.to_string(), Arc::new(|value, variable_value| {
        Ok(compare(value, variable_value) == 0)
    }));
    map.insert(ConditionType::NE.to_string(), Arc::new(|value, variable_value| {
        Ok(compare(value, variable_value) != 0)
    }));
    map.insert(ConditionType::GT.to_string(), Arc::new(|value, variable_value| {
        Ok(compare(value, variable_value) > 0)
    }));
    map.insert(ConditionType::GE.to_string(), Arc::new(|value, variable_value| {
        Ok(compare(value, variable_value) >= 0)
    }));
    map.insert(ConditionType::LT.to_string(), Arc::new(|value, variable_value| {
        Ok(compare(value, variable_value) < 0)
    }));
    map.insert(ConditionType::LE.to_string(), Arc::new(|value, variable_value| {
        Ok(compare(value, variable_value) <= 0)
    }));
    map.insert(ConditionType::LIKE.to_string(), Arc::new(|value, variable_value| {
        Ok(variable_value.contains(value))
    }));
    map.insert(ConditionType::NOT_LIKE.to_string(), Arc::new(|value, variable_value| {
        Ok(!variable_value.contains(value))
    }));
    RwLock::new(map)
});

/// 注册（或覆盖）条件表达式策略，等价 Java `ExpressionUtil.setExpression`
pub fn register_condition(kind: &str, handler: ConditionFn) {
    CONDITION_REGISTRY.write().insert(kind.to_string(), handler);
}

/// 执行条件表达式，如 `eq@@flag|4`
pub fn eval_condition(expression: &str, variable: &Map<String, Value>) -> Result<bool> {
    let expr = expression.trim();
    if expr.is_empty() {
        return Err(FlowError::biz(exception::NULL_CONDITION_VALUE));
    }
    // default@@xxx：与普通条件表达式同一套解析
    let expr = expr.strip_prefix(&format!("{}@@", flow::DEFAULT)).unwrap_or(expr);
    if expr.starts_with(&format!("{}@@", flow::SPEL)) || expr.starts_with(&format!("{}@@", flow::SNEL)) {
        return Err(FlowError::biz("Rust 版暂未支持 spel/snel 条件表达式，请改用内置条件表达式"));
    }

    let (kind, body) = match expr.split_once("@@") {
        Some((kind, body)) => (kind.trim(), body.trim()),
        None => (ConditionType::EQ, expr),
    };

    let (name, expect) = match body.split_once(flow::SPLIT_VERTICAL) {
        Some((name, value)) => (name.trim(), value.trim()),
        None => return Err(FlowError::biz(exception::NULL_CONDITION_VALUE)),
    };

    let actual = variable
        .get(name)
        .ok_or_else(|| FlowError::biz(exception::NULL_CONDITION_VALUE))?;
    let actual_str = value_to_string(actual);

    let handler = CONDITION_REGISTRY
        .read()
        .get(kind)
        .cloned()
        .ok_or_else(|| FlowError::biz(format!("未支持的条件表达式类型: {kind}")))?;
    handler(expect, &actual_str)
}

/// 执行办理人表达式，如 `${handler}`；未命中表达式时原样返回（与 Java 一致）
pub fn eval_variable(expression: &str, variable: &Map<String, Value>) -> Vec<String> {
    let expr = expression.trim();
    if !(expr.starts_with("${") && expr.ends_with('}')) {
        return vec![expression.to_string()];
    }
    let key = &expr[2..expr.len() - 1];
    match variable.get(key) {
        Some(Value::Array(items)) => items.iter().map(value_to_string).filter(|s| !s.is_empty()).collect(),
        Some(Value::String(s)) => {
            if s.is_empty() {
                Vec::new()
            } else {
                // 变量本身可能是 `a,b` 或 `a@@b` 形式
                s.split(&[',', '|'])
                    .flat_map(|part| part.split(flow::SPLIT_AT))
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            }
        }
        Some(other) => {
            let s = value_to_string(other);
            if s.is_empty() {
                Vec::new()
            } else {
                vec![s]
            }
        }
        None => Vec::new(),
    }
}

/// 执行票签表达式（`default@@` 前缀）
pub fn eval_vote_sign(expression: &str, variable: &Map<String, Value>) -> Result<bool> {
    eval_condition(expression, variable)
}

/// 把 JSON 值转成参与比较的字符串
pub fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        other => other.to_string(),
    }
}

/// 比较实际值与期望值：返回 (actual - expect) 的符号（-1 / 0 / 1）。
/// 数字按数值比，其余按字典序比，与 Java `MathUtil.determineSize` 一致。
fn compare(expect: &str, actual: &str) -> i32 {
    math::determine_size(actual, expect)
}
