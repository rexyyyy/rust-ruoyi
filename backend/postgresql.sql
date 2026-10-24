/*
 Navicat Premium Dump SQL

 Source Server         : localPostgis
 Source Server Type    : PostgreSQL
 Source Server Version : 160007 (160007)
 Source Host           : localhost:5432
 Source Catalog        : ruoyirust
 Source Schema         : public

 Target Server Type    : PostgreSQL
 Target Server Version : 160007 (160007)
 File Encoding         : 65001

 Date: 18/11/2025 16:30:15
*/


-- ----------------------------
-- Table structure for oms_order
-- ----------------------------
DROP TABLE IF EXISTS "public"."oms_order";
CREATE TABLE "public"."oms_order" (
  "id" int8 NOT NULL,
  "member_id" int8,
  "coupon_id" int8,
  "order_sn" varchar(64) COLLATE "pg_catalog"."default",
  "member_username" varchar(64) COLLATE "pg_catalog"."default",
  "total_amount" float8,
  "pay_amount" float8,
  "freight_amount" float8,
  "promotion_amount" float8,
  "integration_amount" float8,
  "coupon_amount" float8,
  "discount_amount" float8,
  "pay_type" int4,
  "source_type" int4,
  "status" int4,
  "order_type" int4,
  "delivery_company" varchar(64) COLLATE "pg_catalog"."default",
  "delivery_sn" varchar(64) COLLATE "pg_catalog"."default",
  "auto_confirm_day" int4,
  "integration" int4,
  "growth" int4,
  "promotion_info" varchar(100) COLLATE "pg_catalog"."default",
  "bill_type" int4,
  "bill_header" varchar(200) COLLATE "pg_catalog"."default",
  "bill_content" varchar(200) COLLATE "pg_catalog"."default",
  "bill_receiver_phone" varchar(32) COLLATE "pg_catalog"."default",
  "bill_receiver_email" varchar(64) COLLATE "pg_catalog"."default",
  "receiver_name" varchar(100) COLLATE "pg_catalog"."default" NOT NULL,
  "receiver_phone" varchar(32) COLLATE "pg_catalog"."default" NOT NULL,
  "receiver_post_code" varchar(32) COLLATE "pg_catalog"."default",
  "receiver_province" varchar(32) COLLATE "pg_catalog"."default",
  "receiver_city" varchar(32) COLLATE "pg_catalog"."default",
  "receiver_region" varchar(32) COLLATE "pg_catalog"."default",
  "receiver_detail_address" varchar(200) COLLATE "pg_catalog"."default",
  "note" varchar(500) COLLATE "pg_catalog"."default",
  "confirm_status" int4,
  "use_integration" int4,
  "payment_time" timestamp(6),
  "delivery_time" timestamp(6),
  "receive_time" timestamp(6),
  "comment_time" timestamp(6),
  "del_flag" char(1) COLLATE "pg_catalog"."default",
  "create_by" varchar(64) COLLATE "pg_catalog"."default",
  "create_time" timestamp(6),
  "update_by" varchar(64) COLLATE "pg_catalog"."default",
  "update_time" timestamp(6)
)
;
COMMENT ON COLUMN "public"."oms_order"."id" IS '订单id';
COMMENT ON COLUMN "public"."oms_order"."order_sn" IS '订单编号';
COMMENT ON COLUMN "public"."oms_order"."member_username" IS '用户帐号';
COMMENT ON COLUMN "public"."oms_order"."total_amount" IS '订单总金额';
COMMENT ON COLUMN "public"."oms_order"."pay_amount" IS '应付金额（实际支付金额）';
COMMENT ON COLUMN "public"."oms_order"."freight_amount" IS '运费金额';
COMMENT ON COLUMN "public"."oms_order"."promotion_amount" IS '促销优化金额（促销价、满减、阶梯价）';
COMMENT ON COLUMN "public"."oms_order"."integration_amount" IS '积分抵扣金额';
COMMENT ON COLUMN "public"."oms_order"."coupon_amount" IS '优惠券抵扣金额';
COMMENT ON COLUMN "public"."oms_order"."discount_amount" IS '管理员后台调整订单使用的折扣金额';
COMMENT ON COLUMN "public"."oms_order"."pay_type" IS '支付方式：0->未支付；1->支付宝；2->微信';
COMMENT ON COLUMN "public"."oms_order"."source_type" IS '订单来源：0->PC订单；1->app订单';
COMMENT ON COLUMN "public"."oms_order"."status" IS '订单状态：0->待付款；1->待发货；2->已发货；3->已完成；4->已关闭；5->无效订单';
COMMENT ON COLUMN "public"."oms_order"."order_type" IS '订单类型：0->正常订单；1->秒杀订单';
COMMENT ON COLUMN "public"."oms_order"."delivery_company" IS '物流公司(配送方式)';
COMMENT ON COLUMN "public"."oms_order"."delivery_sn" IS '物流单号';
COMMENT ON COLUMN "public"."oms_order"."auto_confirm_day" IS '自动确认时间（天）';
COMMENT ON COLUMN "public"."oms_order"."integration" IS '可以获得的积分';
COMMENT ON COLUMN "public"."oms_order"."growth" IS '可以活动的成长值';
COMMENT ON COLUMN "public"."oms_order"."promotion_info" IS '活动信息';
COMMENT ON COLUMN "public"."oms_order"."bill_type" IS '发票类型：0->不开发票；1->电子发票；2->纸质发票';
COMMENT ON COLUMN "public"."oms_order"."bill_header" IS '发票抬头';
COMMENT ON COLUMN "public"."oms_order"."bill_content" IS '发票内容';
COMMENT ON COLUMN "public"."oms_order"."bill_receiver_phone" IS '收票人电话';
COMMENT ON COLUMN "public"."oms_order"."bill_receiver_email" IS '收票人邮箱';
COMMENT ON COLUMN "public"."oms_order"."receiver_name" IS '收货人姓名';
COMMENT ON COLUMN "public"."oms_order"."receiver_phone" IS '收货人电话';
COMMENT ON COLUMN "public"."oms_order"."receiver_post_code" IS '收货人邮编';
COMMENT ON COLUMN "public"."oms_order"."receiver_province" IS '省份/直辖市';
COMMENT ON COLUMN "public"."oms_order"."receiver_city" IS '城市';
COMMENT ON COLUMN "public"."oms_order"."receiver_region" IS '区';
COMMENT ON COLUMN "public"."oms_order"."receiver_detail_address" IS '详细地址';
COMMENT ON COLUMN "public"."oms_order"."note" IS '订单备注';
COMMENT ON COLUMN "public"."oms_order"."confirm_status" IS '确认收货状态：0->未确认；1->已确认';
COMMENT ON COLUMN "public"."oms_order"."use_integration" IS '下单时使用的积分';
COMMENT ON COLUMN "public"."oms_order"."payment_time" IS '支付时间';
COMMENT ON COLUMN "public"."oms_order"."delivery_time" IS '发货时间';
COMMENT ON COLUMN "public"."oms_order"."receive_time" IS '确认收货时间';
COMMENT ON COLUMN "public"."oms_order"."comment_time" IS '评价时间';
COMMENT ON COLUMN "public"."oms_order"."del_flag" IS '删除标志（0代表存在 2代表删除）';
COMMENT ON COLUMN "public"."oms_order"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."oms_order"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."oms_order"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."oms_order"."update_time" IS '更新时间';
COMMENT ON TABLE "public"."oms_order" IS '订单表';

-- ----------------------------
-- Records of oms_order
-- ----------------------------
INSERT INTO "public"."oms_order" VALUES (2025030600001, NULL, NULL, '2025030600001', NULL, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, NULL, NULL, 0, 0, 0, NULL, 0, NULL, NULL, NULL, NULL, '2', '2', NULL, NULL, NULL, NULL, NULL, NULL, 0, NULL, '2025-03-06 04:49:56', NULL, NULL, NULL, NULL, 'admin', '2025-03-06 04:50:05.36', '2025-03-06 04:50:05.36', NULL);

-- ----------------------------
-- Table structure for oms_regorder
-- ----------------------------
DROP TABLE IF EXISTS "public"."oms_regorder";
CREATE TABLE "public"."oms_regorder" (
  "regid" int8 NOT NULL,
  "order_sn" varchar(64) COLLATE "pg_catalog"."default",
  "member_username" varchar(64) COLLATE "pg_catalog"."default",
  "total_amount" float8,
  "pay_amount" float8,
  "pay_type" varchar(2) COLLATE "pg_catalog"."default",
  "source_type" varchar(2) COLLATE "pg_catalog"."default",
  "status" varchar(2) COLLATE "pg_catalog"."default",
  "order_type" varchar(2) COLLATE "pg_catalog"."default",
  "delivery_company" varchar(64) COLLATE "pg_catalog"."default",
  "request_sn" varchar(256) COLLATE "pg_catalog"."default",
  "delivery_sn" varchar(256) COLLATE "pg_catalog"."default",
  "receiver_name" varchar(100) COLLATE "pg_catalog"."default" NOT NULL,
  "receiver_phone" varchar(32) COLLATE "pg_catalog"."default" NOT NULL,
  "receiver_province" varchar(32) COLLATE "pg_catalog"."default",
  "receiver_city" varchar(32) COLLATE "pg_catalog"."default",
  "receiver_region" varchar(32) COLLATE "pg_catalog"."default",
  "note" varchar(500) COLLATE "pg_catalog"."default",
  "confirm_status" varchar(2) COLLATE "pg_catalog"."default",
  "payment_time" timestamp(6),
  "delivery_time" timestamp(6),
  "receive_time" timestamp(6),
  "comment_time" timestamp(6),
  "del_flag" char(1) COLLATE "pg_catalog"."default",
  "create_by" varchar(64) COLLATE "pg_catalog"."default",
  "create_time" timestamp(6),
  "update_by" varchar(64) COLLATE "pg_catalog"."default",
  "update_time" timestamp(6)
)
;
COMMENT ON COLUMN "public"."oms_regorder"."regid" IS '主键';
COMMENT ON COLUMN "public"."oms_regorder"."order_sn" IS '订单号';
COMMENT ON COLUMN "public"."oms_regorder"."member_username" IS '用户名';
COMMENT ON COLUMN "public"."oms_regorder"."total_amount" IS '订单总金额';
COMMENT ON COLUMN "public"."oms_regorder"."pay_amount" IS '应付金额';
COMMENT ON COLUMN "public"."oms_regorder"."pay_type" IS '支付方式：0->未支付；1->支付宝；2->微信';
COMMENT ON COLUMN "public"."oms_regorder"."source_type" IS '订单来源：0->PC订单；1->app订单';
COMMENT ON COLUMN "public"."oms_regorder"."status" IS '订单状态：0->待付款；1->待发货；2->已发货；3->已完成；4->已关闭；5->无效订单';
COMMENT ON COLUMN "public"."oms_regorder"."order_type" IS '订单类型：0->正常订单；1->秒杀订单';
COMMENT ON COLUMN "public"."oms_regorder"."delivery_company" IS '物流公司(配送方式)';
COMMENT ON COLUMN "public"."oms_regorder"."request_sn" IS '申请号';
COMMENT ON COLUMN "public"."oms_regorder"."delivery_sn" IS '回复号';
COMMENT ON COLUMN "public"."oms_regorder"."receiver_name" IS '收货人姓名';
COMMENT ON COLUMN "public"."oms_regorder"."receiver_phone" IS '收货人电话';
COMMENT ON COLUMN "public"."oms_regorder"."receiver_province" IS '省份/直辖市';
COMMENT ON COLUMN "public"."oms_regorder"."receiver_city" IS '城市';
COMMENT ON COLUMN "public"."oms_regorder"."receiver_region" IS '区';
COMMENT ON COLUMN "public"."oms_regorder"."note" IS '订单备注';
COMMENT ON COLUMN "public"."oms_regorder"."confirm_status" IS '确认收货状态：0->未确认；1->已确认';
COMMENT ON COLUMN "public"."oms_regorder"."payment_time" IS '支付时间';
COMMENT ON COLUMN "public"."oms_regorder"."delivery_time" IS '发货时间';
COMMENT ON COLUMN "public"."oms_regorder"."receive_time" IS '确认收货时间';
COMMENT ON COLUMN "public"."oms_regorder"."comment_time" IS '评价时间';
COMMENT ON COLUMN "public"."oms_regorder"."del_flag" IS '删除标志（0代表存在 2代表删除）';
COMMENT ON COLUMN "public"."oms_regorder"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."oms_regorder"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."oms_regorder"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."oms_regorder"."update_time" IS '更新时间';
COMMENT ON TABLE "public"."oms_regorder" IS '注册码登记';

