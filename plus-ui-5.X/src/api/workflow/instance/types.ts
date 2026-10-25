/**
 * 流程实例 视图对象
 */
export interface FlowInstanceVO {
  id: string;
  definitionId: string;
  flowName?: string;
  flowCode?: string;
  /** 业务主键 */
  businessId: string;
  /** 0 开始 1 中间 2 结束 3 互斥 4 并行 5 包容 */
  nodeType: number;
  nodeCode: string;
  nodeName?: string;
  /** 流程状态字典 flow_status */
  flowStatus: string;
  /** 1 激活 0 挂起 */
  activityStatus: number;
  createBy?: string;
  /** 发起人昵称（后端按 createBy 的用户 id 反查 sys_user 补齐） */
  createByName?: string;
  createTime?: string;
  updateTime?: string;
}

/**
 * 流程实例 查询参数
 */
export interface FlowInstanceQuery extends PageQuery {
  flowCode?: string;
  businessId?: string;
  flowStatus?: string;
  createBy?: string;
}

/**
 * 流程变量
 */
export interface FlowVariableVO {
  [key: string]: any;
}
