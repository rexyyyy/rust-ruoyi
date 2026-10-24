/**
 * 注册码登记 视图对象 OmsRegorderVO
 * 源表：oms_regorder　由 codegen 于 2026-10-10 16:24:42 生成
 */
export interface OmsRegorderVO {
  /**
   * 主键
   */
  regid: string | number;
  /**
   * 订单号
   */
  orderSn?: string;
  /**
   * 用户名
   */
  memberUsername?: string;
  /**
   * 订单总金额
   */
  totalAmount?: number;
  /**
   * 应付金额
   */
  payAmount?: number;
  /**
   * 支付方式：0->未支付；1->支付宝；2->微信
   */
  payType?: string;
  /**
   * 订单来源：0->PC订单；1->app订单
   */
  sourceType?: string;
  /**
   * 订单状态：0->待付款；1->待发货；2->已发货；3->已完成；4->已关闭；5->无效订单
   */
  status?: string;
  /**
   * 订单类型：0->正常订单；1->秒杀订单
   */
  orderType?: string;
  /**
   * 物流公司(配送方式)
   */
  deliveryCompany?: string;
  /**
   * 申请号
   */
  requestSn?: string;
  /**
   * 回复号
   */
  deliverySn?: string;
  /**
   * 收货人姓名
   */
  receiverName?: string;
  /**
   * 收货人电话
   */
  receiverPhone?: string;
  /**
   * 省份/直辖市
   */
  receiverProvince?: string;
  /**
   * 城市
   */
  receiverCity?: string;
  /**
   * 区
   */
  receiverRegion?: string;
  /**
   * 订单备注
   */
  note?: string;
  /**
   * 确认收货状态：0->未确认；1->已确认
   */
  confirmStatus?: string;
  /**
   * 支付时间
   */
  paymentTime?: string;
  /**
   * 发货时间
   */
  deliveryTime?: string;
  /**
   * 确认收货时间
   */
  receiveTime?: string;
  /**
   * 评价时间
   */
  commentTime?: string;
  /**
   * 删除标志（0代表存在 2代表删除）
   */
  delFlag?: string;
  /**
   * 创建者
   */
  createBy?: string;
  /**
   * 创建时间
   */
  createTime?: string;
  /**
   * 更新者
   */
  updateBy?: string;
  /**
   * 更新时间
   */
  updateTime?: string;
  }

/**
 * 注册码登记 查询参数
 */
export interface OmsRegorderQuery extends PageQuery {
  memberUsername?: string;
  payType?: string;
  sourceType?: string;
  status?: string;
  orderType?: string;
  receiverName?: string;
  }

/**
 * 注册码登记 表单对象（新增 / 修改共用）
 */
export interface OmsRegorderForm {
  regid?: string | number;
  orderSn?: string;
  memberUsername?: string;
  totalAmount?: number;
  payAmount?: number;
  payType?: string;
  sourceType?: string;
  status?: string;
  orderType?: string;
  deliveryCompany?: string;
  requestSn?: string;
  deliverySn?: string;
  receiverName?: string;
  receiverPhone?: string;
  receiverProvince?: string;
  receiverCity?: string;
  receiverRegion?: string;
  note?: string;
  confirmStatus?: string;
  paymentTime?: string;
  deliveryTime?: string;
  receiveTime?: string;
  commentTime?: string;
  delFlag?: string;
  
  
  
  
  }