-- ----------------------------
-- Records of oms_regorder
-- ----------------------------
INSERT INTO "public"."oms_regorder" VALUES (7304774654751805440, NULL, 'e', 2, 2, '1', NULL, '4', NULL, NULL, NULL, NULL, '2', '2', NULL, NULL, NULL, NULL, '1', NULL, NULL, NULL, NULL, NULL, 'admin', '2025-03-10 08:06:33.842', NULL, NULL);

-- ----------------------------
-- Table structure for sys_config
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_config";
CREATE TABLE "public"."sys_config" (
  "config_id" varchar(32) COLLATE "pg_catalog"."default" NOT NULL,
  "config_name" varchar(100) COLLATE "pg_catalog"."default",
  "config_key" varchar(100) COLLATE "pg_catalog"."default",
  "config_value" varchar(1000) COLLATE "pg_catalog"."default",
  "config_type" varchar(1) COLLATE "pg_catalog"."default",
  "create_by" varchar(64) COLLATE "pg_catalog"."default",
  "create_time" timestamp(6),
  "update_by" varchar(64) COLLATE "pg_catalog"."default",
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default"
)
;
COMMENT ON COLUMN "public"."sys_config"."config_id" IS '参数主键';
COMMENT ON COLUMN "public"."sys_config"."config_name" IS '参数名称';
COMMENT ON COLUMN "public"."sys_config"."config_key" IS '参数键名';
COMMENT ON COLUMN "public"."sys_config"."config_value" IS '参数键值';
COMMENT ON COLUMN "public"."sys_config"."config_type" IS '系统内置（Y是 N否）';
COMMENT ON COLUMN "public"."sys_config"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_config"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_config"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_config"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_config"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_config" IS '参数配置表';

-- ----------------------------
-- Records of sys_config
-- ----------------------------
INSERT INTO "public"."sys_config" VALUES ('1', '主框架页-默认皮肤样式名称', 'sys.index.skinName', 'skin-blue', 'Y', 'admin', '2021-08-16 10:39:41', '', NULL, '蓝色 skin-blue、绿色 skin-green、紫色 skin-purple、红色 skin-red、黄色 skin-yellow');
INSERT INTO "public"."sys_config" VALUES ('2', '用户管理-账号初始密码', 'sys.user.initPassword', '123456', 'Y', 'admin', '2021-08-16 10:39:41', '', NULL, '初始化密码 123456');
INSERT INTO "public"."sys_config" VALUES ('3', '主框架页-侧边栏主题', 'sys.index.sideTheme', 'theme-light', 'Y', 'admin', '2021-08-16 10:39:41', '', '2022-12-14 22:57:07', '深色主题theme-dark，浅色主题theme-light');
INSERT INTO "public"."sys_config" VALUES ('4', '账号自助-验证码开关', 'sys.account.captchaEnabled', 'false', 'Y', 'admin', '2021-08-16 10:39:41', 'admin', '2021-10-21 20:02:31', '是否开启验证码功能（true开启，false关闭）');
INSERT INTO "public"."sys_config" VALUES ('5', '账号自助-是否开启用户注册功能', 'sys.account.registerUser', 'true', 'Y', 'admin', '2021-08-16 10:39:41', 'admin', '2021-10-21 09:56:18', '是否开启注册用户功能（true开启，false关闭）');
INSERT INTO "public"."sys_config" VALUES ('670f649f0094d88600000000', 'test参数1', 'test1', '1', 'Y', 'admin', '2024-10-16 07:00:47.53', NULL, NULL, '参数1');

-- ----------------------------
-- Table structure for sys_dept
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_dept";
CREATE TABLE "public"."sys_dept" (
  "dept_id" varchar(32) COLLATE "pg_catalog"."default" NOT NULL,
  "parent_id" varchar(32) COLLATE "pg_catalog"."default",
  "ancestors" varchar(200) COLLATE "pg_catalog"."default",
  "dept_name" varchar(30) COLLATE "pg_catalog"."default",
  "order_num" int4,
  "leader" varchar(20) COLLATE "pg_catalog"."default",
  "phone" varchar(11) COLLATE "pg_catalog"."default",
  "email" varchar(50) COLLATE "pg_catalog"."default",
  "status" varchar(1) COLLATE "pg_catalog"."default",
  "del_flag" varchar(1) COLLATE "pg_catalog"."default",
  "create_by" varchar(64) COLLATE "pg_catalog"."default",
  "create_time" timestamp(6),
  "update_by" varchar(64) COLLATE "pg_catalog"."default",
  "update_time" timestamp(6)
)
;
COMMENT ON COLUMN "public"."sys_dept"."dept_id" IS '部门id';
COMMENT ON COLUMN "public"."sys_dept"."parent_id" IS '父部门id';
COMMENT ON COLUMN "public"."sys_dept"."ancestors" IS '祖级列表';
COMMENT ON COLUMN "public"."sys_dept"."dept_name" IS '部门名称';
COMMENT ON COLUMN "public"."sys_dept"."order_num" IS '显示顺序';
COMMENT ON COLUMN "public"."sys_dept"."leader" IS '负责人';
COMMENT ON COLUMN "public"."sys_dept"."phone" IS '联系电话';
COMMENT ON COLUMN "public"."sys_dept"."email" IS '邮箱';
COMMENT ON COLUMN "public"."sys_dept"."status" IS '部门状态（0正常 1停用）';
COMMENT ON COLUMN "public"."sys_dept"."del_flag" IS '删除标志（0代表存在 2代表删除）';
COMMENT ON COLUMN "public"."sys_dept"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_dept"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_dept"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_dept"."update_time" IS '更新时间';
COMMENT ON TABLE "public"."sys_dept" IS '部门表';

