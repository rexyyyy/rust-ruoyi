//! 工作流入参（DTO）。
//!
//! 字段名统一 camelCase，与 RuoYi 前端保持一致；分页字段沿用 `pageNum` / `pageSize`。

use serde::{Deserialize, Serialize};

/// 兼容前端回传的雪花 ID：字符串 / 整数 / 浮点 / null 四种 JSON 形态均可。
///
/// VO 主键输出为 String（防 JS 精度丢失），前端会原样回传，这里必须双向兼容。
pub fn string_or_number<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum V {
        S(String),
        I(i64),
        F(f64),
    }
    match Option::<V>::deserialize(deserializer)? {
        None => Ok(None),
        Some(V::S(s)) => {
            let s = s.trim();
            if s.is_empty() {
                return Ok(None);
            }
            s.parse().map(Some).map_err(serde::de::Error::custom)
        }
        Some(V::I(i)) => Ok(Some(i)),
        Some(V::F(f)) => Ok(Some(f as i64)),
    }
}

// ======================================================================
// 流程分类
// ======================================================================

/// 分类查询（支持按名称模糊）
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CategoryQuery {
    pub category_name: Option<String>,
}

/// 新增 / 修改分类
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CategoryBo {
    #[serde(default, deserialize_with = "string_or_number")]
    pub category_id: Option<i64>,
    #[serde(default, deserialize_with = "string_or_number")]
    pub parent_id: Option<i64>,
    pub category_name: Option<String>,
    pub order_num: Option<i32>,
}

// ======================================================================
// 流程定义
// ======================================================================

/// 流程定义分页
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionPageBo {
    #[serde(rename = "pageNum")]
    pub page_no: Option<u64>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<u64>,
    pub flow_code: Option<String>,
    pub flow_name: Option<String>,
    pub category: Option<String>,
    pub is_publish: Option<i32>,
}

/// 批量操作（删除 / 发布 / 取消发布）入参
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionIdsBo {
    #[serde(default, deserialize_with = "de_ids")]
    pub ids: Vec<i64>,
    /// 兼容单个 id 传参
    #[serde(default, deserialize_with = "string_or_number")]
    pub id: Option<i64>,
}

impl DefinitionIdsBo {
    /// 合并 `ids` 与单个 `id`，去重后返回
    pub fn all_ids(&self) -> Vec<i64> {
        let mut out = self.ids.clone();
        if let Some(id) = self.id {
            if !out.contains(&id) {
                out.push(id);
            }
        }
        out
    }
}

