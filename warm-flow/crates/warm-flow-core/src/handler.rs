//! 扩展点与内置处理器：ID 生成、权限、租户、监听器、全局配置。
//!
//! Java 版通过类路径 + Spring 容器注入（`DataFillHandler` / `PermissionHandler` /
//! `TenantHandler` / `GlobalListener`），Rust 版改为 trait 对象，由调用方在启动时注册。

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

/// 内置 19 位雪花 ID，对应 Java `SnowFlakeId19`
///
/// 41 位毫秒时间戳 + 5 位数据中心 + 5 位机器号 + 12 位序列，
/// 起始纪元取 2020-01-01，可用到 2089 年。
#[derive(Debug)]
pub struct SnowflakeId19 {
    epoch: i64,
    worker_id: u64,
    datacenter_id: u64,
    sequence: AtomicU64,
    last_timestamp: AtomicU64,
}

impl Clone for SnowflakeId19 {
    fn clone(&self) -> Self {
        Self {
            epoch: self.epoch,
            worker_id: self.worker_id,
            datacenter_id: self.datacenter_id,
            sequence: AtomicU64::new(self.sequence.load(Ordering::Relaxed)),
            last_timestamp: AtomicU64::new(self.last_timestamp.load(Ordering::Relaxed)),
        }
    }
}

impl Default for SnowflakeId19 {
    fn default() -> Self {
        Self::new(1, 1)
    }
}

impl SnowflakeId19 {
    pub fn new(worker_id: u64, datacenter_id: u64) -> Self {
        Self {
            epoch: 1_577_836_800_000, // 2020-01-01T00:00:00Z
            worker_id: worker_id & 0x1F,
            datacenter_id: datacenter_id & 0x1F,
            sequence: AtomicU64::new(0),
            last_timestamp: AtomicU64::new(0),
        }
    }

    pub fn next_id(&self) -> i64 {
        loop {
            let now = chrono::Utc::now().timestamp_millis() - self.epoch;
            let now = now.max(0) as u64;
            let last = self.last_timestamp.load(Ordering::Relaxed);
            if now == last {
                let seq = self.sequence.fetch_add(1, Ordering::Relaxed) + 1;
                if seq < 4096 {
                    return self.compose(now, seq);
                }
                // 同一毫秒序列用尽，自旋到下一毫秒
                while (chrono::Utc::now().timestamp_millis() - self.epoch).max(0) as u64 == last {
                    std::hint::spin_loop();
                }
            } else if now > last {
                if self.last_timestamp.compare_exchange(last, now, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                    self.sequence.store(0, Ordering::Relaxed);
                    return self.compose(now, 0);
                }
            }
            // now < last（时钟回拨）时短暂等待，避免生成重复 id
            std::thread::yield_now();
        }
    }

    fn compose(&self, timestamp: u64, sequence: u64) -> i64 {
        ((timestamp << 22) | (self.datacenter_id << 17) | (self.worker_id << 12) | sequence) as i64
    }
}

/// ID 生成器抽象
pub trait IdGenerator: Send + Sync {
    fn next_id(&self) -> i64;
}

impl IdGenerator for SnowflakeId19 {
    fn next_id(&self) -> i64 {
        SnowflakeId19::next_id(self)
    }
}

/// 当前时间（不带时区，与 MySQL datetime 对应）
pub fn now() -> NaiveDateTime {
    chrono::Local::now().naive_local()
}

/// 数据填充处理器：对应 Java `DataFillHandler`。
///
/// 引擎新增记录时统一走这里，业务方可以借此把 create_by / update_by / tenant_id
/// 替换成业务系统自己的上下文。
pub trait DataFillHandler: Send + Sync {
    /// 生成主键
    fn id(&self) -> i64;
    /// 当前操作人（写入 create_by / update_by）
    fn operator(&self) -> Option<String> {
        None
    }
    /// 当前租户（写入 tenant_id）
    fn tenant_id(&self) -> Option<String> {
        None
    }
    /// 当前时间
    fn now(&self) -> NaiveDateTime {
        now()
    }
}

/// 默认填充器：19 位雪花 + 本地时间
pub struct DefaultDataFillHandler {
    id_gen: SnowflakeId19,
    operator: Option<String>,
}

impl Default for DefaultDataFillHandler {
    fn default() -> Self {
        Self { id_gen: SnowflakeId19::default(), operator: None }
    }
}

impl DefaultDataFillHandler {
    pub fn new(operator: Option<String>) -> Self {
        Self { id_gen: SnowflakeId19::default(), operator }
    }
}