-- ----------------------------
-- Records of sys_dept
-- ----------------------------
INSERT INTO "public"."sys_dept" VALUES ('100', '0', '0', '若依科技', 0, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2023-02-21 10:31:16', '', NULL);
INSERT INTO "public"."sys_dept" VALUES ('102', '100', '0,100', '长沙分公司', 2, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2023-02-21 10:31:16', '', NULL);
INSERT INTO "public"."sys_dept" VALUES ('103', '101', '0,100,101', '研发部门', 1, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2023-02-21 10:31:16', '', NULL);
INSERT INTO "public"."sys_dept" VALUES ('104', '101', '0,100,101', '市场部门', 2, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2023-02-21 10:31:16', '', NULL);
INSERT INTO "public"."sys_dept" VALUES ('105', '101', '0,100,101', '测试部门', 3, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2023-02-21 10:31:16', '', NULL);
INSERT INTO "public"."sys_dept" VALUES ('106', '101', '0,100,101', '财务部门', 4, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2023-02-21 10:31:16', '', NULL);
INSERT INTO "public"."sys_dept" VALUES ('107', '101', '0,100,101', '运维部门', 5, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2023-02-21 10:31:16', '', NULL);
INSERT INTO "public"."sys_dept" VALUES ('108', '102', '0,100,102', '市场部门', 1, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2023-02-21 10:31:16', '', NULL);
INSERT INTO "public"."sys_dept" VALUES ('109', '102', '0,100,102', '财务部门', 2, '若依', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2023-02-21 10:31:16', '', NULL);
INSERT INTO "public"."sys_dept" VALUES ('101', '100', '0,100', '深圳总公司', 1, 'rex', '15888888888', 'ry@qq.com', '0', '0', 'admin', '2023-02-21 10:31:16', 'admin', '2024-10-16 07:00:03.844');

-- ----------------------------
-- Table structure for sys_dict_data
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_dict_data";
CREATE TABLE "public"."sys_dict_data" (
  "dict_code" varchar(32) COLLATE "pg_catalog"."default" NOT NULL,
  "dict_sort" int4,
  "dict_label" varchar(100) COLLATE "pg_catalog"."default",
  "dict_value" varchar(100) COLLATE "pg_catalog"."default",
  "dict_type" varchar(100) COLLATE "pg_catalog"."default",
  "css_class" varchar(100) COLLATE "pg_catalog"."default",
  "list_class" varchar(100) COLLATE "pg_catalog"."default",
  "is_default" varchar(1) COLLATE "pg_catalog"."default",
  "status" varchar(2) COLLATE "pg_catalog"."default",
  "create_by" varchar(64) COLLATE "pg_catalog"."default",
  "create_time" timestamp(6),
  "update_by" varchar(64) COLLATE "pg_catalog"."default",
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default"
)
;
COMMENT ON COLUMN "public"."sys_dict_data"."dict_code" IS '字典编码';
COMMENT ON COLUMN "public"."sys_dict_data"."dict_sort" IS '字典排序';
COMMENT ON COLUMN "public"."sys_dict_data"."dict_label" IS '字典标签';
COMMENT ON COLUMN "public"."sys_dict_data"."dict_value" IS '字典键值';
COMMENT ON COLUMN "public"."sys_dict_data"."dict_type" IS '字典类型';
COMMENT ON COLUMN "public"."sys_dict_data"."css_class" IS '样式属性（其他样式扩展）';
COMMENT ON COLUMN "public"."sys_dict_data"."list_class" IS '表格回显样式';
COMMENT ON COLUMN "public"."sys_dict_data"."is_default" IS '是否默认（Y是 N否）';
COMMENT ON COLUMN "public"."sys_dict_data"."status" IS '状态（0正常 1停用）';
COMMENT ON COLUMN "public"."sys_dict_data"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_dict_data"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_dict_data"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_dict_data"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_dict_data"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_dict_data" IS '字典数据表';

-- ----------------------------
-- Records of sys_dict_data
-- ----------------------------
INSERT INTO "public"."sys_dict_data" VALUES ('1', 1, '男', '0', 'sys_user_sex', '', '', 'Y', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '性别男');
INSERT INTO "public"."sys_dict_data" VALUES ('10', 1, '默认', 'DEFAULT', 'sys_job_group', '', '', 'Y', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '默认分组');
INSERT INTO "public"."sys_dict_data" VALUES ('105', 0, '是', '1', 'sys_boolean', NULL, 'default', 'N', '0', 'admin', '2021-09-15 21:54:11', 'admin', '2021-09-18 15:59:55', NULL);
INSERT INTO "public"."sys_dict_data" VALUES ('106', 1, '否', '0', 'sys_boolean', NULL, 'default', 'N', '0', 'admin', '2021-09-15 21:54:26', 'admin', '2021-09-18 16:00:01', NULL);
INSERT INTO "public"."sys_dict_data" VALUES ('11', 2, '系统', 'SYSTEM', 'sys_job_group', '', '', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '系统分组');
INSERT INTO "public"."sys_dict_data" VALUES ('12', 1, '是', 'Y', 'sys_yes_no', '', 'primary', 'Y', '0', 'admin', '2021-08-16 10:39:41', 'admin', '2021-09-13 23:05:19', '系统默认是');
INSERT INTO "public"."sys_dict_data" VALUES ('13', 2, '否', 'N', 'sys_yes_no', '', 'danger', 'N', '0', 'admin', '2021-08-16 10:39:41', 'admin', '2021-09-13 23:05:34', '系统默认否');
INSERT INTO "public"."sys_dict_data" VALUES ('14', 1, '通知', '1', 'sys_notice_type', '', 'warning', 'Y', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '通知');
INSERT INTO "public"."sys_dict_data" VALUES ('15', 2, '公告', '2', 'sys_notice_type', '', 'success', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '公告');
INSERT INTO "public"."sys_dict_data" VALUES ('16', 1, '正常', '0', 'sys_notice_status', '', 'primary', 'Y', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '正常状态');
INSERT INTO "public"."sys_dict_data" VALUES ('17', 2, '关闭', '1', 'sys_notice_status', '', 'danger', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '关闭状态');
INSERT INTO "public"."sys_dict_data" VALUES ('18', 1, '新增', '1', 'sys_oper_type', '', 'info', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '新增操作');
INSERT INTO "public"."sys_dict_data" VALUES ('19', 2, '修改', '2', 'sys_oper_type', '', 'info', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '修改操作');
INSERT INTO "public"."sys_dict_data" VALUES ('2', 2, '女', '1', 'sys_user_sex', '', '', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '性别女');
INSERT INTO "public"."sys_dict_data" VALUES ('20', 3, '删除', '3', 'sys_oper_type', '', 'danger', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '删除操作');
INSERT INTO "public"."sys_dict_data" VALUES ('21', 4, '授权', '4', 'sys_oper_type', '', 'primary', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '授权操作');
INSERT INTO "public"."sys_dict_data" VALUES ('22', 5, '导出', '5', 'sys_oper_type', '', 'warning', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '导出操作');
INSERT INTO "public"."sys_dict_data" VALUES ('23', 6, '导入', '6', 'sys_oper_type', '', 'warning', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '导入操作');
INSERT INTO "public"."sys_dict_data" VALUES ('24', 7, '强退', '7', 'sys_oper_type', '', 'danger', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '强退操作');
INSERT INTO "public"."sys_dict_data" VALUES ('25', 8, '生成代码', '8', 'sys_oper_type', '', 'warning', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '生成操作');
INSERT INTO "public"."sys_dict_data" VALUES ('26', 9, '清空数据', '9', 'sys_oper_type', '', 'danger', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '清空操作');
INSERT INTO "public"."sys_dict_data" VALUES ('27', 1, '成功', '0', 'sys_common_status', '', 'primary', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '正常状态');
INSERT INTO "public"."sys_dict_data" VALUES ('28', 2, '失败', '1', 'sys_common_status', '', 'danger', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '停用状态');
INSERT INTO "public"."sys_dict_data" VALUES ('3', 3, '未知', '2', 'sys_user_sex', '', '', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '性别未知');
INSERT INTO "public"."sys_dict_data" VALUES ('4', 1, '显示', '0', 'sys_show_hide', '', 'primary', 'Y', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '显示菜单');
INSERT INTO "public"."sys_dict_data" VALUES ('5', 2, '隐藏', '1', 'sys_show_hide', '', 'danger', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '隐藏菜单');
INSERT INTO "public"."sys_dict_data" VALUES ('6', 1, '正常', '0', 'sys_normal_disable', '', 'primary', 'Y', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '正常状态');
INSERT INTO "public"."sys_dict_data" VALUES ('7', 2, '停用', '1', 'sys_normal_disable', '', 'danger', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '停用状态');
INSERT INTO "public"."sys_dict_data" VALUES ('8', 1, '正常', '0', 'sys_job_status', '', 'primary', 'Y', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '正常状态');
INSERT INTO "public"."sys_dict_data" VALUES ('9', 2, '暂停', '1', 'sys_job_status', '', 'danger', 'N', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '停用状态');
INSERT INTO "public"."sys_dict_data" VALUES ('67c91abe0004fbed00000001', 0, '未支付', '0', 'order_pay_methd', NULL, 'default', NULL, '0', 'admin', NULL, NULL, NULL, '未支付');
INSERT INTO "public"."sys_dict_data" VALUES ('67c91ace0004fbed00000002', 0, '支付宝', '1', 'order_pay_methd', NULL, 'default', NULL, '0', 'admin', NULL, 'admin', NULL, '支付宝');
INSERT INTO "public"."sys_dict_data" VALUES ('67c91ae90004fbed00000003', 0, '微信', '2', 'order_pay_methd', NULL, 'default', NULL, '0', 'admin', NULL, NULL, NULL, '微信');
INSERT INTO "public"."sys_dict_data" VALUES ('67c91b1f0004fbed00000004', 0, '网银', '3', 'order_pay_methd', NULL, 'default', NULL, '0', 'admin', NULL, NULL, NULL, NULL);
INSERT INTO "public"."sys_dict_data" VALUES ('67c920770007782e00000000', 0, 'pc直单', '0', 'ord_source', NULL, 'default', NULL, '0', 'admin', NULL, NULL, NULL, NULL);
INSERT INTO "public"."sys_dict_data" VALUES ('67c920870007782e00000001', 0, 'app直单', '1', 'ord_source', NULL, 'default', NULL, '0', 'admin', NULL, NULL, NULL, NULL);
INSERT INTO "public"."sys_dict_data" VALUES ('67c920e50007782e00000003', 0, '待付款', '0', 'ord_statu', NULL, 'default', NULL, '0', 'admin', NULL, NULL, NULL, NULL);
INSERT INTO "public"."sys_dict_data" VALUES ('67c920f30007782e00000004', 0, '待发货', '1', 'ord_statu', NULL, 'default', NULL, '0', 'admin', NULL, NULL, NULL, NULL);
INSERT INTO "public"."sys_dict_data" VALUES ('67c921040007782e00000005', 0, '已发货', '2', 'ord_statu', NULL, 'default', NULL, '0', 'admin', NULL, NULL, NULL, NULL);
INSERT INTO "public"."sys_dict_data" VALUES ('67c921170007782e00000006', 0, '已完成', '3', 'ord_statu', NULL, 'default', NULL, '0', 'admin', NULL, NULL, NULL, NULL);
INSERT INTO "public"."sys_dict_data" VALUES ('67c921240007782e00000007', 0, '已关闭', '4', 'ord_statu', NULL, 'default', NULL, '0', 'admin', NULL, 'admin', NULL, NULL);
INSERT INTO "public"."sys_dict_data" VALUES ('67c9218a0007782e00000009', 0, '未确认', '0', 'ord_receiv', NULL, 'default', NULL, '0', 'admin', NULL, NULL, NULL, NULL);
INSERT INTO "public"."sys_dict_data" VALUES ('67c921910007782e0000000a', 0, '已确认', '1', 'ord_receiv', NULL, 'default', NULL, '0', 'admin', NULL, NULL, NULL, NULL);
INSERT INTO "public"."sys_dict_data" VALUES ('67c923f00007782e0000000b', 0, '无效订单', '5', 'ord_statu', NULL, 'default', NULL, '0', 'admin', NULL, NULL, NULL, NULL);

-- ----------------------------
-- Table structure for sys_dict_type
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_dict_type";
CREATE TABLE "public"."sys_dict_type" (
  "dict_id" varchar(32) COLLATE "pg_catalog"."default" NOT NULL,
  "dict_name" varchar(100) COLLATE "pg_catalog"."default",
  "dict_type" varchar(100) COLLATE "pg_catalog"."default",
  "status" varchar(1) COLLATE "pg_catalog"."default",
  "create_by" varchar(64) COLLATE "pg_catalog"."default",
  "create_time" timestamp(6),
  "update_by" varchar(64) COLLATE "pg_catalog"."default",
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default"
)
;
COMMENT ON COLUMN "public"."sys_dict_type"."dict_id" IS '字典主键';
COMMENT ON COLUMN "public"."sys_dict_type"."dict_name" IS '字典名称';
COMMENT ON COLUMN "public"."sys_dict_type"."dict_type" IS '字典类型';
COMMENT ON COLUMN "public"."sys_dict_type"."status" IS '状态（0正常 1停用）';
COMMENT ON COLUMN "public"."sys_dict_type"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_dict_type"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_dict_type"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_dict_type"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_dict_type"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_dict_type" IS '字典类型表';

-- ----------------------------
-- Records of sys_dict_type
-- ----------------------------
INSERT INTO "public"."sys_dict_type" VALUES ('10', '系统状态', 'sys_common_status', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '登录状态列表');
INSERT INTO "public"."sys_dict_type" VALUES ('101', '布尔值', 'sys_boolean', '0', 'admin', '2021-09-15 21:53:34', '', NULL, NULL);
INSERT INTO "public"."sys_dict_type" VALUES ('2', '菜单状态', 'sys_show_hide', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '菜单状态列表');
INSERT INTO "public"."sys_dict_type" VALUES ('3', '系统开关', 'sys_normal_disable', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '系统开关列表');
INSERT INTO "public"."sys_dict_type" VALUES ('4', '任务状态', 'sys_job_status', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '任务状态列表');
INSERT INTO "public"."sys_dict_type" VALUES ('5', '任务分组', 'sys_job_group', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '任务分组列表');
INSERT INTO "public"."sys_dict_type" VALUES ('6', '系统是否', 'sys_yes_no', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '系统是否列表');
INSERT INTO "public"."sys_dict_type" VALUES ('7', '通知类型', 'sys_notice_type', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '通知类型列表');
INSERT INTO "public"."sys_dict_type" VALUES ('8', '通知状态', 'sys_notice_status', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '通知状态列表');
INSERT INTO "public"."sys_dict_type" VALUES ('9', '操作类型', 'sys_oper_type', '0', 'admin', '2021-08-16 10:39:41', '', NULL, '操作类型列表');
INSERT INTO "public"."sys_dict_type" VALUES ('67c914760004fbed00000000', '支付方式', 'order_pay_methd', '0', 'admin', '2025-03-06 11:20:22.826577', NULL, NULL, '支付方式');
INSERT INTO "public"."sys_dict_type" VALUES ('67c91b520004fbed00000005', '订单来源', 'ord_source', '0', 'admin', '2025-03-06 03:49:38.381', NULL, NULL, '订单来源');
INSERT INTO "public"."sys_dict_type" VALUES ('67c920a60007782e00000002', '订单状态', 'ord_statu', '0', 'admin', '2025-03-06 04:12:22.82', NULL, NULL, NULL);
INSERT INTO "public"."sys_dict_type" VALUES ('67c921710007782e00000008', '确认收货状态', 'ord_receiv', '0', 'admin', '2025-03-06 04:15:45.742', NULL, NULL, NULL);

-- ----------------------------
-- Table structure for sys_job
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_job";
CREATE TABLE "public"."sys_job" (
  "job_id" int8 NOT NULL,
  "job_name" varchar(64) COLLATE "pg_catalog"."default" NOT NULL,
  "job_group" varchar(64) COLLATE "pg_catalog"."default" NOT NULL,
  "invoke_target" varchar(500) COLLATE "pg_catalog"."default" NOT NULL,
  "cron_expression" varchar(255) COLLATE "pg_catalog"."default",
  "misfire_policy" varchar(20) COLLATE "pg_catalog"."default",
  "concurrent" varchar(1) COLLATE "pg_catalog"."default",
  "status" varchar(1) COLLATE "pg_catalog"."default",
  "create_by" varchar(64) COLLATE "pg_catalog"."default",
  "create_time" timestamp(6),
  "update_by" varchar(64) COLLATE "pg_catalog"."default",
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default"
)
;
COMMENT ON COLUMN "public"."sys_job"."job_id" IS '任务ID';
COMMENT ON COLUMN "public"."sys_job"."job_name" IS '任务名称';
COMMENT ON COLUMN "public"."sys_job"."job_group" IS '任务组名';
COMMENT ON COLUMN "public"."sys_job"."invoke_target" IS '调用目标字符串';
COMMENT ON COLUMN "public"."sys_job"."cron_expression" IS 'cron执行表达式';
COMMENT ON COLUMN "public"."sys_job"."misfire_policy" IS '计划执行错误策略（1立即执行 2执行一次 3放弃执行）';
COMMENT ON COLUMN "public"."sys_job"."concurrent" IS '是否并发执行（0允许 1禁止）';
COMMENT ON COLUMN "public"."sys_job"."status" IS '状态（0正常 1暂停）';
COMMENT ON COLUMN "public"."sys_job"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_job"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_job"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_job"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_job"."remark" IS '备注信息';
COMMENT ON TABLE "public"."sys_job" IS '定时任务调度表';

-- ----------------------------
-- Records of sys_job
-- ----------------------------
INSERT INTO "public"."sys_job" VALUES (1, '系统默认（无参）', 'DEFAULT', 'ryTask.ryNoParams', '0/10 * * * * ?', '3', '1', '1', 'admin', '2021-08-16 10:39:41', '', NULL, '');
INSERT INTO "public"."sys_job" VALUES (2, '系统默认（有参）', 'DEFAULT', 'ryTask.ryParams(''ry'')', '0/15 * * * * ?', '3', '1', '1', 'admin', '2021-08-16 10:39:41', '', NULL, '');
INSERT INTO "public"."sys_job" VALUES (3, '系统默认（多参）', 'DEFAULT', 'ryTask.ryMultipleParams(''ry'', true, 2000L, 316.50D, 100)', '0/20 * * * * ?', '3', '1', '1', 'admin', '2021-08-16 10:39:41', '', NULL, '');

-- ----------------------------
-- Table structure for sys_job_log
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_job_log";
CREATE TABLE "public"."sys_job_log" (
  "job_log_id" int8 NOT NULL,
  "job_name" varchar(64) COLLATE "pg_catalog"."default" NOT NULL,
  "job_group" varchar(64) COLLATE "pg_catalog"."default" NOT NULL,
  "invoke_target" varchar(500) COLLATE "pg_catalog"."default" NOT NULL,
  "job_message" varchar(500) COLLATE "pg_catalog"."default",
  "status" varchar(1) COLLATE "pg_catalog"."default",
  "exception_info" varchar(2000) COLLATE "pg_catalog"."default",
  "create_time" timestamp(6)
)
;
COMMENT ON COLUMN "public"."sys_job_log"."job_log_id" IS '任务日志ID';
COMMENT ON COLUMN "public"."sys_job_log"."job_name" IS '任务名称';
COMMENT ON COLUMN "public"."sys_job_log"."job_group" IS '任务组名';
COMMENT ON COLUMN "public"."sys_job_log"."invoke_target" IS '调用目标字符串';
COMMENT ON COLUMN "public"."sys_job_log"."job_message" IS '日志信息';
COMMENT ON COLUMN "public"."sys_job_log"."status" IS '执行状态（0正常 1失败）';
COMMENT ON COLUMN "public"."sys_job_log"."exception_info" IS '异常信息';
COMMENT ON COLUMN "public"."sys_job_log"."create_time" IS '创建时间';
COMMENT ON TABLE "public"."sys_job_log" IS '定时任务调度日志表';

-- ----------------------------
-- Records of sys_job_log
-- ----------------------------

-- ----------------------------
-- Table structure for sys_logininfor
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_logininfor";
CREATE TABLE "public"."sys_logininfor" (
  "info_id" varchar(32) COLLATE "pg_catalog"."default" NOT NULL,
  "user_name" varchar(50) COLLATE "pg_catalog"."default",
  "ipaddr" varchar(128) COLLATE "pg_catalog"."default",
  "login_location" varchar(255) COLLATE "pg_catalog"."default",
  "browser" varchar(50) COLLATE "pg_catalog"."default",
  "os" varchar(50) COLLATE "pg_catalog"."default",
  "status" varchar(1) COLLATE "pg_catalog"."default",
  "msg" varchar(255) COLLATE "pg_catalog"."default",
  "login_time" timestamp(6)
)
;
COMMENT ON COLUMN "public"."sys_logininfor"."info_id" IS '访问ID';
COMMENT ON COLUMN "public"."sys_logininfor"."user_name" IS '用户账号';
COMMENT ON COLUMN "public"."sys_logininfor"."ipaddr" IS '登录IP地址';
COMMENT ON COLUMN "public"."sys_logininfor"."login_location" IS '登录地点';
COMMENT ON COLUMN "public"."sys_logininfor"."browser" IS '浏览器类型';
COMMENT ON COLUMN "public"."sys_logininfor"."os" IS '操作系统';
COMMENT ON COLUMN "public"."sys_logininfor"."status" IS '登录状态（0成功 1失败）';
COMMENT ON COLUMN "public"."sys_logininfor"."msg" IS '提示消息';
COMMENT ON COLUMN "public"."sys_logininfor"."login_time" IS '访问时间';
COMMENT ON TABLE "public"."sys_logininfor" IS '系统访问记录';

-- ----------------------------
-- Records of sys_logininfor
-- ----------------------------
INSERT INTO "public"."sys_logininfor" VALUES ('648c257000e9dfe4000da3db', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2023-06-16 17:03:44');
INSERT INTO "public"."sys_logininfor" VALUES ('648c430000e9dfe4000da3f4', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2023-06-16 19:09:52');
INSERT INTO "public"."sys_logininfor" VALUES ('648c430400e9dfe4000da3f5', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2023-06-16 19:09:56');
INSERT INTO "public"."sys_logininfor" VALUES ('648c690900e9dfe4000da3f6', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2023-06-16 21:52:09');
INSERT INTO "public"."sys_logininfor" VALUES ('648c692c00e9dfe4000da3f7', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2023-06-16 21:52:44');
INSERT INTO "public"."sys_logininfor" VALUES ('648c693200e9dfe4000da3f8', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2023-06-16 21:52:50');
INSERT INTO "public"."sys_logininfor" VALUES ('670f33cf00ac183900000000', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 03:32:31');
INSERT INTO "public"."sys_logininfor" VALUES ('670f55f200715a8400000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-16 05:58:10');
INSERT INTO "public"."sys_logininfor" VALUES ('670f55f800715a8400000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 05:58:16');
INSERT INTO "public"."sys_logininfor" VALUES ('670f6c9e005b309f00000000', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 07:34:54');
INSERT INTO "public"."sys_logininfor" VALUES ('670f7579008af78100000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-16 08:12:41');
INSERT INTO "public"."sys_logininfor" VALUES ('670f757d008af78100000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 08:12:45');
INSERT INTO "public"."sys_logininfor" VALUES ('670f75fb00ed081f00000000', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 08:14:51');
INSERT INTO "public"."sys_logininfor" VALUES ('670f760700ed081f00000001', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-16 08:15:03');
INSERT INTO "public"."sys_logininfor" VALUES ('670f761200ed081f00000002', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 08:15:14');
INSERT INTO "public"."sys_logininfor" VALUES ('670f780500ed081f00000003', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-16 08:23:33');
INSERT INTO "public"."sys_logininfor" VALUES ('670f780800ed081f00000004', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 08:23:36');
INSERT INTO "public"."sys_logininfor" VALUES ('670f785e00ed081f00000005', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-16 08:25:02');
INSERT INTO "public"."sys_logininfor" VALUES ('670f786000ed081f00000006', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 08:25:04');
INSERT INTO "public"."sys_logininfor" VALUES ('670f7f8400ba366b00000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-16 08:55:32');
INSERT INTO "public"."sys_logininfor" VALUES ('670f7f8600ba366b00000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 08:55:34');
INSERT INTO "public"."sys_logininfor" VALUES ('670f801900f1d71100000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-16 08:58:01');
INSERT INTO "public"."sys_logininfor" VALUES ('670f801b00f1d71100000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 08:58:03');
INSERT INTO "public"."sys_logininfor" VALUES ('670f808f00a033c500000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-16 08:59:59');
INSERT INTO "public"."sys_logininfor" VALUES ('670f809100a033c500000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 09:00:01');
INSERT INTO "public"."sys_logininfor" VALUES ('670f820500d1e2a500000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-16 09:06:13');
INSERT INTO "public"."sys_logininfor" VALUES ('670f820700d1e2a500000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '1', '密码不正确!', '2024-10-16 09:06:15');
INSERT INTO "public"."sys_logininfor" VALUES ('670f85780049140e00000000', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '1', '密码不正确!', '2024-10-16 09:20:56');
INSERT INTO "public"."sys_logininfor" VALUES ('670f85c00049140e00000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 09:22:08');
INSERT INTO "public"."sys_logininfor" VALUES ('670f85d00049140e00000002', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-16 09:22:24');
INSERT INTO "public"."sys_logininfor" VALUES ('670f85d10049140e00000003', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 09:22:25');
INSERT INTO "public"."sys_logininfor" VALUES ('670f85d90049140e00000004', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-16 09:22:33');
INSERT INTO "public"."sys_logininfor" VALUES ('670f85da0049140e00000005', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 09:22:34');
INSERT INTO "public"."sys_logininfor" VALUES ('670f87fa00b1b80f00000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-16 09:31:38');
INSERT INTO "public"."sys_logininfor" VALUES ('670f88730073528e00000000', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 09:33:39');
INSERT INTO "public"."sys_logininfor" VALUES ('670f887d0073528e00000001', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-16 09:33:49');
INSERT INTO "public"."sys_logininfor" VALUES ('670f88830073528e00000002', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 09:33:55');
INSERT INTO "public"."sys_logininfor" VALUES ('670f8a130073528e00000003', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-16 09:40:35');
INSERT INTO "public"."sys_logininfor" VALUES ('670f8a1f0073528e00000004', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 09:40:47');
INSERT INTO "public"."sys_logininfor" VALUES ('670f8cec00d5455600000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-16 09:52:44');
INSERT INTO "public"."sys_logininfor" VALUES ('670f8cf400d5455600000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 09:52:52');
INSERT INTO "public"."sys_logininfor" VALUES ('670f8cfb00d5455600000002', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-16 09:52:59');
INSERT INTO "public"."sys_logininfor" VALUES ('670f8cff00d5455600000003', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-16 09:53:03');
INSERT INTO "public"."sys_logininfor" VALUES ('67106a88005b236a00000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2024-10-17 01:38:16');
INSERT INTO "public"."sys_logininfor" VALUES ('67106a90005b236a00000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2024-10-17 01:38:24');
INSERT INTO "public"."sys_logininfor" VALUES ('67bbe6b4009b042800000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-02-24 03:25:40');
INSERT INTO "public"."sys_logininfor" VALUES ('67bbe6bc009b042800000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-02-24 03:25:48');
INSERT INTO "public"."sys_logininfor" VALUES ('67c7ebbb00066e5c00000000', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-03-05 06:14:19');
INSERT INTO "public"."sys_logininfor" VALUES ('67c81722006d4ade00000000', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-03-05 09:19:30');
INSERT INTO "public"."sys_logininfor" VALUES ('67c8172e006d4ade00000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-03-05 09:19:42');
INSERT INTO "public"."sys_logininfor" VALUES ('67c8feb70061512a00000000', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-03-06 01:47:35');
INSERT INTO "public"."sys_logininfor" VALUES ('67c9407200f9dc9e00000000', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-03-06 06:28:02');
INSERT INTO "public"."sys_logininfor" VALUES ('68905bf00042feb900000000', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-08-04 07:06:24');
INSERT INTO "public"."sys_logininfor" VALUES ('6912988800e769cc00000000', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-11 01:59:36');
INSERT INTO "public"."sys_logininfor" VALUES ('6912a36b00e769cc00000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-11 02:46:03');
INSERT INTO "public"."sys_logininfor" VALUES ('6912bab90016342a00000000', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-11-11 04:25:29');
INSERT INTO "public"."sys_logininfor" VALUES ('6912c3c80016342a00000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-11-11 05:04:08');
INSERT INTO "public"."sys_logininfor" VALUES ('6912c3cd0016342a00000002', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-11 05:04:13');
INSERT INTO "public"."sys_logininfor" VALUES ('6912c7b30016342a00000003', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-11 05:20:51');
INSERT INTO "public"."sys_logininfor" VALUES ('6912d5e50068fbc500000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-11-11 06:21:25');
INSERT INTO "public"."sys_logininfor" VALUES ('6912d5ee0068fbc500000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-11 06:21:34');
INSERT INTO "public"."sys_logininfor" VALUES ('6912f40f00ffad6900000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-11-11 08:30:07');
INSERT INTO "public"."sys_logininfor" VALUES ('6912f41e00ffad6900000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-11 08:30:22');
INSERT INTO "public"."sys_logininfor" VALUES ('6913df3b00a434ac00000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-11-12 01:13:31');
INSERT INTO "public"."sys_logininfor" VALUES ('6913df4600a434ac00000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-12 01:13:42');
INSERT INTO "public"."sys_logininfor" VALUES ('69141bb500d13f5c00000001', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-11-12 05:31:33');
INSERT INTO "public"."sys_logininfor" VALUES ('69141bfb00d13f5c00000002', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-12 05:32:43');
INSERT INTO "public"."sys_logininfor" VALUES ('69141c0100d13f5c00000003', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-12 05:32:49');
INSERT INTO "public"."sys_logininfor" VALUES ('6914239c00f62c0d00000002', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-11-12 06:05:16');
INSERT INTO "public"."sys_logininfor" VALUES ('691423a500f62c0d00000003', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-12 06:05:25');
INSERT INTO "public"."sys_logininfor" VALUES ('69152aa3006eeeba00000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-11-13 00:47:31');
INSERT INTO "public"."sys_logininfor" VALUES ('69152aaa006eeeba00000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-13 00:47:38');
INSERT INTO "public"."sys_logininfor" VALUES ('69154527006eeeba00000002', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-11-13 02:40:39');
INSERT INTO "public"."sys_logininfor" VALUES ('691ac4f1002a6f1100000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-11-17 06:47:13');
INSERT INTO "public"."sys_logininfor" VALUES ('691ac4f9002a6f1100000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-17 06:47:21');
INSERT INTO "public"."sys_logininfor" VALUES ('691aeba7009b5b4a00000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-11-17 09:32:23');
INSERT INTO "public"."sys_logininfor" VALUES ('691aebb0009b5b4a00000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-17 09:32:32');
INSERT INTO "public"."sys_logininfor" VALUES ('691aecc4009b5b4a00000002', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-17 09:37:08');
INSERT INTO "public"."sys_logininfor" VALUES ('691bc7d000cfd68e00000000', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-18 01:11:44');
INSERT INTO "public"."sys_logininfor" VALUES ('691bf58d00b4b07d00000000', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-11-18 04:26:53');
INSERT INTO "public"."sys_logininfor" VALUES ('691bf59d00b4b07d00000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-18 04:27:09');
INSERT INTO "public"."sys_logininfor" VALUES ('691c088500d6d35700000000', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-11-18 05:47:49');
INSERT INTO "public"."sys_logininfor" VALUES ('691c088d00d6d35700000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-18 05:47:57');
INSERT INTO "public"."sys_logininfor" VALUES ('691c177e00f4471200000000', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-11-18 06:51:42');
INSERT INTO "public"."sys_logininfor" VALUES ('691c187500f4471200000001', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '退出成功', '2025-11-18 06:55:49');
INSERT INTO "public"."sys_logininfor" VALUES ('691c18a00033a65000000000', 'admin', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2025-11-18 06:56:32');

-- ----------------------------
-- Table structure for sys_menu
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_menu";
CREATE TABLE "public"."sys_menu" (
  "menu_id" int8 NOT NULL,
  "menu_name" varchar(50) COLLATE "pg_catalog"."default" NOT NULL,
  "parent_id" int8,
  "order_num" int4,
  "path" varchar(200) COLLATE "pg_catalog"."default",
  "component" varchar(255) COLLATE "pg_catalog"."default",
  "query" varchar(255) COLLATE "pg_catalog"."default",
  "is_frame" varchar(1) COLLATE "pg_catalog"."default",
  "is_cache" varchar(1) COLLATE "pg_catalog"."default",
  "menu_type" varchar(1) COLLATE "pg_catalog"."default",
  "visible" varchar(1) COLLATE "pg_catalog"."default",
  "status" varchar(1) COLLATE "pg_catalog"."default",
  "perms" varchar(100) COLLATE "pg_catalog"."default",
  "icon" varchar(100) COLLATE "pg_catalog"."default",
  "create_by" varchar(64) COLLATE "pg_catalog"."default",
  "create_time" timestamp(6),
  "update_by" varchar(64) COLLATE "pg_catalog"."default",
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default"
)
;
COMMENT ON COLUMN "public"."sys_menu"."menu_id" IS '菜单ID';
COMMENT ON COLUMN "public"."sys_menu"."menu_name" IS '菜单名称';
COMMENT ON COLUMN "public"."sys_menu"."parent_id" IS '父菜单ID';
COMMENT ON COLUMN "public"."sys_menu"."order_num" IS '显示顺序';
COMMENT ON COLUMN "public"."sys_menu"."path" IS '路由地址';
COMMENT ON COLUMN "public"."sys_menu"."component" IS '组件路径';
COMMENT ON COLUMN "public"."sys_menu"."query" IS '路由参数';
COMMENT ON COLUMN "public"."sys_menu"."is_frame" IS '是否为外链（0是 1否）';
COMMENT ON COLUMN "public"."sys_menu"."is_cache" IS '是否缓存（0缓存 1不缓存）';
COMMENT ON COLUMN "public"."sys_menu"."menu_type" IS '菜单类型（M目录 C菜单 F按钮）';
COMMENT ON COLUMN "public"."sys_menu"."visible" IS '菜单状态（0显示 1隐藏）';
COMMENT ON COLUMN "public"."sys_menu"."status" IS '菜单状态（0正常 1停用）';
COMMENT ON COLUMN "public"."sys_menu"."perms" IS '权限标识';
COMMENT ON COLUMN "public"."sys_menu"."icon" IS '菜单图标';
COMMENT ON COLUMN "public"."sys_menu"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_menu"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_menu"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_menu"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_menu"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_menu" IS '菜单权限表';

-- ----------------------------
-- Records of sys_menu
-- ----------------------------
INSERT INTO "public"."sys_menu" VALUES (1, '系统管理', 0, 3, 'system', NULL, '', '1', '0', 'M', '0', '0', '', 'system', 'admin', '2022-12-21 20:22:45', 'admin', '2023-04-10 15:26:27', '系统管理目录');
INSERT INTO "public"."sys_menu" VALUES (2, '系统监控', 0, 4, 'monitor', NULL, '', '1', '0', 'M', '0', '0', '', 'monitor', 'admin', '2022-12-21 20:22:45', 'admin', '2023-04-10 15:26:36', '系统监控目录');
INSERT INTO "public"."sys_menu" VALUES (3, '系统工具', 0, 5, 'tool', NULL, '', '1', '0', 'M', '0', '0', '', 'tool', 'admin', '2022-12-21 20:22:45', 'admin', '2023-04-10 15:26:51', '系统工具目录');
INSERT INTO "public"."sys_menu" VALUES (100, '用户管理', 1, 1, 'user', 'system/user/index', '', '1', '0', 'C', '0', '0', 'system:user:list', 'user', 'admin', '2022-12-21 20:22:45', '', NULL, '用户管理菜单');
INSERT INTO "public"."sys_menu" VALUES (101, '角色管理', 1, 2, 'role', 'system/role/index', '', '1', '0', 'C', '0', '0', 'system:role:list', 'peoples', 'admin', '2022-12-21 20:22:45', '', NULL, '角色管理菜单');
INSERT INTO "public"."sys_menu" VALUES (102, '菜单管理', 1, 3, 'menu', 'system/menu/index', '', '1', '0', 'C', '0', '0', 'system:menu:list', 'tree-table', 'admin', '2022-12-21 20:22:45', '', NULL, '菜单管理菜单');
INSERT INTO "public"."sys_menu" VALUES (103, '部门管理', 1, 4, 'dept', 'system/dept/index', '', '1', '0', 'C', '0', '0', 'system:dept:list', 'tree', 'admin', '2022-12-21 20:22:45', '', NULL, '部门管理菜单');
INSERT INTO "public"."sys_menu" VALUES (104, '岗位管理', 1, 5, 'post', 'system/post/index', '', '1', '0', 'C', '0', '0', 'system:post:list', 'post', 'admin', '2022-12-21 20:22:45', '', NULL, '岗位管理菜单');
INSERT INTO "public"."sys_menu" VALUES (105, '字典管理', 1, 6, 'dict', 'system/dict/index', '', '1', '0', 'C', '0', '0', 'system:dict:list', 'dict', 'admin', '2022-12-21 20:22:45', '', NULL, '字典管理菜单');
INSERT INTO "public"."sys_menu" VALUES (106, '参数设置', 1, 7, 'config', 'system/config/index', '', '1', '0', 'C', '0', '0', 'system:config:list', 'edit', 'admin', '2022-12-21 20:22:45', '', NULL, '参数设置菜单');
INSERT INTO "public"."sys_menu" VALUES (107, '通知公告', 1, 8, 'notice', 'system/notice/index', '', '1', '0', 'C', '0', '0', 'system:notice:list', 'message', 'admin', '2022-12-21 20:22:45', '', NULL, '通知公告菜单');
INSERT INTO "public"."sys_menu" VALUES (108, '日志管理', 1, 9, 'log', '', '', '1', '0', 'M', '0', '0', '', 'log', 'admin', '2022-12-21 20:22:45', '', NULL, '日志管理菜单');
INSERT INTO "public"."sys_menu" VALUES (109, '在线用户', 2, 1, 'online', 'monitor/online/index', '', '1', '0', 'C', '0', '0', 'monitor:online:list', 'online', 'admin', '2022-12-21 20:22:45', '', NULL, '在线用户菜单');
INSERT INTO "public"."sys_menu" VALUES (114, '表单构建', 3, 1, 'build', 'tool/build/index', '', '1', '0', 'C', '0', '0', 'tool:build:list', 'build', 'admin', '2022-12-21 20:22:45', '', NULL, '表单构建菜单');
INSERT INTO "public"."sys_menu" VALUES (115, '代码生成', 3, 2, 'gen', 'tool/gen/index', '', '1', '0', 'C', '0', '0', 'tool:gen:list', 'code', 'admin', '2022-12-21 20:22:45', '', NULL, '代码生成菜单');
INSERT INTO "public"."sys_menu" VALUES (116, '系统接口', 3, 3, 'swagger', 'tool/swagger/index', '', '1', '0', 'C', '0', '0', 'tool:swagger:list', 'swagger', 'admin', '2022-12-21 20:22:45', '', NULL, '系统接口菜单');
INSERT INTO "public"."sys_menu" VALUES (500, '操作日志', 108, 1, 'operlog', 'monitor/operlog/index', '', '1', '0', 'C', '0', '0', 'monitor:operlog:list', 'form', 'admin', '2022-12-21 20:22:45', '', NULL, '操作日志菜单');
INSERT INTO "public"."sys_menu" VALUES (501, '登录日志', 108, 2, 'logininfor', 'monitor/logininfor/index', '', '1', '0', 'C', '0', '0', 'monitor:logininfor:list', 'logininfor', 'admin', '2022-12-21 20:22:45', '', NULL, '登录日志菜单');
INSERT INTO "public"."sys_menu" VALUES (1001, '用户查询', 100, 1, '', '', '', '1', '0', 'F', '0', '0', 'system:user:query', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1002, '用户新增', 100, 2, '', '', '', '1', '0', 'F', '0', '0', 'system:user:add', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1003, '用户修改', 100, 3, '', '', '', '1', '0', 'F', '0', '0', 'system:user:edit', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1004, '用户删除', 100, 4, '', '', '', '1', '0', 'F', '0', '0', 'system:user:remove', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1005, '用户导出', 100, 5, '', '', '', '1', '0', 'F', '0', '0', 'system:user:export', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1006, '用户导入', 100, 6, '', '', '', '1', '0', 'F', '0', '0', 'system:user:import', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1007, '重置密码', 100, 7, '', '', '', '1', '0', 'F', '0', '0', 'system:user:resetPwd', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1008, '角色查询', 101, 1, '', '', '', '1', '0', 'F', '0', '0', 'system:role:query', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1009, '角色新增', 101, 2, '', '', '', '1', '0', 'F', '0', '0', 'system:role:add', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1010, '角色修改', 101, 3, '', '', '', '1', '0', 'F', '0', '0', 'system:role:edit', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1011, '角色删除', 101, 4, '', '', '', '1', '0', 'F', '0', '0', 'system:role:remove', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1012, '角色导出', 101, 5, '', '', '', '1', '0', 'F', '0', '0', 'system:role:export', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1013, '菜单查询', 102, 1, '', '', '', '1', '0', 'F', '0', '0', 'system:menu:query', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1014, '菜单新增', 102, 2, '', '', '', '1', '0', 'F', '0', '0', 'system:menu:add', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1015, '菜单修改', 102, 3, '', '', '', '1', '0', 'F', '0', '0', 'system:menu:edit', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1016, '菜单删除', 102, 4, '', '', '', '1', '0', 'F', '0', '0', 'system:menu:remove', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1017, '部门查询', 103, 1, '', '', '', '1', '0', 'F', '0', '0', 'system:dept:query', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1018, '部门新增', 103, 2, '', '', '', '1', '0', 'F', '0', '0', 'system:dept:add', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1019, '部门修改', 103, 3, '', '', '', '1', '0', 'F', '0', '0', 'system:dept:edit', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1020, '部门删除', 103, 4, '', '', '', '1', '0', 'F', '0', '0', 'system:dept:remove', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1021, '岗位查询', 104, 1, '', '', '', '1', '0', 'F', '0', '0', 'system:post:query', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1022, '岗位新增', 104, 2, '', '', '', '1', '0', 'F', '0', '0', 'system:post:add', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1023, '岗位修改', 104, 3, '', '', '', '1', '0', 'F', '0', '0', 'system:post:edit', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1024, '岗位删除', 104, 4, '', '', '', '1', '0', 'F', '0', '0', 'system:post:remove', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1025, '岗位导出', 104, 5, '', '', '', '1', '0', 'F', '0', '0', 'system:post:export', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1026, '字典查询', 105, 1, '#', '', '', '1', '0', 'F', '0', '0', 'system:dict:query', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1027, '字典新增', 105, 2, '#', '', '', '1', '0', 'F', '0', '0', 'system:dict:add', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1028, '字典修改', 105, 3, '#', '', '', '1', '0', 'F', '0', '0', 'system:dict:edit', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1029, '字典删除', 105, 4, '#', '', '', '1', '0', 'F', '0', '0', 'system:dict:remove', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1030, '字典导出', 105, 5, '#', '', '', '1', '0', 'F', '0', '0', 'system:dict:export', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1031, '参数查询', 106, 1, '#', '', '', '1', '0', 'F', '0', '0', 'system:config:query', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1032, '参数新增', 106, 2, '#', '', '', '1', '0', 'F', '0', '0', 'system:config:add', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1033, '参数修改', 106, 3, '#', '', '', '1', '0', 'F', '0', '0', 'system:config:edit', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (112, '服务器监控', 2, 4, 'server', 'monitor/server/index', '', '1', '0', 'C', '0', '0', 'monitor:server:list', 'server', 'admin', '2022-12-21 20:22:45', 'admin', '2024-10-16 10:24:21.678', '服务监控菜单');
INSERT INTO "public"."sys_menu" VALUES (111, '数据监控', 2, 3, 'druid', 'monitor/druid/index', '', '1', '0', 'C', '1', '1', 'monitor:druid:list', 'druid', 'admin', '2022-12-21 20:22:45', 'admin', '2024-10-16 10:23:47.477', '数据监控菜单');
INSERT INTO "public"."sys_menu" VALUES (113, '缓存监控', 2, 5, 'cache', 'monitor/cache/index', '', '1', '0', 'C', '1', '1', 'monitor:cache:list', 'redis', 'admin', '2022-12-21 20:22:45', 'admin', '2024-10-16 10:23:43.094', '缓存监控菜单');
INSERT INTO "public"."sys_menu" VALUES (1034, '参数删除', 106, 4, '#', '', '', '1', '0', 'F', '0', '0', 'system:config:remove', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1035, '参数导出', 106, 5, '#', '', '', '1', '0', 'F', '0', '0', 'system:config:export', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1036, '公告查询', 107, 1, '#', '', '', '1', '0', 'F', '0', '0', 'system:notice:query', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1037, '公告新增', 107, 2, '#', '', '', '1', '0', 'F', '0', '0', 'system:notice:add', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1038, '公告修改', 107, 3, '#', '', '', '1', '0', 'F', '0', '0', 'system:notice:edit', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1039, '公告删除', 107, 4, '#', '', '', '1', '0', 'F', '0', '0', 'system:notice:remove', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1040, '操作查询', 500, 1, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:operlog:query', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1041, '操作删除', 500, 2, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:operlog:remove', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1042, '日志导出', 500, 4, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:operlog:export', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1043, '登录查询', 501, 1, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:logininfor:query', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1044, '登录删除', 501, 2, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:logininfor:remove', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1045, '日志导出', 501, 3, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:logininfor:export', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1046, '在线查询', 109, 1, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:online:query', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1047, '批量强退', 109, 2, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:online:batchLogout', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1048, '单条强退', 109, 3, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:online:forceLogout', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1049, '任务查询', 110, 1, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:job:query', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1050, '任务新增', 110, 2, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:job:add', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1051, '任务修改', 110, 3, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:job:edit', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1052, '任务删除', 110, 4, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:job:remove', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1053, '状态修改', 110, 5, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:job:changeStatus', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1054, '任务导出', 110, 7, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:job:export', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1055, '生成查询', 115, 1, '#', '', '', '1', '0', 'F', '0', '0', 'tool:gen:query', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1056, '生成修改', 115, 2, '#', '', '', '1', '0', 'F', '0', '0', 'tool:gen:edit', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1057, '生成删除', 115, 3, '#', '', '', '1', '0', 'F', '0', '0', 'tool:gen:remove', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1058, '导入代码', 115, 2, '#', '', '', '1', '0', 'F', '0', '0', 'tool:gen:import', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1059, '预览代码', 115, 4, '#', '', '', '1', '0', 'F', '0', '0', 'tool:gen:preview', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1060, '生成代码', 115, 5, '#', '', '', '1', '0', 'F', '0', '0', 'tool:gen:code', '#', 'admin', '2022-12-21 20:22:45', '', NULL, '');
INSERT INTO "public"."sys_menu" VALUES (110, '定时任务1', 2, 2, 'job', 'monitor/job/index', '', '1', '0', 'C', '1', '1', 'monitor:job:list', 'job', 'admin', '2022-12-21 20:22:45', 'admin', '2024-10-16 10:23:51.045', '定时任务菜单');
INSERT INTO "public"."sys_menu" VALUES (4, '商城', 0, 6, 'mall', NULL, '', '1', '0', 'M', '0', '0', '', 'tool', 'admin', '2022-12-21 20:22:45', 'admin', '2023-04-10 15:26:51', '商城目录');
INSERT INTO "public"."sys_menu" VALUES (1062, '订单管理', 4, 1, 'order', 'mall/order/index', '', '1', '0', 'C', '0', '0', 'mall:order:list', 'tool', 'admin', '2022-12-21 20:22:45', 'admin', '2023-04-10 15:26:51', '订单管理');
INSERT INTO "public"."sys_menu" VALUES (1064, '订单管理查询', 1062, 1, '#', '', '', '1', '0', 'F', '0', '0', 'mall:order:query', '#', 'admin', '2022-12-21 20:22:45', 'admin', '2023-04-10 15:26:51', '订单管理查询');
INSERT INTO "public"."sys_menu" VALUES (1065, '订单管理新增', 1062, 2, '#', '', '', '1', '0', 'F', '0', '0', 'mall:order:add', '#', 'admin', '2022-12-21 20:22:45', 'admin', '2023-04-10 15:26:51', '订单管理新增');
INSERT INTO "public"."sys_menu" VALUES (1066, '订单管理修改', 1062, 3, '#', '', '', '1', '0', 'F', '0', '0', 'mall:order:edit', '#', 'admin', '2022-12-21 20:22:45', 'admin', '2023-04-10 15:26:51', '订单管理修改');
INSERT INTO "public"."sys_menu" VALUES (1067, '订单管理删除', 1062, 4, '#', '', '', '1', '0', 'F', '0', '0', 'mall:order:remove', '#', 'admin', '2022-12-21 20:22:45', 'admin', '2023-04-10 15:26:51', '订单管理删除');
INSERT INTO "public"."sys_menu" VALUES (1068, '订单管理删除', 1062, 5, '#', '', '', '1', '0', 'F', '0', '0', 'mall:order:export', '#', 'admin', '2022-12-21 20:22:45', 'admin', '2023-04-10 15:26:51', '订单管理删除');
INSERT INTO "public"."sys_menu" VALUES (2025111100002, '上传文件演示', 2, 6, 'system_uploadfile', 'system/uploadfile/index', NULL, '1', '1', 'C', '0', '0', 'system:uploadfile:index', '404', NULL, '2025-11-11 03:37:12.741', 'admin', '2025-11-17 09:34:12.547', NULL);
INSERT INTO "public"."sys_menu" VALUES (2025111700002, 'ws演示', 2, 7, 'system_websocket', 'system/websocket/index', NULL, '1', '0', 'C', '0', '0', 'system:websocket:index', '404', NULL, '2025-11-17 09:35:42.864', NULL, NULL, NULL);

-- ----------------------------
-- Table structure for sys_notice
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_notice";
CREATE TABLE "public"."sys_notice" (
  "notice_id" varchar(32) COLLATE "pg_catalog"."default" NOT NULL,
  "notice_title" varchar(50) COLLATE "pg_catalog"."default" NOT NULL,
  "notice_type" varchar(1) COLLATE "pg_catalog"."default" NOT NULL,
  "notice_content" text COLLATE "pg_catalog"."default",
  "status" varchar(1) COLLATE "pg_catalog"."default",
  "create_by" varchar(64) COLLATE "pg_catalog"."default",
  "create_time" timestamp(6),
  "update_by" varchar(64) COLLATE "pg_catalog"."default",
  "update_time" timestamp(6),
  "remark" varchar(255) COLLATE "pg_catalog"."default"
)
;
COMMENT ON COLUMN "public"."sys_notice"."notice_id" IS '公告ID';
COMMENT ON COLUMN "public"."sys_notice"."notice_title" IS '公告标题';
COMMENT ON COLUMN "public"."sys_notice"."notice_type" IS '公告类型（1通知 2公告）';
COMMENT ON COLUMN "public"."sys_notice"."notice_content" IS '公告内容';
COMMENT ON COLUMN "public"."sys_notice"."status" IS '公告状态（0正常 1关闭）';
COMMENT ON COLUMN "public"."sys_notice"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_notice"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_notice"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_notice"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_notice"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_notice" IS '通知公告表';

-- ----------------------------
-- Records of sys_notice
-- ----------------------------

-- ----------------------------
-- Table structure for sys_oper_log
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_oper_log";
CREATE TABLE "public"."sys_oper_log" (
  "oper_id" int8 NOT NULL,
  "title" varchar(50) COLLATE "pg_catalog"."default",
  "business_type" int4,
  "method" varchar(100) COLLATE "pg_catalog"."default",
  "request_method" varchar(10) COLLATE "pg_catalog"."default",
  "operator_type" int4,
  "oper_name" varchar(50) COLLATE "pg_catalog"."default",
  "dept_name" varchar(50) COLLATE "pg_catalog"."default",
  "oper_url" varchar(255) COLLATE "pg_catalog"."default",
  "oper_ip" varchar(128) COLLATE "pg_catalog"."default",
  "oper_location" varchar(255) COLLATE "pg_catalog"."default",
  "oper_param" varchar(2000) COLLATE "pg_catalog"."default",
  "json_result" varchar(2000) COLLATE "pg_catalog"."default",
  "status" int4,
  "error_msg" varchar(2000) COLLATE "pg_catalog"."default",
  "oper_time" timestamp(6)
)
;
COMMENT ON COLUMN "public"."sys_oper_log"."oper_id" IS '日志主键';
COMMENT ON COLUMN "public"."sys_oper_log"."title" IS '模块标题';
COMMENT ON COLUMN "public"."sys_oper_log"."business_type" IS '业务类型（0其它 1新增 2修改 3删除）';
COMMENT ON COLUMN "public"."sys_oper_log"."method" IS '方法名称';
COMMENT ON COLUMN "public"."sys_oper_log"."request_method" IS '请求方式';
COMMENT ON COLUMN "public"."sys_oper_log"."operator_type" IS '操作类别（0其它 1后台用户 2手机端用户）';
COMMENT ON COLUMN "public"."sys_oper_log"."oper_name" IS '操作人员';
COMMENT ON COLUMN "public"."sys_oper_log"."dept_name" IS '部门名称';
COMMENT ON COLUMN "public"."sys_oper_log"."oper_url" IS '请求URL';
COMMENT ON COLUMN "public"."sys_oper_log"."oper_ip" IS '主机地址';
COMMENT ON COLUMN "public"."sys_oper_log"."oper_location" IS '操作地点';
COMMENT ON COLUMN "public"."sys_oper_log"."oper_param" IS '请求参数';
COMMENT ON COLUMN "public"."sys_oper_log"."json_result" IS '返回参数';
COMMENT ON COLUMN "public"."sys_oper_log"."status" IS '操作状态（0正常 1异常）';
COMMENT ON COLUMN "public"."sys_oper_log"."error_msg" IS '错误消息';
COMMENT ON COLUMN "public"."sys_oper_log"."oper_time" IS '操作时间';
COMMENT ON TABLE "public"."sys_oper_log" IS '操作日志记录';

-- ----------------------------
-- Records of sys_oper_log
-- ----------------------------

-- ----------------------------
-- Table structure for sys_post
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_post";
CREATE TABLE "public"."sys_post" (
  "post_id" varchar(32) COLLATE "pg_catalog"."default" NOT NULL,
  "post_code" varchar(64) COLLATE "pg_catalog"."default" NOT NULL,
  "post_name" varchar(50) COLLATE "pg_catalog"."default" NOT NULL,
  "post_sort" int4 NOT NULL,
  "status" varchar(1) COLLATE "pg_catalog"."default" NOT NULL,
  "create_by" varchar(64) COLLATE "pg_catalog"."default",
  "create_time" timestamp(6),
  "update_by" varchar(64) COLLATE "pg_catalog"."default",
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default"
)
;
COMMENT ON COLUMN "public"."sys_post"."post_id" IS '岗位ID';
COMMENT ON COLUMN "public"."sys_post"."post_code" IS '岗位编码';
COMMENT ON COLUMN "public"."sys_post"."post_name" IS '岗位名称';
COMMENT ON COLUMN "public"."sys_post"."post_sort" IS '显示顺序';
COMMENT ON COLUMN "public"."sys_post"."status" IS '状态（0正常 1停用）';
COMMENT ON COLUMN "public"."sys_post"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_post"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_post"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_post"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_post"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_post" IS '岗位信息表';

-- ----------------------------
-- Records of sys_post
-- ----------------------------
INSERT INTO "public"."sys_post" VALUES ('63f4262b004ea4c100e10b0e', 'gw2', '岗位2', 2, '1', 'admin', '2023-02-21 10:02:19', 'admin', '2023-03-22 10:35:26', NULL);
INSERT INTO "public"."sys_post" VALUES ('63f42611004ea4c100e10b0d', 'gw1', '岗位11', 1, '0', 'admin', '2023-02-21 10:01:53', 'admin', '2024-10-16 07:00:16.258', NULL);
INSERT INTO "public"."sys_post" VALUES ('67c7f07e00a5edf100000000', '44', '44', 0, '0', 'admin', '2025-03-05 06:34:38.747', NULL, NULL, NULL);

-- ----------------------------
-- Table structure for sys_role
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_role";
CREATE TABLE "public"."sys_role" (
  "role_id" varchar(32) COLLATE "pg_catalog"."default" NOT NULL,
  "role_name" varchar(30) COLLATE "pg_catalog"."default" NOT NULL,
  "role_key" varchar(100) COLLATE "pg_catalog"."default" NOT NULL,
  "role_sort" int4 NOT NULL,
  "data_scope" varchar(1) COLLATE "pg_catalog"."default",
  "menu_check_strictly" varchar(1) COLLATE "pg_catalog"."default",
  "dept_check_strictly" varchar(1) COLLATE "pg_catalog"."default",
  "status" varchar(1) COLLATE "pg_catalog"."default" NOT NULL,
  "del_flag" varchar(1) COLLATE "pg_catalog"."default",
  "create_by" varchar(64) COLLATE "pg_catalog"."default",
  "create_time" timestamp(6),
  "update_by" varchar(64) COLLATE "pg_catalog"."default",
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default"
)
;
COMMENT ON COLUMN "public"."sys_role"."role_id" IS '角色ID';
COMMENT ON COLUMN "public"."sys_role"."role_name" IS '角色名称';
COMMENT ON COLUMN "public"."sys_role"."role_key" IS '角色权限字符串';
COMMENT ON COLUMN "public"."sys_role"."role_sort" IS '显示顺序';
COMMENT ON COLUMN "public"."sys_role"."data_scope" IS '数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）';
COMMENT ON COLUMN "public"."sys_role"."menu_check_strictly" IS '菜单树选择项是否关联显示';
COMMENT ON COLUMN "public"."sys_role"."dept_check_strictly" IS '部门树选择项是否关联显示';
COMMENT ON COLUMN "public"."sys_role"."status" IS '角色状态（0正常 1停用）';
COMMENT ON COLUMN "public"."sys_role"."del_flag" IS '删除标志（0代表存在 2代表删除）';
COMMENT ON COLUMN "public"."sys_role"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_role"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_role"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_role"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_role"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_role" IS '角色信息表';

-- ----------------------------
-- Records of sys_role
-- ----------------------------
INSERT INTO "public"."sys_role" VALUES ('1', '超级管理员', 'admin', 1, '1', '1', '1', '0', '0', 'admin', '2021-08-16 10:39:40', '', NULL, '超级管理员');
INSERT INTO "public"."sys_role" VALUES ('63d8c82a00e2c21200979b90', '普通用户', 'general_user', 1, NULL, '1', '1', '0', '0', 'admin', '2023-01-31 15:50:02', 'admin', '2023-02-15 14:44:49', NULL);

-- ----------------------------
-- Table structure for sys_role_dept
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_role_dept";
CREATE TABLE "public"."sys_role_dept" (
  "role_id" int8 NOT NULL,
  "dept_id" int8 NOT NULL
)
;
COMMENT ON COLUMN "public"."sys_role_dept"."role_id" IS '角色ID';
COMMENT ON COLUMN "public"."sys_role_dept"."dept_id" IS '部门ID';
COMMENT ON TABLE "public"."sys_role_dept" IS '角色和部门关联表';

-- ----------------------------
-- Records of sys_role_dept
-- ----------------------------

-- ----------------------------
-- Table structure for sys_role_menu
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_role_menu";
CREATE TABLE "public"."sys_role_menu" (
  "role_id" varchar(32) COLLATE "pg_catalog"."default" NOT NULL,
  "menu_id" int8 NOT NULL
)
;
COMMENT ON COLUMN "public"."sys_role_menu"."role_id" IS '角色ID';
COMMENT ON COLUMN "public"."sys_role_menu"."menu_id" IS '菜单ID';
COMMENT ON TABLE "public"."sys_role_menu" IS '角色和菜单关联表';

-- ----------------------------
-- Records of sys_role_menu
-- ----------------------------

-- ----------------------------
-- Table structure for sys_trash
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_trash";
CREATE TABLE "public"."sys_trash" (
  "id" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "table_name" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "data" text COLLATE "pg_catalog"."default",
  "create_time" timestamp(6)
)
;

-- ----------------------------
-- Records of sys_trash
-- ----------------------------
INSERT INTO "public"."sys_trash" VALUES ('67c9257d00e4c66600000000', 'oms_order', '{"id":2025030600004,"order_sn":"5","member_username":"5","total_amount":null,"pay_amount":null,"freight_amount":null,"promotion_amount":null,"integration_amount":null,"coupon_amount":null,"discount_amount":null,"pay_type":null,"source_type":null,"status":null,"order_type":null,"delivery_company":null,"delivery_sn":null,"auto_confirm_day":null,"integration":null,"growth":null,"promotion_info":null,"bill_type":null,"bill_header":null,"bill_content":null,"bill_receiver_phone":null,"bill_receiver_email":null,"receiver_name":"4","receiver_phone":"4","receiver_post_code":null,"receiver_province":null,"receiver_city":null,"receiver_region":null,"receiver_detail_address":null,"note":null,"confirm_status":null,"use_integration":null,"payment_time":null,"delivery_time":null,"receive_time":null,"comment_time":null,"del_flag":null,"create_by":"admin","create_time":"2025-03-06T02:31:55.863Z","update_by":"admin","update_time":"2025-03-06T02:32:08.014Z"}', '2025-03-06 04:33:01.721');
INSERT INTO "public"."sys_trash" VALUES ('67c9258000e4c66600000001', 'oms_order', '{"id":2025030600003,"order_sn":"3","member_username":"3","total_amount":null,"pay_amount":null,"freight_amount":null,"promotion_amount":null,"integration_amount":null,"coupon_amount":null,"discount_amount":null,"pay_type":null,"source_type":null,"status":null,"order_type":null,"delivery_company":null,"delivery_sn":null,"auto_confirm_day":null,"integration":null,"growth":null,"promotion_info":null,"bill_type":null,"bill_header":null,"bill_content":null,"bill_receiver_phone":null,"bill_receiver_email":null,"receiver_name":"3","receiver_phone":"3","receiver_post_code":null,"receiver_province":null,"receiver_city":null,"receiver_region":null,"receiver_detail_address":null,"note":null,"confirm_status":null,"use_integration":null,"payment_time":null,"delivery_time":null,"receive_time":null,"comment_time":null,"del_flag":null,"create_by":"admin","create_time":"2025-03-06T02:31:37.229Z","update_by":null,"update_time":null}', '2025-03-06 04:33:04.933');
INSERT INTO "public"."sys_trash" VALUES ('67c9258200e4c66600000002', 'oms_order', '{"id":2025030600002,"order_sn":"2","member_username":"2","total_amount":null,"pay_amount":null,"freight_amount":null,"promotion_amount":null,"integration_amount":null,"coupon_amount":null,"discount_amount":null,"pay_type":null,"source_type":null,"status":null,"order_type":null,"delivery_company":null,"delivery_sn":null,"auto_confirm_day":null,"integration":null,"growth":null,"promotion_info":null,"bill_type":null,"bill_header":null,"bill_content":null,"bill_receiver_phone":null,"bill_receiver_email":null,"receiver_name":"2","receiver_phone":"2","receiver_post_code":null,"receiver_province":null,"receiver_city":null,"receiver_region":null,"receiver_detail_address":null,"note":null,"confirm_status":null,"use_integration":null,"payment_time":null,"delivery_time":null,"receive_time":null,"comment_time":null,"del_flag":null,"create_by":"admin","create_time":"2025-03-06T02:20:43.304Z","update_by":null,"update_time":null}', '2025-03-06 04:33:06.649');
INSERT INTO "public"."sys_trash" VALUES ('67c9258400e4c66600000003', 'oms_order', '{"id":2025030600001,"order_sn":"1","member_username":null,"total_amount":null,"pay_amount":null,"freight_amount":null,"promotion_amount":null,"integration_amount":null,"coupon_amount":null,"discount_amount":null,"pay_type":null,"source_type":null,"status":null,"order_type":null,"delivery_company":null,"delivery_sn":null,"auto_confirm_day":null,"integration":null,"growth":null,"promotion_info":null,"bill_type":null,"bill_header":null,"bill_content":null,"bill_receiver_phone":null,"bill_receiver_email":null,"receiver_name":"1","receiver_phone":"1","receiver_post_code":null,"receiver_province":null,"receiver_city":null,"receiver_region":null,"receiver_detail_address":null,"note":null,"confirm_status":null,"use_integration":null,"payment_time":null,"delivery_time":null,"receive_time":null,"comment_time":null,"del_flag":null,"create_by":"admin","create_time":"2025-03-06T01:49:37.287Z","update_by":null,"update_time":null}', '2025-03-06 04:33:08.566');

-- ----------------------------
-- Table structure for sys_user
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_user";
CREATE TABLE "public"."sys_user" (
  "user_id" varchar(32) COLLATE "pg_catalog"."default" NOT NULL,
  "dept_id" varchar(20) COLLATE "pg_catalog"."default",
  "user_name" varchar(30) COLLATE "pg_catalog"."default" NOT NULL,
  "nick_name" varchar(30) COLLATE "pg_catalog"."default" NOT NULL,
  "user_type" varchar(2) COLLATE "pg_catalog"."default",
  "email" varchar(50) COLLATE "pg_catalog"."default",
  "phonenumber" varchar(11) COLLATE "pg_catalog"."default",
  "sex" varchar(1) COLLATE "pg_catalog"."default",
  "avatar" varchar(100) COLLATE "pg_catalog"."default",
  "password" varchar(100) COLLATE "pg_catalog"."default",
  "status" varchar(1) COLLATE "pg_catalog"."default",
  "del_flag" varchar(1) COLLATE "pg_catalog"."default",
  "login_ip" varchar(128) COLLATE "pg_catalog"."default",
  "login_date" timestamp(6),
  "create_by" varchar(64) COLLATE "pg_catalog"."default",
  "create_time" timestamp(6),
  "update_by" varchar(64) COLLATE "pg_catalog"."default",
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default"
)
;
COMMENT ON COLUMN "public"."sys_user"."user_id" IS '用户ID';
COMMENT ON COLUMN "public"."sys_user"."dept_id" IS '部门ID';
COMMENT ON COLUMN "public"."sys_user"."user_name" IS '用户账号';
COMMENT ON COLUMN "public"."sys_user"."nick_name" IS '用户昵称';
COMMENT ON COLUMN "public"."sys_user"."user_type" IS '用户类型（00系统用户）';
COMMENT ON COLUMN "public"."sys_user"."email" IS '用户邮箱';
COMMENT ON COLUMN "public"."sys_user"."phonenumber" IS '手机号码';
COMMENT ON COLUMN "public"."sys_user"."sex" IS '用户性别（0男 1女 2未知）';
COMMENT ON COLUMN "public"."sys_user"."avatar" IS '头像地址';
COMMENT ON COLUMN "public"."sys_user"."password" IS '密码';
COMMENT ON COLUMN "public"."sys_user"."status" IS '帐号状态（0正常 1停用）';
COMMENT ON COLUMN "public"."sys_user"."del_flag" IS '删除标志（0代表存在 2代表删除）';
COMMENT ON COLUMN "public"."sys_user"."login_ip" IS '最后登录IP';
COMMENT ON COLUMN "public"."sys_user"."login_date" IS '最后登录时间';
COMMENT ON COLUMN "public"."sys_user"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_user"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_user"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_user"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_user"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_user" IS '用户信息表';

-- ----------------------------
-- Records of sys_user
-- ----------------------------
INSERT INTO "public"."sys_user" VALUES ('1', '100', 'admin', '管理员', '00', '34234534@qq.com', '13414111112', '0', NULL, 'b227bff0d28823d4599a39a5b55725b0811c9c13184087e9a122eb572e6ff139', '0', '0', '120.43.159.4', '2022-12-12 23:12:37', 'admin', '2021-08-16 10:39:40', '', '2024-10-16 09:25:42.117', '管理员');
INSERT INTO "public"."sys_user" VALUES ('6710768b00a2a5dc00000000', '102', 'user', '用户1', NULL, NULL, '18088138716', '0', NULL, 'E1AA6533B72C63A26EBED8BBD778F5F50C88B83DB6977546F00D7A1097689ABB', '0', '0', NULL, NULL, 'admin', '2024-10-17 02:29:31.273', NULL, NULL, NULL);
INSERT INTO "public"."sys_user" VALUES ('68905d49004eacc900000000', '108', 'ddd', 'ddd', NULL, NULL, NULL, NULL, NULL, 'D7190EB194FF9494625514B6D178C87F99C5973E28C398969D2233F2960A573E', '0', '0', NULL, NULL, 'admin', '2025-08-04 07:12:09.253', NULL, NULL, NULL);

-- ----------------------------
-- Table structure for sys_user_post
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_user_post";
CREATE TABLE "public"."sys_user_post" (
  "user_id" varchar(64) COLLATE "pg_catalog"."default" NOT NULL,
  "post_id" varchar(64) COLLATE "pg_catalog"."default" NOT NULL
)
;
COMMENT ON COLUMN "public"."sys_user_post"."user_id" IS '用户ID';
COMMENT ON COLUMN "public"."sys_user_post"."post_id" IS '岗位ID';
COMMENT ON TABLE "public"."sys_user_post" IS '用户与岗位关联表';

-- ----------------------------
-- Records of sys_user_post
-- ----------------------------
INSERT INTO "public"."sys_user_post" VALUES ('1', '5');

-- ----------------------------
-- Table structure for sys_user_role
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_user_role";
CREATE TABLE "public"."sys_user_role" (
  "user_id" varchar(32) COLLATE "pg_catalog"."default" NOT NULL,
  "role_id" varchar(32) COLLATE "pg_catalog"."default" NOT NULL
)
;
COMMENT ON COLUMN "public"."sys_user_role"."user_id" IS '用户ID';
COMMENT ON COLUMN "public"."sys_user_role"."role_id" IS '角色ID';
COMMENT ON TABLE "public"."sys_user_role" IS '用户和角色关联表';

-- ----------------------------
-- Records of sys_user_role
-- ----------------------------
INSERT INTO "public"."sys_user_role" VALUES ('1', '1');
INSERT INTO "public"."sys_user_role" VALUES ('6710768b00a2a5dc00000000', '63d8c82a00e2c21200979b90');

-- ----------------------------
-- Primary Key structure for table oms_order
-- ----------------------------
ALTER TABLE "public"."oms_order" ADD CONSTRAINT "oms_order_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Primary Key structure for table oms_regorder
-- ----------------------------
ALTER TABLE "public"."oms_regorder" ADD CONSTRAINT "oms_regorder_pkey" PRIMARY KEY ("regid");

-- ----------------------------
-- Primary Key structure for table sys_config
-- ----------------------------
ALTER TABLE "public"."sys_config" ADD CONSTRAINT "sys_config_pkey" PRIMARY KEY ("config_id");

-- ----------------------------
-- Primary Key structure for table sys_dept
-- ----------------------------
ALTER TABLE "public"."sys_dept" ADD CONSTRAINT "sys_dept_pkey" PRIMARY KEY ("dept_id");

-- ----------------------------
-- Primary Key structure for table sys_dict_data
-- ----------------------------
ALTER TABLE "public"."sys_dict_data" ADD CONSTRAINT "sys_dict_data_pkey" PRIMARY KEY ("dict_code");

-- ----------------------------
-- Indexes structure for table sys_dict_type
-- ----------------------------
CREATE INDEX "dict_type" ON "public"."sys_dict_type" USING btree (
  "dict_type" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
);

-- ----------------------------
-- Primary Key structure for table sys_dict_type
-- ----------------------------
ALTER TABLE "public"."sys_dict_type" ADD CONSTRAINT "sys_dict_type_pkey" PRIMARY KEY ("dict_id");

-- ----------------------------
-- Primary Key structure for table sys_job
-- ----------------------------
ALTER TABLE "public"."sys_job" ADD CONSTRAINT "sys_job_pkey" PRIMARY KEY ("job_id", "job_name", "job_group");

-- ----------------------------
-- Primary Key structure for table sys_job_log
-- ----------------------------
ALTER TABLE "public"."sys_job_log" ADD CONSTRAINT "sys_job_log_pkey" PRIMARY KEY ("job_log_id");

-- ----------------------------
-- Primary Key structure for table sys_logininfor
-- ----------------------------
ALTER TABLE "public"."sys_logininfor" ADD CONSTRAINT "sys_logininfor_pkey" PRIMARY KEY ("info_id");

-- ----------------------------
-- Primary Key structure for table sys_menu
-- ----------------------------
ALTER TABLE "public"."sys_menu" ADD CONSTRAINT "sys_menu_pkey" PRIMARY KEY ("menu_id");

-- ----------------------------
-- Primary Key structure for table sys_notice
-- ----------------------------
ALTER TABLE "public"."sys_notice" ADD CONSTRAINT "sys_notice_pkey" PRIMARY KEY ("notice_id");

-- ----------------------------
-- Primary Key structure for table sys_oper_log
-- ----------------------------
ALTER TABLE "public"."sys_oper_log" ADD CONSTRAINT "sys_oper_log_pkey" PRIMARY KEY ("oper_id");

-- ----------------------------
-- Primary Key structure for table sys_post
-- ----------------------------
ALTER TABLE "public"."sys_post" ADD CONSTRAINT "sys_post_pkey" PRIMARY KEY ("post_id");

-- ----------------------------
-- Primary Key structure for table sys_role
-- ----------------------------
ALTER TABLE "public"."sys_role" ADD CONSTRAINT "sys_role_pkey" PRIMARY KEY ("role_id");

-- ----------------------------
-- Primary Key structure for table sys_role_dept
-- ----------------------------
ALTER TABLE "public"."sys_role_dept" ADD CONSTRAINT "sys_role_dept_pkey" PRIMARY KEY ("role_id", "dept_id");

-- ----------------------------
-- Primary Key structure for table sys_role_menu
-- ----------------------------
ALTER TABLE "public"."sys_role_menu" ADD CONSTRAINT "sys_role_menu_pkey" PRIMARY KEY ("role_id", "menu_id");

-- ----------------------------
-- Primary Key structure for table sys_user
-- ----------------------------
ALTER TABLE "public"."sys_user" ADD CONSTRAINT "sys_user_pkey" PRIMARY KEY ("user_id");

-- ----------------------------
-- Primary Key structure for table sys_user_post
-- ----------------------------
ALTER TABLE "public"."sys_user_post" ADD CONSTRAINT "sys_user_post_pkey" PRIMARY KEY ("user_id", "post_id");

-- ----------------------------
-- Primary Key structure for table sys_user_role
-- ----------------------------
ALTER TABLE "public"."sys_user_role" ADD CONSTRAINT "sys_user_role_pkey" PRIMARY KEY ("user_id", "role_id");
