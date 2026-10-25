/**
 * 流程定义 视图对象
 */
export interface FlowDefinitionVO {
  id: string;
  flowCode: string;
  flowName: string;
  modelValue: string;
  category?: string;
  categoryName?: string;
  version: string;
  /** 0 未发布 1 已发布 9 已失效 */
  isPublish: number;
  /** 1 激活 0 挂起 */
  activityStatus: number;
  formCustom?: string;
  formPath?: string;
  createBy?: string;
  createTime?: string;
  updateBy?: string;
  updateTime?: string;
}

/**
 * 流程定义 查询参数
 */
export interface FlowDefinitionQuery extends PageQuery {
  flowCode?: string;
  flowName?: string;
  category?: string;
  isPublish?: number;
}

/**
 * 设计器：树节点（分类、外挂表单下拉）
 */
export interface DesignTree {
  id: string;
  name?: string;
  label?: string;
  value?: string;
  parentId?: string | null;
  children?: DesignTree[];
}

/**
 * 设计器：连线
 */
export interface DesignSkip {
  nowNodeCode: string;
  nextNodeCode: string;
  skipName?: string | null;
  /** PASS 通过 / REJECT 退回 / NONE 无条件 */
  skipType?: string | null;
  skipCondition?: string | null;
  /** "x1,y1;x2,y2" 起点与终点 */
  coordinate?: string | null;
  /** 流程图着色用（0 未办理 1 待办理 2 已办理），设计器不填 */
  status?: number | null;
}

/**
 * 设计器：节点
 *
 * nodeType：0 开始 1 审批 2 结束 3 互斥网关 4 并行网关 5 包容网关（与 warm-flow NodeType 一致）
 */
export interface DesignNode {
  id?: string | number | null;
  nodeType: number;
  nodeCode: string;
  nodeName: string;
  version?: string | null;
  /** 办理人标识，多个用 @@ 分隔，如 role:1@@dept:103@@user:1 */
  permissionFlag?: string | null;
  nodeRatio?: string | null;
  /** "x,y|x,y" 节点坐标 */
  coordinate?: string | null;
  /** 自由跳转目标节点 */
  anyNodeSkip?: string | null;
  listenerType?: string | null;
  listenerPath?: string | null;
  formCustom?: string | null;
  formPath?: string | null;
  /** JSON 串，按钮权限等扩展配置 */
  ext?: string | null;
  /** 办理状态：0 未办理 1 待办理 2 已办理 */
  status?: number | null;
  skipList: DesignSkip[];
}

/**
 * 设计器整份存盘 JSON
 */
export interface DesignJson {
  /** 雪花 id，后端以字符串下发避免 JS 精度丢失 */
  id?: string | number | null;
  flowCode?: string | null;
  flowName?: string | null;
  modelValue?: string | null;
  category?: string | null;
  version?: string | null;
  isPublish?: number | null;
  formCustom?: string | null;
  formPath?: string | null;
  listenerType?: string | null;
  listenerPath?: string | null;
  ext?: string | null;
  nodeList: DesignNode[];
  /** 三个状态对应的 rgb 颜色 */
  chartStatusColor?: string[] | null;
  chartExt?: string | null;
  topText?: string | null;
  topTextShow?: boolean | null;
  categoryList?: DesignTree[] | null;
  formPathList?: DesignTree[] | null;
}
