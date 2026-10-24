/*
 Navicat Premium Data Transfer

 Source Server         : post_localhost
 Source Server Type    : PostgreSQL
 Source Server Version : 160007
 Source Host           : localhost:5432
 Source Catalog        : ruoyirust
 Source Schema         : public

 Target Server Type    : PostgreSQL
 Target Server Version : 160007
 File Encoding         : 65001

 Date: 10/03/2025 16:09:01
*/


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
-- Primary Key structure for table oms_regorder
-- ----------------------------
ALTER TABLE "public"."oms_regorder" ADD CONSTRAINT "oms_regorder_pkey" PRIMARY KEY ("regid");
