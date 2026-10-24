/**
 * 演示课程表 视图对象 DemoCourseVO
 * 源表：demo_course　由 codegen 于 2026-10-08 21:03:18 生成
 */
export interface DemoCourseVO {
  /**
   * 课程编号
   */
  courseId: string | number;
  /**
   * 课程编码
   */
  courseCode?: string;
  /**
   * 课程名称
   */
  courseName?: string;
  /**
   * 授课教师
   */
  teacher?: string;
  /**
   * 学分
   */
  credit?: number;
  /**
   * 开课状态
   */
  openStatus?: string;
  /**
   * 开课日期
   */
  startDate?: string;
  /**
   * 备注
   */
  remark?: string;
  /**
   * 创建者
   */
  createBy?: number;
  /**
   * 创建时间
   */
  createTime?: string;
  /**
   * 更新者
   */
  updateBy?: number;
  /**
   * 更新时间
   */
  updateTime?: string;
  }

/**
 * 演示课程表 查询参数
 */
export interface DemoCourseQuery extends PageQuery {
  courseCode?: string;
  courseName?: string;
  }

/**
 * 演示课程表 表单对象（新增 / 修改共用）
 */
export interface DemoCourseForm {
  courseId?: string | number;
  courseCode?: string;
  courseName?: string;
  teacher?: string;
  credit?: number;
  openStatus?: string;
  startDate?: string;
  remark?: string;
  
  
  
  
  }