/// 兼容 `ids: ["123"]` / `ids: [123]` / `ids: "123,456"`
fn de_ids<'de, D>(deserializer: D) -> Result<Vec<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum V {
        One(String),
        Many(Vec<serde_json::Value>),
    }
    let v = Option::<V>::deserialize(deserializer)?;
    let mut out = Vec::new();
    match v {
        None => {}
        Some(V::One(s)) => {
            for part in s.split(',') {
                if let Ok(id) = part.trim().parse::<i64>() {
                    out.push(id);
                }
            }
        }
        Some(V::Many(list)) => {
            for item in list {
                match item {
                    serde_json::Value::String(s) => {
                        if let Ok(id) = s.trim().parse::<i64>() {
                            out.push(id);
                        }
                    }
                    serde_json::Value::Number(n) => {
                        if let Some(id) = n.as_i64() {
                            out.push(id);
                        } else if let Some(f) = n.as_f64() {
                            out.push(f as i64);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(out)
}

/// 流程定义导出 / 导入入参
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionImportBo {
    /// 设计器导出的 JSON 字符串
    pub json: Option<String>,
}

// ======================================================================
// 流程实例
// ======================================================================

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstancePageBo {
    #[serde(rename = "pageNum")]
    pub page_no: Option<u64>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<u64>,
    /// 流程编码（前端传，服务层翻译成 definitionId 集合过滤）
    pub flow_code: Option<String>,
    pub business_id: Option<String>,
    pub flow_status: Option<String>,
    pub create_by: Option<String>,
}

// ======================================================================
// 任务（待办 / 已办）
// ======================================================================

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TaskPageBo {
    #[serde(rename = "pageNum")]
    pub page_no: Option<u64>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<u64>,
    pub node_code: Option<String>,
    pub flow_status: Option<String>,
    pub business_id: Option<String>,
    pub flow_code: Option<String>,
}

/// 启动流程
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartRrocessBo {
    pub flow_code: Option<String>,
    /// 业务主键（请假单 id）
    pub business_id: Option<String>,
    #[serde(default)]
    pub variables: serde_json::Map<String, serde_json::Value>,
}

/// 办理任务
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompleteTaskBo {
    #[serde(deserialize_with = "string_or_number")]
    pub task_id: Option<i64>,
    /// PASS / REJECT
    pub skip_type: Option<String>,
    /// 办理意见
    pub message: Option<String>,
    /// 指定退回 / 跳转的目标节点
    pub node_code: Option<String>,
    #[serde(default)]
    pub variables: serde_json::Map<String, serde_json::Value>,
    /// 指定下一节点办理人
    #[serde(default)]
    pub next_handlers: Vec<String>,
}

/// 任务操作（转办 / 委派 / 加签 / 减签）
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TaskOperationBo {
    #[serde(deserialize_with = "string_or_number")]
    pub task_id: Option<i64>,
    /// 目标办理人
    #[serde(default)]
    pub handler_ids: Vec<String>,
    pub message: Option<String>,
}

/// 终止流程
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TerminationBo {
    #[serde(deserialize_with = "string_or_number")]
    pub task_id: Option<i64>,
    pub message: Option<String>,
}

/// 撤销流程
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct RevokeBo {
    #[serde(deserialize_with = "string_or_number")]
    pub instance_id: Option<i64>,
    pub message: Option<String>,
}

// ======================================================================
// 请假单
// ======================================================================

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeavePageBo {
    #[serde(rename = "pageNum")]
    pub page_no: Option<u64>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<u64>,
    pub leave_type: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaveBo {
    #[serde(default, deserialize_with = "string_or_number")]
    pub id: Option<i64>,
    pub leave_type: Option<String>,
    /// `YYYY-MM-DD HH:mm:ss` 或 RFC3339
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub leave_days: Option<i32>,
    pub remark: Option<String>,
    /// 要走的流程编码，不传则用默认请假流程（leave1）
    pub flow_code: Option<String>,
}

impl LeaveBo {
    /// 解析时间：兼容 `YYYY-MM-DD`、`YYYY-MM-DD HH:MM:SS`、RFC3339
    pub fn start_dt(&self) -> Result<Option<chrono::NaiveDateTime>, String> {
        parse_dt(self.start_date.as_deref())
    }

    pub fn end_dt(&self) -> Result<Option<chrono::NaiveDateTime>, String> {
        parse_dt(self.end_date.as_deref())
    }
}

pub fn parse_dt(raw: Option<&str>) -> Result<Option<chrono::NaiveDateTime>, String> {
    use chrono::NaiveDateTime;
    let Some(text) = raw.map(str::trim).filter(|s| !s.is_empty()) else {
        return Ok(None);
    };
    const FORMATS: [&str; 5] = [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%dT%H:%M",
        "%Y-%m-%d",
    ];
    for fmt in FORMATS {
        if let Ok(dt) = NaiveDateTime::parse_from_str(text, fmt) {
            return Ok(Some(dt));
        }
    }
    if let Ok(d) = chrono::NaiveDate::parse_from_str(text, "%Y-%m-%d") {
        return Ok(Some(d.and_hms_opt(0, 0, 0).unwrap_or_default()));
    }
    Err(format!("时间格式不正确: {text}"))
}
