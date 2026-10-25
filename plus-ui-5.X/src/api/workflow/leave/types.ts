/**
 * 请假单 视图对象
 */
export interface LeaveVO {
  id: string;
  /** 申请编号（发起流程后回填） */
  applyCode?: string;
  leaveType: string;
  startDate?: string;
  endDate?: string;
  leaveDays: number;
  remark?: string;
  /** draft / waiting / finish / cancel / back / termination / invalid */
  status?: string;
  /** 实际走的流程编码（未发起流程时为空） */
  flowCode?: string;
  createBy?: string;
  createTime?: string;
}

/**
 * 请假单 表单（新增 / 修改 / 提交共用）
 */
export interface LeaveForm {
  id?: string | number;
  leaveType?: string;
  startDate?: string;
  endDate?: string;
  leaveDays?: number;
  remark?: string;
  /** 要走的流程编码，留空则用后端默认请假流程 leave1 */
  flowCode?: string;
}

/**
 * 请假单 查询参数
 */
export interface LeaveQuery extends PageQuery {
  leaveType?: string;
  status?: string;
}