impl DataFillHandler for DefaultDataFillHandler {
    fn id(&self) -> i64 {
        self.id_gen.next_id()
    }

    fn operator(&self) -> Option<String> {
        self.operator.clone()
    }
}

/// 办理人权限处理器，对应 Java `PermissionHandler`
pub trait PermissionHandler: Send + Sync {
    /// 当前登录用户 id
    fn get_handler(&self) -> Option<String> {
        None
    }

    /// 当前登录用户的权限标识集合（角色 / 部门等）
    fn permissions(&self) -> Vec<String> {
        Vec::new()
    }

    /// 把设计器里配置的 "角色/部门" 转换成实际用户 id
    fn convert_permissions(&self, permissions: Vec<String>) -> Vec<String> {
        permissions
    }
}

/// 空权限处理器
pub struct NoopPermissionHandler;

impl PermissionHandler for NoopPermissionHandler {}

/// 租户处理器，对应 Java `TenantHandler`
pub trait TenantHandler: Send + Sync {
    fn tenant_id(&self) -> Option<String> {
        None
    }
}

/// 空租户处理器
pub struct NoopTenantHandler;

impl TenantHandler for NoopTenantHandler {}

/// 监听器上下文，对应 Java `ListenerVariable`
#[derive(Debug, Clone, Default)]
pub struct ListenerContext {
    pub definition: Option<crate::entity::Definition>,
    pub instance: Option<crate::entity::Instance>,
    pub node: Option<crate::entity::Node>,
    pub task: Option<crate::entity::Task>,
    pub next_nodes: Vec<crate::entity::Node>,
    pub next_tasks: Vec<crate::entity::Task>,
    pub variable: serde_json::Map<String, serde_json::Value>,
}

/// 全局监听器，对应 Java `GlobalListener`
pub trait GlobalListener: Send + Sync {
    fn notify(&self, listener_type: &str, ctx: &ListenerContext);
}

/// 空全局监听器
pub struct NoopGlobalListener;

impl GlobalListener for NoopGlobalListener {
    fn notify(&self, _listener_type: &str, _ctx: &ListenerContext) {}
}

/// 引擎配置，对应 Java `WarmFlow`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarmFlowConfig {
    /// 是否启用引擎
    pub enabled: bool,
    /// 是否打印 banner
    pub banner: bool,
    /// id 生成器类型：SnowId14 / SnowId15 / SnowId19
    pub key_type: Option<String>,
    /// 是否开启逻辑删除
    pub logic_delete: bool,
    /// 已删除标志值
    pub logic_delete_value: String,
    /// 未删除标志值
    pub logic_not_delete_value: String,
    /// 数据源类型：mysql / postgresql / sqlite
    pub data_source_type: Option<String>,
    /// 是否启用设计器接口
    pub ui: bool,
    /// 鉴权 token 头名称，多个用逗号分隔
    pub token_name: String,
    /// 流程图已办理颜色（r,g,b）
    pub chart_status_color: Option<Vec<String>>,
    /// 经典模式三原色
    pub chart_status_color_classics: Option<Vec<String>>,
    /// 仿钉钉模式三原色
    pub chart_status_color_mimic: Option<Vec<String>>,
    /// 流程图是否显示顶部文字
    pub top_text_show: bool,
}

impl Default for WarmFlowConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            banner: true,
            key_type: Some("SnowId19".to_string()),
            logic_delete: false,
            logic_delete_value: "2".to_string(),
            logic_not_delete_value: "0".to_string(),
            data_source_type: None,
            ui: true,
            token_name: "Authorization".to_string(),
            chart_status_color: None,
            chart_status_color_classics: None,
            chart_status_color_mimic: None,
            top_text_show: true,
        }
    }
}

impl WarmFlowConfig {
    /// 解析三原色，缺省用内置色值
    pub fn rgb_of(&self, model_value: &str) -> [[u8; 3]; 3] {
        let custom = match model_value {
            crate::enums::ModelEnum::MIMIC => self.chart_status_color_mimic.as_ref(),
            _ => self.chart_status_color_classics.as_ref(),
        }
        .or(self.chart_status_color.as_ref());

        let mut rgb = crate::enums::ChartStatus::DEFAULT_RGB;
        if let Some(list) = custom {
            for (i, item) in list.iter().take(3).enumerate() {
                let parts: Vec<&str> = item.split(',').collect();
                if parts.len() == 3 {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        parts[0].trim().parse::<u8>(),
                        parts[1].trim().parse::<u8>(),
                        parts[2].trim().parse::<u8>(),
                    ) {
                        rgb[i] = [r, g, b];
                    }
                }
            }
        }
        rgb
    }
}
