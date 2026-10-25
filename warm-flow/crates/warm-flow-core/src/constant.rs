//! 常量：对应 Java 版 `FlowCons` / `ExceptionCons`。

/// 流程常量
pub mod flow {
    /// 多值分隔符，节点权限标识、监听器路径都用它分隔
    pub const SPLIT_AT: &str = "@@";
    /// 条件表达式内部变量与值分隔符，如 `flag|4`
    pub const SPLIT_VERTICAL: &str = "|";
    /// 多目标节点 / 多人拼接分隔符，如历史任务的 target_node_code
    pub const COMMA: &str = ",";
    /// 票签默认表达式前缀
    pub const DEFAULT: &str = "default";
    /// spel 表达式前缀
    pub const SPEL: &str = "spel";
    /// snel 表达式前缀
    pub const SNEL: &str = "snel";

    /// 发起人标识，办理过程中会被替换为实际发起人
    pub const WARMFLOWINITIATOR: &str = "warmFlowInitiator";
    /// 监听器参数在流程变量中的 key
    pub const WARM_LISTENER_PARAM: &str = "WarmListenerParam";

    /// 审批表单自定义
    pub const FORM_CUSTOM_Y: &str = "Y";
    /// 审批表单非自定义
    pub const FORM_CUSTOM_N: &str = "N";
    /// 表单数据在流程变量中的 key
    pub const FORM_DATA: &str = "formData";

    /// 前置节点遍历方向
    pub const PREVIOUS: &str = "previous";
    /// 后置节点遍历方向
    pub const SUFFIX: &str = "suffix";
}

/// 监听器类型，对应 Java `Listener` 接口常量
pub mod listener {
    pub const LISTENER_START: &str = "start";
    pub const LISTENER_ASSIGNMENT: &str = "assignment";
    pub const LISTENER_FINISH: &str = "finish";
    pub const LISTENER_CREATE: &str = "create";
    pub const LISTENER_FORM_LOAD: &str = "formLoad";
}

/// 错误提示语，文案与 Java 版保持一致，便于前端提示复用
pub mod exception {
    pub const SAME_CONDITION_VALUE: &str = "中间节点，同一个节点不能有相同跳转类型，跳转同一个目标节点!";
    pub const SAME_CONDITION_NODE: &str = "互斥网关，同一个节点不能有相同跳转条件，跳转同一个目标节点!";
    pub const SAME_DEST_NODE: &str = "并行网关，同一个节点不能跳转同一个目标节点!";
    pub const MUL_START_NODE: &str = "开始节点不能超过1个!";
    pub const MUL_SKIP_BETWEEN: &str = "不可同时通过或者退回到多个中间节点，必须先流转到网关节点!";
    pub const MUL_START_SKIP: &str = "节点流转条件不能超过1个!";
    pub const LOST_START_NODE: &str = "流程缺少开始节点!";
    pub const LOST_NODE_CODE: &str = "节点编码缺失";
    pub const MUST_SKIP: &str = "开始或者中间节点必须画跳转线";
    pub const SAME_NODE_CODE: &str = "同一流程中节点编码重复!";
    pub const NULL_DEST_NODE: &str = "无法跳转，未配置目标节点!";
    pub const NULL_SKIP_TYPE: &str = "未找到跳转类型匹配的目标节点!";
    pub const NULL_CONDITION_VALUE_NODE: &str = "未找到跳转条件，不支持跳转!";
    pub const NULL_CONDITION_VALUE: &str = "跳转条件不能为空!";
    pub const FIRST_FORBID_BACK: &str = "禁止退回到第一个节点";
    pub const NULL_ROLE_NODE: &str = "无法跳转到该节点,请检查当前用户是否有权限!";
    pub const LOST_DEST_NODE: &str = "目标节点为空!";
    pub const LOST_CUR_NODE: &str = "当前流程节点丢失!";
    pub const NULL_NODE_CODE: &str = "目标节点编码不存在!";
    pub const NULL_BUSINESS_ID: &str = "业务id为空!";
    pub const NULL_FLOW_CODE: &str = "流程编码缺失!";
    pub const NOT_FOUND_DEF: &str = "流程定义不存在!";
    pub const NOT_FOUND_INSTANCE: &str = "流程实例获取失败!";
    pub const NULL_INSTANCE_ID: &str = "流程实例id不能为空!";
    pub const NULL_TASK_ID: &str = "任务id不能为空!";
    pub const NOT_FOUND_TASK: &str = "未找到待办任务!";
    pub const TASK_NOT_ONE: &str = "此接口不能同时跳转多个待办任务，请更换!";
    pub const NOT_DEFINITION_ID: &str = "流程定义id不能为空!";
    pub const NOT_NODE_DATA: &str = "流程节点数据缺失!";
    pub const EXIST_START_TASK: &str = "流程定义已开启过审批任务，不可操作!";
    pub const FLOW_FINISH: &str = "流程已完成！";
    pub const NOT_AUTHORITY: &str = "请检查当前用户是否有权限!";
    pub const SIGN_NULL_HANDLER: &str = "会签票签时，办理人标识不能为空";
    pub const REDUCTION_SIGN_ONE_ERROR: &str = "办理人不足或者只有一人，不可减签";
    pub const IS_ALREADY_SIGN: &str = "已经是待办人，不可加签";
    pub const IS_ALREADY_TRANSFER: &str = "已经是转办人，不可转办";
    pub const IS_ALREADY_DEPUTE: &str = "已经是受托人，不可委托";
    pub const NOT_ACTIVITY: &str = "当前流程定义或者实例已经挂起，请先激活";
    pub const NOT_DEFINITION_ACTIVITY: &str = "当前流程定义已挂起，不可开启新的流程";
    pub const DEFINITION_ALREADY_ACTIVITY: &str = "当前流程定义已经激活";
    pub const DEFINITION_ALREADY_SUSPENDED: &str = "当前流程定义已经挂起";
    pub const INSTANCE_ALREADY_ACTIVITY: &str = "当前流程实例已经激活";
    pub const NOT_DEF_PROMOTER_NOT_CANCEL: &str = "不是当前流程的发起人，无法撤销";
    pub const HANDLER_NOT_EMPTY: &str = "办理人不能为空";
    pub const TAR_NOT_GATEWAY: &str = "目标节点不能是网关节点!";
    pub const NOT_FOUND_FLOW_TASK: &str = "未获取到流程任务";
    pub const FLOW_HAVE_USELESS_SKIP: &str = "存在无用的跳转";
    pub const NULL_TRANSFER_HANDLER: &str = "转办对象不能为空";
    pub const NULL_DEPUTE_HANDLER: &str = "委托对象不能为空";
    pub const NULL_ADD_SIGNATURE_HANDLER: &str = "加签对象不能为空";
    pub const NULL_REDUCTION_SIGNATURE_HANDLER: &str = "减签对象不能为空";
    pub const NOT_FOUND_LAST_TASK: &str = "未找到前置任务!";
    pub const NOT_FOUND_HANDLED_TASK: &str = "未找到您已办理过的任务!";
    pub const NOT_FOUND_HANDLED_TASK_HANDLER: &str = "拿回的任务未办理!";
    pub const START_NODE_NOT_ALLOW_JUMP: &str = "开始节点不允许跳转!";
    pub const NOT_DRAW_FLOW_ERROR: &str = "未绘制流程图，不可发布";
    pub const ID_EMPTY: &str = "ID不能为空";
}
