//! 枚举与状态常量。对应 Java 版 `org.dromara.warm.flow.core.enums`。
//!
//! Java 用 `enum` 承载 "key + 中文描述"，这里改成 newtype + 常量：
//! 数据库与前端都直接使用 key（数字或字符串），无需再做一次映射。

use serde::{Deserialize, Serialize};

/// 节点类型：0 开始 1 中间 2 结束 3 互斥网关 4 并行网关 5 包容网关
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NodeType(pub i32);

impl NodeType {
    pub const START: NodeType = NodeType(0);
    pub const BETWEEN: NodeType = NodeType(1);
    pub const END: NodeType = NodeType(2);
    pub const SERIAL: NodeType = NodeType(3);
    pub const PARALLEL: NodeType = NodeType(4);
    pub const INCLUSIVE: NodeType = NodeType(5);

    pub fn key(self) -> i32 {
        self.0
    }

    pub fn is_start(key: i32) -> bool {
        key == Self::START.0
    }

    pub fn is_between(key: i32) -> bool {
        key == Self::BETWEEN.0
    }

    pub fn is_end(key: i32) -> bool {
        key == Self::END.0
    }

    pub fn is_gateway(key: i32) -> bool {
        matches!(key, 3 | 4 | 5)
    }

    pub fn is_gateway_serial(key: i32) -> bool {
        key == Self::SERIAL.0
    }

    pub fn is_gateway_parallel(key: i32) -> bool {
        key == Self::PARALLEL.0
    }

    pub fn is_gateway_inclusive(key: i32) -> bool {
        key == Self::INCLUSIVE.0
    }
}

impl From<i32> for NodeType {
    fn from(v: i32) -> Self {
        NodeType(v)
    }
}

/// 跳转类型：PASS 通过、REJECT 退回、NONE 无动作
pub struct SkipType;

impl SkipType {
    pub const PASS: &'static str = "PASS";
    pub const REJECT: &'static str = "REJECT";
    pub const NONE: &'static str = "NONE";

    pub fn is_pass(key: &str) -> bool {
        Self::PASS.eq_ignore_ascii_case(key)
    }

    pub fn is_reject(key: &str) -> bool {
        Self::REJECT.eq_ignore_ascii_case(key)
    }

    pub fn is_none(key: &str) -> bool {
        Self::NONE.eq_ignore_ascii_case(key)
    }
}

/// 流程状态
pub struct FlowStatus;

impl FlowStatus {
    pub const TO_BE_SUBMIT: &'static str = "0";
    pub const APPROVAL: &'static str = "1";
    pub const PASS: &'static str = "2";
    pub const AUTO_PASS: &'static str = "3";
    pub const TERMINATE: &'static str = "4";
    pub const NULLIFY: &'static str = "5";
    pub const CANCEL: &'static str = "6";
    pub const RETRIEVE: &'static str = "7";
    pub const FINISHED: &'static str = "8";
    pub const REJECT: &'static str = "9";
    pub const INVALID: &'static str = "10";
    pub const TASK_BACK: &'static str = "11";
    pub const RE_START: &'static str = "12";
    pub const PENDING: &'static str = "13";

    /// 是否终态（已完成 / 终止 / 作废 / 撤销 / 失效）
    pub fn is_finished(key: &str) -> bool {
        matches!(key, "4" | "5" | "6" | "8" | "10")
    }
}

/// 协作方式：1 无（审批）2 转办 3 委派 4 会签 5 票签 6 加签 7 减签
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CooperateType(pub i32);

impl CooperateType {
    pub const APPROVAL: CooperateType = CooperateType(1);
    pub const TRANSFER: CooperateType = CooperateType(2);
    pub const DEPUTE: CooperateType = CooperateType(3);
    pub const COUNTERSIGN: CooperateType = CooperateType(4);
    pub const VOTE: CooperateType = CooperateType(5);
    pub const ADD_SIGNATURE: CooperateType = CooperateType(6);
    pub const REDUCTION_SIGNATURE: CooperateType = CooperateType(7);

    /// 票签固定通过人数策略前缀
    pub const PASS_COUNT: &'static str = "passCount";
    /// 票签固定驳回人数策略前缀
    pub const REJECT_COUNT: &'static str = "rejectCount";
    /// 顺签后缀
    pub const SEQUENCE: &'static str = "sequence";

    pub fn key(self) -> i32 {
        self.0
    }

    /// 或签：比例值为 0
    pub fn is_or_sign(ratio: &str) -> bool {
        crate::utils::math::is_zero(ratio)
    }

    /// 会签：比例值为 100
    pub fn is_countersign(ratio: &str) -> bool {
        crate::utils::math::is_hundred(ratio)
    }

    /// 票签：比例为 0~100 之间
    pub fn is_vote_sign_pass_ratio(ratio: &str) -> bool {
        crate::utils::math::is_between_zero_and_hundred(ratio)
    }

    pub fn is_vote_sign_pass_count(expr: &str) -> bool {
        !expr.is_empty() && expr.starts_with(Self::PASS_COUNT)
    }

