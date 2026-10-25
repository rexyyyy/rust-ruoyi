//! 设计器的业务扩展点。
//!
//! Java 版通过 `FrameInvoker.getBean(XxxService.class)` 反射取业务实现，
//! Rust 版统一收敛为一个 trait：业务方实现它并注入 [`crate::FlowWebState`]，
//! 未实现的部分走默认空实现，保证设计器可用。

use async_trait::async_trait;

use warm_flow_core::dto::{DefJson, Tree};

use crate::vo::{Dict, HandlerFeedBackVo, HandlerQuery, HandlerSelectVo, ListenerVo, NodeExt};

/// 设计器扩展点，对应 Java 的 `CategoryService` / `FormPathService` /
/// `HandlerSelectService` / `HandlerDictService` / `NodeExtService` /
/// `ListenerListService` / `ChartExtService`
#[async_trait]
pub trait UiExtension: Send + Sync {
    /// 框架名称，透给设计器
    fn framework(&self) -> String {
        "actix-web".to_string()
    }

    /// 办理人权限设置 tabs 页签
    fn handler_types(&self) -> Vec<String> {
        Vec::new()
    }

    /// 办理人权限设置列表
    async fn handler_select(&self, _query: &HandlerQuery) -> HandlerSelectVo {
        HandlerSelectVo::default()
    }

    /// 办理人权限名称回显
    async fn handler_feedback(&self, storage_ids: &[String]) -> Vec<HandlerFeedBackVo> {
        storage_ids
            .iter()
            .map(|id| HandlerFeedBackVo {
                storage_id: Some(id.clone()),
                handler_name: None,
            })
            .collect()
    }

    /// 办理人表达式下拉
    fn handler_dict(&self) -> Vec<Dict> {
        vec![
            Dict::new("默认表达式", "${handler}"),
            Dict::new("spel表达式", "#{@user.evalVar(#handler)}"),
            Dict::new("其他", ""),
        ]
    }

    /// 节点扩展属性
    fn node_ext(&self) -> Vec<NodeExt> {
        Vec::new()
    }

    /// 监听器列表
    fn listener_list(&self) -> Vec<ListenerVo> {
        Vec::new()
    }

    /// 流程分类树（扁平列表，含 parentId，由接口层拼装）
    fn categories(&self) -> Vec<Tree> {
        Vec::new()
    }

    /// 表单路径树（扁平列表，含 parentId，由接口层拼装）
    fn form_paths(&self) -> Vec<Tree> {
        Vec::new()
    }

    /// 流程图渲染前的补充：写入节点提示语、业务高亮等
    fn chart_ext(&self, _def_json: &mut DefJson) {}
}

/// 空实现：设计器可用，但办理人选择、节点扩展等需要业务系统自己实现
pub struct DefaultUiExtension;

impl UiExtension for DefaultUiExtension {}
