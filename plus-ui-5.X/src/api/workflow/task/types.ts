/**
 * 待办任务 视图对象
 */
export interface FlowTaskVO {
  id: string;
  instanceId: string;
  definitionId: string;
  flowName?: string;
  flowCode?: string;
  businessId?: string;
  nodeCode: string;
  nodeName?: string;
  /** 0 开始 1 中间 2 结束 3 互斥 4 并行 5 包容 */
  nodeType: number;
  flowStatus: string;
  formCustom?: string;
  formPath?: string;
  createBy?: string;
  createTime?: string;
}

/**
 * 已办任务（历史记录）视图对象
 */
export interface FlowHisTaskVO {
  id: string;
  definitionId: string;
  instanceId: string;
  flowName?: string;
  businessId?: string;
  nodeCode?: string;
  nodeName?: string;
  targetNodeCode?: string;
  targetNodeName?: string;
  approver?: string;
  /** 办理人昵称（后端按 approver 的用户 id 反查 sys_user 补齐） */
  approverName?: string;
  /** 1 审批 2 转办 3 委派 4 会签 5 票签 6 加签 7 减签 */
  cooperateType: number;
  /** PASS / REJECT / NONE */
  skipType: string;
  flowStatus: string;
  message?: string;
  createTime?: string;
  updateTime?: string;
}

/**
 * 待办 / 已办 查询参数
 */
export interface FlowTaskQuery extends PageQuery {
  nodeCode?: string;
  flowStatus?: string;
  businessId?: string;
  flowCode?: string;
}

/**
 * 发起流程入参
 */
export interface StartProcessBo {
  flowCode?: string;
  businessId?: string;
  variables?: Record<string, any>;
}

/**
 * 发起流程返回
 */
export interface StartReturnVO {
  processInstanceId: string;
  taskId?: string;
  businessCode?: string;
}

/**
 * 办理任务入参
 */
export interface CompleteTaskBo {
  taskId: string | number;
  /** PASS 通过 / REJECT 退回 */
  skipType?: string;
  message?: string;
  /** 退回 / 跳转的目标节点编码 */
  nodeCode?: string;
  variables?: Record<string, any>;
  /** 指定下一节点办理人（用户 id 字符串） */
  nextHandlers?: string[];
}

/**
 * 终止流程入参
 */
export interface TerminationBo {
  taskId: string | number;
  message?: string;
}

/**
 * 撤销流程入参
 */
export interface RevokeBo {
  instanceId: string | number;
  message?: string;
}

/**
 * 任务操作入参（转办 / 委派 / 加签 / 减签）
 */
export interface TaskOperationBo {
  taskId: string | number;
  handlerIds: string[];
  message?: string;
}