    pub fn is_vote_sign_reject_count(expr: &str) -> bool {
        !expr.is_empty() && expr.starts_with(Self::REJECT_COUNT)
    }

    pub fn is_vote_sign_default(expr: &str) -> bool {
        !expr.is_empty() && expr.starts_with(crate::constant::flow::DEFAULT)
    }

    pub fn is_vote_sign_reject_spel(expr: &str) -> bool {
        !expr.is_empty() && expr.starts_with(crate::constant::flow::SPEL)
    }

    /// 顺签：`@@sequence` 结尾
    pub fn is_sequence_sign(expr: &str) -> bool {
        let suffix = format!("{}{}", crate::constant::flow::SPLIT_AT, Self::SEQUENCE);
        expr.ends_with(&suffix)
    }

    pub fn remove_sequence(expr: &str) -> String {
        if Self::is_sequence_sign(expr) {
            let suffix = format!("{}{}", crate::constant::flow::SPLIT_AT, Self::SEQUENCE);
            expr.trim_end_matches(&suffix).to_string()
        } else {
            expr.to_string()
        }
    }
}

impl From<i32> for CooperateType {
    fn from(v: i32) -> Self {
        CooperateType(v)
    }
}

/// 流程图节点办理状态：0 未办理 1 待办理 2 已办理
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChartStatus(pub i32);

impl ChartStatus {
    pub const NOT_DONE: ChartStatus = ChartStatus(0);
    pub const TO_DO: ChartStatus = ChartStatus(1);
    pub const DONE: ChartStatus = ChartStatus(2);

    /// 默认三原色：未办理(灰) / 待办理(黄) / 已办理(绿)
    pub const DEFAULT_RGB: [[u8; 3]; 3] = [[107, 114, 128], [245, 158, 11], [56, 161, 105]];

    pub fn key(self) -> i32 {
        self.0
    }

    pub fn is_not_done(key: i32) -> bool {
        key == Self::NOT_DONE.0
    }

    pub fn is_to_do(key: i32) -> bool {
        key == Self::TO_DO.0
    }

    pub fn is_done(key: i32) -> bool {
        key == Self::DONE.0
    }
}

impl From<i32> for ChartStatus {
    fn from(v: i32) -> Self {
        ChartStatus(v)
    }
}

/// 激活状态：0 挂起 1 激活
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActivityStatus(pub i32);

impl ActivityStatus {
    pub const SUSPENDED: ActivityStatus = ActivityStatus(0);
    pub const ACTIVITY: ActivityStatus = ActivityStatus(1);

    pub fn is_activity(key: i32) -> bool {
        key == Self::ACTIVITY.0
    }

    pub fn is_suspended(key: i32) -> bool {
        key == Self::SUSPENDED.0
    }
}

impl From<i32> for ActivityStatus {
    fn from(v: i32) -> Self {
        ActivityStatus(v)
    }
}

/// 发布状态：0 未发布 1 已发布 9 已失效
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PublishStatus(pub i32);

impl PublishStatus {
    pub const UNPUBLISHED: PublishStatus = PublishStatus(0);
    pub const PUBLISHED: PublishStatus = PublishStatus(1);
    pub const EXPIRED: PublishStatus = PublishStatus(9);
}

impl From<i32> for PublishStatus {
    fn from(v: i32) -> Self {
        PublishStatus(v)
    }
}

/// 流程用户类型：1 审批人 2 转办人 3 委托人
pub struct UserType;

impl UserType {
    pub const APPROVAL: &'static str = "1";
    pub const TRANSFER: &'static str = "2";
    pub const DEPUTE: &'static str = "3";
}

/// 条件表达式类型
pub struct ConditionType;

impl ConditionType {
    pub const EQ: &'static str = "eq";
    pub const GE: &'static str = "ge";
    pub const GT: &'static str = "gt";
    pub const LE: &'static str = "le";
    pub const LIKE: &'static str = "like";
    pub const LT: &'static str = "lt";
    pub const NE: &'static str = "ne";
    pub const NOT_LIKE: &'static str = "notLike";
    pub const ALL: [&'static str; 8] =
        [Self::EQ, Self::GE, Self::GT, Self::LE, Self::LIKE, Self::LT, Self::NE, Self::NOT_LIKE];
}

/// 设计器模型：CLASSICS 经典模式 / MIMIC 仿钉钉模式
pub struct ModelEnum;

impl ModelEnum {
    pub const CLASSICS: &'static str = "CLASSICS";
    pub const MIMIC: &'static str = "MIMIC";
}

/// 表单自定义：Y 内置表单 / N 外挂表单
pub struct FormCustomEnum;

impl FormCustomEnum {
    pub const Y: &'static str = "Y";
    pub const N: &'static str = "N";
}

/// 表单类型：0 内置表单 1 外挂表单
pub struct FormType;

impl FormType {
    pub const INNER: i32 = 0;
    pub const OUTER: i32 = 1;
}
