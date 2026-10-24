/*
 Navicat Premium Dump SQL

 Source Server         : local-pg
 Source Server Type    : PostgreSQL
 Source Server Version : 160007 (160007)
 Source Host           : localhost:5434
 Source Catalog        : ruoyirust
 Source Schema         : public

 Target Server Type    : PostgreSQL
 Target Server Version : 160007 (160007)
 File Encoding         : 65001

 Date: 08/10/2026 20:59:33
*/


-- ----------------------------
-- Type structure for ghstore
-- ----------------------------
DROP TYPE IF EXISTS "public"."ghstore";
CREATE TYPE "public"."ghstore" (
  INPUT = "public"."ghstore_in",
  OUTPUT = "public"."ghstore_out",
  INTERNALLENGTH = VARIABLE,
  CATEGORY = U,
  DELIMITER = ','
);

-- ----------------------------
-- Type structure for hstore
-- ----------------------------
DROP TYPE IF EXISTS "public"."hstore";
CREATE TYPE "public"."hstore" (
  INPUT = "public"."hstore_in",
  OUTPUT = "public"."hstore_out",
  RECEIVE = "public"."hstore_recv",
  SEND = "public"."hstore_send",
  INTERNALLENGTH = VARIABLE,
  STORAGE = extended,
  CATEGORY = U,
  DELIMITER = ','
);

-- ----------------------------
-- Sequence structure for casbin_rule_id_seq
-- ----------------------------
DROP SEQUENCE IF EXISTS "public"."casbin_rule_id_seq";
CREATE SEQUENCE "public"."casbin_rule_id_seq" 
INCREMENT 1
MINVALUE  1
MAXVALUE 9223372036854775807
START 1
CACHE 1;

-- ----------------------------
-- Sequence structure for sys_job_job_id_seq
-- ----------------------------
DROP SEQUENCE IF EXISTS "public"."sys_job_job_id_seq";
CREATE SEQUENCE "public"."sys_job_job_id_seq" 
INCREMENT 1
MINVALUE  1
MAXVALUE 9223372036854775807
START 1
CACHE 1;

-- ----------------------------
-- Sequence structure for sys_job_log_job_log_id_seq
-- ----------------------------
DROP SEQUENCE IF EXISTS "public"."sys_job_log_job_log_id_seq";
CREATE SEQUENCE "public"."sys_job_log_job_log_id_seq" 
INCREMENT 1
MINVALUE  1
MAXVALUE 9223372036854775807
START 1
CACHE 1;

-- ----------------------------
-- Table structure for casbin_rule
-- ----------------------------
DROP TABLE IF EXISTS "public"."casbin_rule";
CREATE TABLE "public"."casbin_rule" (
  "id" int8 NOT NULL DEFAULT nextval('casbin_rule_id_seq'::regclass),
  "ptype" varchar(18) COLLATE "pg_catalog"."default" NOT NULL,
  "v0" varchar(125) COLLATE "pg_catalog"."default" NOT NULL,
  "v1" varchar(125) COLLATE "pg_catalog"."default" NOT NULL,
  "v2" varchar(125) COLLATE "pg_catalog"."default" NOT NULL,
  "v3" varchar(125) COLLATE "pg_catalog"."default" NOT NULL,
  "v4" varchar(125) COLLATE "pg_catalog"."default" NOT NULL,
  "v5" varchar(125) COLLATE "pg_catalog"."default" NOT NULL
)
;

-- ----------------------------
-- Records of casbin_rule
-- ----------------------------

-- ----------------------------
-- Table structure for demo_course
-- ----------------------------
DROP TABLE IF EXISTS "public"."demo_course";
CREATE TABLE "public"."demo_course" (
  "course_id" int8 NOT NULL,
  "course_code" varchar(64) COLLATE "pg_catalog"."default",
  "course_name" varchar(200) COLLATE "pg_catalog"."default",
  "teacher" varchar(50) COLLATE "pg_catalog"."default",
  "credit" float8,
  "open_status" char(1) COLLATE "pg_catalog"."default" DEFAULT '0'::bpchar,
  "start_date" date,
  "remark" varchar(500) COLLATE "pg_catalog"."default",
  "create_by" int8,
  "create_time" timestamp(6),
  "update_by" int8,
  "update_time" timestamp(6)
)
;
COMMENT ON COLUMN "public"."demo_course"."course_id" IS '课程编号';
COMMENT ON COLUMN "public"."demo_course"."course_code" IS '课程编码';
COMMENT ON COLUMN "public"."demo_course"."course_name" IS '课程名称';
COMMENT ON COLUMN "public"."demo_course"."teacher" IS '授课教师';
COMMENT ON COLUMN "public"."demo_course"."credit" IS '学分';
COMMENT ON COLUMN "public"."demo_course"."open_status" IS '开课状态';
COMMENT ON COLUMN "public"."demo_course"."start_date" IS '开课日期';
COMMENT ON COLUMN "public"."demo_course"."remark" IS '备注';
COMMENT ON COLUMN "public"."demo_course"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."demo_course"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."demo_course"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."demo_course"."update_time" IS '更新时间';
COMMENT ON TABLE "public"."demo_course" IS '演示课程表';

-- ----------------------------
-- Records of demo_course
-- ----------------------------
INSERT INTO "public"."demo_course" VALUES (1001, 'GIS-001', '地理信息系统原理', '老杨', 3, '0', '2026-03-01', '演示数据一', NULL, '2026-10-01 06:08:55.355207', NULL, NULL);
INSERT INTO "public"."demo_course" VALUES (1002, 'RS-002', '遥感数字图像处理', '小云', 2.5, '0', '2026-04-15', '演示数据二', NULL, '2026-10-01 06:08:55.355207', NULL, NULL);
INSERT INTO "public"."demo_course" VALUES (32047808266243, 'CS101', 'Python入门冒烟', '老杨', 3.5, '0', '2026-10-01', 'codegen 冒烟测试', 0, '2026-10-01 06:09:33.170729', NULL, NULL);
INSERT INTO "public"."demo_course" VALUES (32047835922435, 'CS101', 'Python入门冒烟', '老杨', 3.5, '0', '2026-10-01', 'codegen 冒烟测试', 0, '2026-10-01 06:13:04.41353', NULL, NULL);
INSERT INTO "public"."demo_course" VALUES (32047881535491, 'CS101', 'Python入门冒烟', '老杨', 3.5, '0', '2026-10-01', 'codegen 冒烟测试', 0, '2026-10-01 06:18:52.182984', NULL, NULL);
INSERT INTO "public"."demo_course" VALUES (32125276536833, NULL, NULL, NULL, NULL, NULL, NULL, NULL, 0, '2026-10-08 02:20:09.860388', NULL, NULL);

-- ----------------------------
-- Table structure for gen_table
-- ----------------------------
DROP TABLE IF EXISTS "public"."gen_table";
CREATE TABLE "public"."gen_table" (
  "table_id" int8 NOT NULL,
  "data_name" varchar(200) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "table_name" varchar(200) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "table_comment" varchar(500) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "sub_table_name" varchar(64) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "sub_table_fk_name" varchar(64) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "class_name" varchar(100) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "tpl_category" varchar(200) COLLATE "pg_catalog"."default" DEFAULT 'crud'::character varying,
  "package_name" varchar(100) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "module_name" varchar(30) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "business_name" varchar(30) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "function_name" varchar(50) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "function_author" varchar(50) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "gen_type" char(1) COLLATE "pg_catalog"."default" NOT NULL DEFAULT '0'::bpchar,
  "gen_path" varchar(200) COLLATE "pg_catalog"."default" DEFAULT '/'::character varying,
  "options" varchar(1000) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "create_dept" int8,
  "create_by" int8,
  "create_time" timestamp(6),
  "update_by" int8,
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying
)
;
COMMENT ON COLUMN "public"."gen_table"."table_id" IS '编号';
COMMENT ON COLUMN "public"."gen_table"."data_name" IS '数据源名称';
COMMENT ON COLUMN "public"."gen_table"."table_name" IS '表名称';
COMMENT ON COLUMN "public"."gen_table"."table_comment" IS '表描述';
COMMENT ON COLUMN "public"."gen_table"."sub_table_name" IS '关联子表的表名';
COMMENT ON COLUMN "public"."gen_table"."sub_table_fk_name" IS '子表关联的外键名';
COMMENT ON COLUMN "public"."gen_table"."class_name" IS '实体类名称';
COMMENT ON COLUMN "public"."gen_table"."tpl_category" IS '使用的模板（CRUD单表操作 TREE树表操作）';
COMMENT ON COLUMN "public"."gen_table"."package_name" IS '生成包路径';
COMMENT ON COLUMN "public"."gen_table"."module_name" IS '生成模块名';
COMMENT ON COLUMN "public"."gen_table"."business_name" IS '生成业务名';
COMMENT ON COLUMN "public"."gen_table"."function_name" IS '生成功能名';
COMMENT ON COLUMN "public"."gen_table"."function_author" IS '生成功能作者';
COMMENT ON COLUMN "public"."gen_table"."gen_type" IS '生成代码方式（0zip压缩包 1自定义路径）';
COMMENT ON COLUMN "public"."gen_table"."gen_path" IS '生成路径（不填默认项目路径）';
COMMENT ON COLUMN "public"."gen_table"."options" IS '其它生成选项';
COMMENT ON COLUMN "public"."gen_table"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."gen_table"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."gen_table"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."gen_table"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."gen_table"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."gen_table"."remark" IS '备注';
COMMENT ON TABLE "public"."gen_table" IS '代码生成业务表';

-- ----------------------------
-- Records of gen_table
-- ----------------------------
INSERT INTO "public"."gen_table" VALUES (32133813649409, 'master', 'demo_course', '演示课程表', NULL, NULL, 'DemoCourse', 'crud', 'backend', 'demo', 'course', '演示课程表', 'ruoyi-rust', '0', '/', '{"parentMenuId":0}', NULL, 1, '2026-10-08 20:25:42.213239', NULL, '2026-10-08 20:37:09.986224', NULL);

-- ----------------------------
-- Table structure for gen_table_column
-- ----------------------------
DROP TABLE IF EXISTS "public"."gen_table_column";
CREATE TABLE "public"."gen_table_column" (
  "column_id" int8 NOT NULL,
  "table_id" int8,
  "column_name" varchar(200) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "column_comment" varchar(500) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "column_type" varchar(100) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "java_type" varchar(500) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "java_field" varchar(200) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "is_pk" char(1) COLLATE "pg_catalog"."default" DEFAULT NULL::bpchar,
  "is_increment" char(1) COLLATE "pg_catalog"."default" DEFAULT NULL::bpchar,
  "is_required" char(1) COLLATE "pg_catalog"."default" DEFAULT NULL::bpchar,
  "is_insert" char(1) COLLATE "pg_catalog"."default" DEFAULT NULL::bpchar,
  "is_edit" char(1) COLLATE "pg_catalog"."default" DEFAULT NULL::bpchar,
  "is_list" char(1) COLLATE "pg_catalog"."default" DEFAULT NULL::bpchar,
  "is_query" char(1) COLLATE "pg_catalog"."default" DEFAULT NULL::bpchar,
  "query_type" varchar(200) COLLATE "pg_catalog"."default" DEFAULT 'EQ'::character varying,
  "html_type" varchar(200) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "dict_type" varchar(200) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "sort" int4,
  "create_dept" int8,
  "create_by" int8,
  "create_time" timestamp(6),
  "update_by" int8,
  "update_time" timestamp(6)
)
;
COMMENT ON COLUMN "public"."gen_table_column"."column_id" IS '编号';
COMMENT ON COLUMN "public"."gen_table_column"."table_id" IS '归属表编号';
COMMENT ON COLUMN "public"."gen_table_column"."column_name" IS '列名称';
COMMENT ON COLUMN "public"."gen_table_column"."column_comment" IS '列描述';
COMMENT ON COLUMN "public"."gen_table_column"."column_type" IS '列类型';
COMMENT ON COLUMN "public"."gen_table_column"."java_type" IS 'JAVA类型';
COMMENT ON COLUMN "public"."gen_table_column"."java_field" IS 'JAVA字段名';
COMMENT ON COLUMN "public"."gen_table_column"."is_pk" IS '是否主键（1是）';
COMMENT ON COLUMN "public"."gen_table_column"."is_increment" IS '是否自增（1是）';
COMMENT ON COLUMN "public"."gen_table_column"."is_required" IS '是否必填（1是）';
COMMENT ON COLUMN "public"."gen_table_column"."is_insert" IS '是否为插入字段（1是）';
COMMENT ON COLUMN "public"."gen_table_column"."is_edit" IS '是否编辑字段（1是）';
COMMENT ON COLUMN "public"."gen_table_column"."is_list" IS '是否列表字段（1是）';
COMMENT ON COLUMN "public"."gen_table_column"."is_query" IS '是否查询字段（1是）';
COMMENT ON COLUMN "public"."gen_table_column"."query_type" IS '查询方式（等于、不等于、大于、小于、范围）';
COMMENT ON COLUMN "public"."gen_table_column"."html_type" IS '显示类型（文本框、文本域、下拉框、复选框、单选框、日期控件）';
COMMENT ON COLUMN "public"."gen_table_column"."dict_type" IS '字典类型';
COMMENT ON COLUMN "public"."gen_table_column"."sort" IS '排序';
COMMENT ON COLUMN "public"."gen_table_column"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."gen_table_column"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."gen_table_column"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."gen_table_column"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."gen_table_column"."update_time" IS '更新时间';
COMMENT ON TABLE "public"."gen_table_column" IS '代码生成业务表字段';

-- ----------------------------
-- Records of gen_table_column
-- ----------------------------
INSERT INTO "public"."gen_table_column" VALUES (32133813649410, 32133813649409, 'course_id', '课程编号', 'int8', NULL, 'courseId', '1', '0', '0', '0', '1', '1', '0', 'EQ', 'input', '', 0, NULL, 1, '2026-10-08 20:25:42.214746', NULL, '2026-10-08 20:37:09.992853');
INSERT INTO "public"."gen_table_column" VALUES (32133813649411, 32133813649409, 'course_code', '课程编码', 'varchar', NULL, 'courseCode', '0', '0', '0', '1', '1', '1', '1', 'LIKE', 'input', '', 1, NULL, 1, '2026-10-08 20:25:42.216076', NULL, '2026-10-08 20:37:10.000341');
INSERT INTO "public"."gen_table_column" VALUES (32133813649412, 32133813649409, 'course_name', '课程名称', 'varchar', NULL, 'courseName', '0', '0', '0', '1', '1', '1', '1', 'LIKE', 'input', '', 2, NULL, 1, '2026-10-08 20:25:42.217405', NULL, '2026-10-08 20:37:10.007335');
INSERT INTO "public"."gen_table_column" VALUES (32133813649413, 32133813649409, 'teacher', '授课教师', 'varchar', NULL, 'teacher', '0', '0', '0', '1', '1', '1', '0', 'LIKE', 'input', '', 3, NULL, 1, '2026-10-08 20:25:42.218613', NULL, '2026-10-08 20:37:10.013183');
INSERT INTO "public"."gen_table_column" VALUES (32133813649414, 32133813649409, 'credit', '学分', 'float8', NULL, 'credit', '0', '0', '0', '1', '1', '1', '0', 'EQ', 'input', '', 4, NULL, 1, '2026-10-08 20:25:42.21981', NULL, '2026-10-08 20:37:10.017794');
INSERT INTO "public"."gen_table_column" VALUES (32133813649415, 32133813649409, 'open_status', '开课状态', 'bpchar', NULL, 'openStatus', '0', '0', '0', '1', '1', '1', '0', 'LIKE', 'radio', 'course_status', 5, NULL, 1, '2026-10-08 20:25:42.221011', NULL, '2026-10-08 20:37:10.020987');
INSERT INTO "public"."gen_table_column" VALUES (32133813649416, 32133813649409, 'start_date', '开课日期', 'date', NULL, 'startDate', '0', '0', '0', '1', '1', '1', '0', 'BETWEEN', 'datetime', '', 6, NULL, 1, '2026-10-08 20:25:42.222192', NULL, '2026-10-08 20:37:10.023808');
INSERT INTO "public"."gen_table_column" VALUES (32133813649417, 32133813649409, 'remark', '备注', 'varchar', NULL, 'remark', '0', '0', '0', '1', '1', '1', '0', 'LIKE', 'textarea', '', 7, NULL, 1, '2026-10-08 20:25:42.223403', NULL, '2026-10-08 20:37:10.026767');
INSERT INTO "public"."gen_table_column" VALUES (32133813649418, 32133813649409, 'create_by', '创建者', 'int8', NULL, 'createBy', '0', '0', '0', '0', '0', '1', '0', 'EQ', 'input', '', 8, NULL, 1, '2026-10-08 20:25:42.224584', NULL, '2026-10-08 20:37:10.029249');
INSERT INTO "public"."gen_table_column" VALUES (32133813649419, 32133813649409, 'create_time', '创建时间', 'timestamp', NULL, 'createTime', '0', '0', '0', '0', '0', '1', '0', 'BETWEEN', 'datetime', '', 9, NULL, 1, '2026-10-08 20:25:42.225828', NULL, '2026-10-08 20:37:10.031645');
INSERT INTO "public"."gen_table_column" VALUES (32133813649420, 32133813649409, 'update_by', '更新者', 'int8', NULL, 'updateBy', '0', '0', '0', '0', '0', '1', '0', 'EQ', 'input', '', 10, NULL, 1, '2026-10-08 20:25:42.227009', NULL, '2026-10-08 20:37:10.033888');
INSERT INTO "public"."gen_table_column" VALUES (32133813649421, 32133813649409, 'update_time', '更新时间', 'timestamp', NULL, 'updateTime', '0', '0', '0', '0', '0', '1', '0', 'BETWEEN', 'datetime', '', 11, NULL, 1, '2026-10-08 20:25:42.228136', NULL, '2026-10-08 20:37:10.035936');

-- ----------------------------
-- Table structure for sys_client
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_client";
CREATE TABLE "public"."sys_client" (
  "id" int8 NOT NULL,
  "client_id" varchar(64) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "client_key" varchar(32) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "client_secret" varchar(255) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "grant_type" varchar(255) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "device_type" varchar(32) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "active_timeout" int4 DEFAULT 1800,
  "timeout" int4 DEFAULT 604800,
  "status" char(1) COLLATE "pg_catalog"."default" DEFAULT '0'::bpchar,
  "del_flag" char(1) COLLATE "pg_catalog"."default" DEFAULT '0'::bpchar,
  "create_dept" int8,
  "create_by" int8,
  "create_time" timestamp(6),
  "update_by" int8,
  "update_time" timestamp(6)
)
;
COMMENT ON COLUMN "public"."sys_client"."id" IS '主键';
COMMENT ON COLUMN "public"."sys_client"."client_id" IS '客户端id';
COMMENT ON COLUMN "public"."sys_client"."client_key" IS '客户端key';
COMMENT ON COLUMN "public"."sys_client"."client_secret" IS '客户端秘钥';
COMMENT ON COLUMN "public"."sys_client"."grant_type" IS '授权类型';
COMMENT ON COLUMN "public"."sys_client"."device_type" IS '设备类型';
COMMENT ON COLUMN "public"."sys_client"."active_timeout" IS 'token活跃超时时间';
COMMENT ON COLUMN "public"."sys_client"."timeout" IS 'token固定超时';
COMMENT ON COLUMN "public"."sys_client"."status" IS '状态（0正常 1停用）';
COMMENT ON COLUMN "public"."sys_client"."del_flag" IS '删除标志（0代表存在 1代表删除）';
COMMENT ON COLUMN "public"."sys_client"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."sys_client"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_client"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_client"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_client"."update_time" IS '更新时间';
COMMENT ON TABLE "public"."sys_client" IS '系统授权表';

-- ----------------------------
-- Records of sys_client
-- ----------------------------
INSERT INTO "public"."sys_client" VALUES (1, 'e5cd7e4891bf95d1d19206ce24a7b32e', 'pc', 'pc123', 'password,social', 'pc', 1800, 604800, '0', '0', 103, 1, '2026-09-30 23:07:47.248766', 1, '2026-09-30 23:07:47.248766');
INSERT INTO "public"."sys_client" VALUES (2, '428a8310cd442757ae699df5d894f051', 'app', 'app123', 'password,sms,social', 'android', 1800, 604800, '0', '0', 103, 1, '2026-09-30 23:07:47.248766', 1, '2026-09-30 23:07:47.248766');

-- ----------------------------
-- Table structure for sys_config
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_config";
CREATE TABLE "public"."sys_config" (
  "config_id" int8 NOT NULL,
  "config_name" varchar(100) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "config_key" varchar(100) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "config_value" varchar(500) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "config_type" char(1) COLLATE "pg_catalog"."default" DEFAULT 'N'::bpchar,
  "create_dept" int8,
  "create_by" int8,
  "create_time" timestamp(6),
  "update_by" int8,
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying
)
;
COMMENT ON COLUMN "public"."sys_config"."config_id" IS '参数主键';
COMMENT ON COLUMN "public"."sys_config"."config_name" IS '参数名称';
COMMENT ON COLUMN "public"."sys_config"."config_key" IS '参数键名';
COMMENT ON COLUMN "public"."sys_config"."config_value" IS '参数键值';
COMMENT ON COLUMN "public"."sys_config"."config_type" IS '系统内置（Y是 N否）';
COMMENT ON COLUMN "public"."sys_config"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."sys_config"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_config"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_config"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_config"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_config"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_config" IS '参数配置表';

-- ----------------------------
-- Records of sys_config
-- ----------------------------
INSERT INTO "public"."sys_config" VALUES (1, '主框架页-默认皮肤样式名称', 'sys.index.skinName', 'skin-blue', 'Y', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '蓝色 skin-blue、绿色 skin-green、紫色 skin-purple、红色 skin-red、黄色 skin-yellow');
INSERT INTO "public"."sys_config" VALUES (2, '用户管理-账号初始密码', 'sys.user.initPassword', '123456', 'Y', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '初始化密码 123456');
INSERT INTO "public"."sys_config" VALUES (3, '主框架页-侧边栏主题', 'sys.index.sideTheme', 'theme-dark', 'Y', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '深色主题theme-dark，浅色主题theme-light');
INSERT INTO "public"."sys_config" VALUES (5, '账号自助-是否开启用户注册功能', 'sys.account.registerUser', 'false', 'Y', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '是否开启注册用户功能（true开启，false关闭）');
INSERT INTO "public"."sys_config" VALUES (11, 'OSS预览列表资源开关', 'sys.oss.previewListResource', 'true', 'Y', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, 'true:开启, false:关闭');

-- ----------------------------
-- Table structure for sys_dept
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_dept";
CREATE TABLE "public"."sys_dept" (
  "dept_id" int8 NOT NULL,
  "parent_id" int8 DEFAULT 0,
  "ancestors" varchar(500) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "dept_name" varchar(30) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "dept_category" varchar(100) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "order_num" int4 DEFAULT 0,
  "leader" int8,
  "phone" varchar(11) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "email" varchar(50) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "status" char(1) COLLATE "pg_catalog"."default" DEFAULT '0'::bpchar,
  "del_flag" char(1) COLLATE "pg_catalog"."default" DEFAULT '0'::bpchar,
  "create_dept" int8,
  "create_by" int8,
  "create_time" timestamp(6),
  "update_by" int8,
  "update_time" timestamp(6)
)
;
COMMENT ON COLUMN "public"."sys_dept"."dept_id" IS '部门ID';
COMMENT ON COLUMN "public"."sys_dept"."parent_id" IS '父部门ID';
COMMENT ON COLUMN "public"."sys_dept"."ancestors" IS '祖级列表';
COMMENT ON COLUMN "public"."sys_dept"."dept_name" IS '部门名称';
COMMENT ON COLUMN "public"."sys_dept"."dept_category" IS '部门类别编码';
COMMENT ON COLUMN "public"."sys_dept"."order_num" IS '显示顺序';
COMMENT ON COLUMN "public"."sys_dept"."leader" IS '负责人';
COMMENT ON COLUMN "public"."sys_dept"."phone" IS '联系电话';
COMMENT ON COLUMN "public"."sys_dept"."email" IS '邮箱';
COMMENT ON COLUMN "public"."sys_dept"."status" IS '部门状态（0正常 1停用）';
COMMENT ON COLUMN "public"."sys_dept"."del_flag" IS '删除标志（0代表存在 1代表删除）';
COMMENT ON COLUMN "public"."sys_dept"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."sys_dept"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_dept"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_dept"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_dept"."update_time" IS '更新时间';
COMMENT ON TABLE "public"."sys_dept" IS '部门表';

-- ----------------------------
-- Records of sys_dept
-- ----------------------------
INSERT INTO "public"."sys_dept" VALUES (100, 0, '0', 'XXX科技', NULL, 0, NULL, '15888888888', 'xxx@qq.com', '0', '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL);
INSERT INTO "public"."sys_dept" VALUES (101, 100, '0,100', '深圳总公司', NULL, 1, NULL, '15888888888', 'xxx@qq.com', '0', '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL);
INSERT INTO "public"."sys_dept" VALUES (102, 100, '0,100', '长沙分公司', NULL, 2, NULL, '15888888888', 'xxx@qq.com', '0', '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL);
INSERT INTO "public"."sys_dept" VALUES (103, 101, '0,100,101', '研发部门', NULL, 1, 1, '15888888888', 'xxx@qq.com', '0', '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL);
INSERT INTO "public"."sys_dept" VALUES (104, 101, '0,100,101', '市场部门', NULL, 2, NULL, '15888888888', 'xxx@qq.com', '0', '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL);
INSERT INTO "public"."sys_dept" VALUES (105, 101, '0,100,101', '测试部门', NULL, 3, NULL, '15888888888', 'xxx@qq.com', '0', '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL);
INSERT INTO "public"."sys_dept" VALUES (106, 101, '0,100,101', '财务部门', NULL, 4, NULL, '15888888888', 'xxx@qq.com', '0', '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL);
INSERT INTO "public"."sys_dept" VALUES (107, 101, '0,100,101', '运维部门', NULL, 5, NULL, '15888888888', 'xxx@qq.com', '0', '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL);
INSERT INTO "public"."sys_dept" VALUES (108, 102, '0,100,102', '市场部门', NULL, 1, NULL, '15888888888', 'xxx@qq.com', '0', '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL);
INSERT INTO "public"."sys_dept" VALUES (109, 102, '0,100,102', '财务部门', NULL, 2, NULL, '15888888888', 'xxx@qq.com', '0', '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL);

-- ----------------------------
-- Table structure for sys_dict_data
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_dict_data";
CREATE TABLE "public"."sys_dict_data" (
  "dict_code" int8 NOT NULL,
  "dict_sort" int4 DEFAULT 0,
  "dict_label" varchar(100) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "dict_value" varchar(100) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "dict_type" varchar(100) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "css_class" varchar(100) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "list_class" varchar(100) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "is_default" char(1) COLLATE "pg_catalog"."default" DEFAULT 'N'::bpchar,
  "create_dept" int8,
  "create_by" int8,
  "create_time" timestamp(6),
  "update_by" int8,
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying
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
COMMENT ON COLUMN "public"."sys_dict_data"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."sys_dict_data"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_dict_data"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_dict_data"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_dict_data"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_dict_data"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_dict_data" IS '字典数据表';

-- ----------------------------
-- Records of sys_dict_data
-- ----------------------------
INSERT INTO "public"."sys_dict_data" VALUES (1, 1, '男', '0', 'sys_user_sex', '', '', 'Y', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '性别男');
INSERT INTO "public"."sys_dict_data" VALUES (2, 2, '女', '1', 'sys_user_sex', '', '', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '性别女');
INSERT INTO "public"."sys_dict_data" VALUES (3, 3, '未知', '2', 'sys_user_sex', '', '', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '性别未知');
INSERT INTO "public"."sys_dict_data" VALUES (4, 1, '显示', '0', 'sys_show_hide', '', 'primary', 'Y', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '显示菜单');
INSERT INTO "public"."sys_dict_data" VALUES (5, 2, '隐藏', '1', 'sys_show_hide', '', 'danger', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '隐藏菜单');
INSERT INTO "public"."sys_dict_data" VALUES (6, 1, '正常', '0', 'sys_normal_disable', '', 'primary', 'Y', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '正常状态');
INSERT INTO "public"."sys_dict_data" VALUES (7, 2, '停用', '1', 'sys_normal_disable', '', 'danger', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '停用状态');
INSERT INTO "public"."sys_dict_data" VALUES (12, 1, '是', 'Y', 'sys_yes_no', '', 'primary', 'Y', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '系统默认是');
INSERT INTO "public"."sys_dict_data" VALUES (13, 2, '否', 'N', 'sys_yes_no', '', 'danger', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '系统默认否');
INSERT INTO "public"."sys_dict_data" VALUES (14, 1, '通知', '1', 'sys_notice_type', '', 'warning', 'Y', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '通知');
INSERT INTO "public"."sys_dict_data" VALUES (15, 2, '公告', '2', 'sys_notice_type', '', 'success', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '公告');
INSERT INTO "public"."sys_dict_data" VALUES (16, 1, '正常', '0', 'sys_notice_status', '', 'primary', 'Y', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '正常状态');
INSERT INTO "public"."sys_dict_data" VALUES (17, 2, '关闭', '1', 'sys_notice_status', '', 'danger', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '关闭状态');
INSERT INTO "public"."sys_dict_data" VALUES (29, 99, '其他', '0', 'sys_oper_type', '', 'info', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '其他操作');
INSERT INTO "public"."sys_dict_data" VALUES (18, 1, '新增', '1', 'sys_oper_type', '', 'info', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '新增操作');
INSERT INTO "public"."sys_dict_data" VALUES (19, 2, '修改', '2', 'sys_oper_type', '', 'info', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '修改操作');
INSERT INTO "public"."sys_dict_data" VALUES (20, 3, '删除', '3', 'sys_oper_type', '', 'danger', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '删除操作');
INSERT INTO "public"."sys_dict_data" VALUES (21, 4, '授权', '4', 'sys_oper_type', '', 'primary', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '授权操作');
INSERT INTO "public"."sys_dict_data" VALUES (22, 5, '导出', '5', 'sys_oper_type', '', 'warning', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '导出操作');
INSERT INTO "public"."sys_dict_data" VALUES (23, 6, '导入', '6', 'sys_oper_type', '', 'warning', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '导入操作');
INSERT INTO "public"."sys_dict_data" VALUES (24, 7, '强退', '7', 'sys_oper_type', '', 'danger', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '强退操作');
INSERT INTO "public"."sys_dict_data" VALUES (25, 8, '生成代码', '8', 'sys_oper_type', '', 'warning', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '生成操作');
INSERT INTO "public"."sys_dict_data" VALUES (26, 9, '清空数据', '9', 'sys_oper_type', '', 'danger', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '清空操作');
INSERT INTO "public"."sys_dict_data" VALUES (27, 1, '成功', '0', 'sys_common_status', '', 'primary', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '正常状态');
INSERT INTO "public"."sys_dict_data" VALUES (28, 2, '失败', '1', 'sys_common_status', '', 'danger', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '停用状态');
INSERT INTO "public"."sys_dict_data" VALUES (30, 0, '密码认证', 'password', 'sys_grant_type', '', 'default', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '密码认证');
INSERT INTO "public"."sys_dict_data" VALUES (31, 0, '短信认证', 'sms', 'sys_grant_type', '', 'default', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '短信认证');
INSERT INTO "public"."sys_dict_data" VALUES (32, 0, '邮件认证', 'email', 'sys_grant_type', '', 'default', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '邮件认证');
INSERT INTO "public"."sys_dict_data" VALUES (33, 0, '小程序认证', 'xcx', 'sys_grant_type', '', 'default', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '小程序认证');
INSERT INTO "public"."sys_dict_data" VALUES (34, 0, '三方登录认证', 'social', 'sys_grant_type', '', 'default', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '三方登录认证');
INSERT INTO "public"."sys_dict_data" VALUES (35, 0, 'PC', 'pc', 'sys_device_type', '', 'default', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, 'PC');
INSERT INTO "public"."sys_dict_data" VALUES (36, 0, '安卓', 'android', 'sys_device_type', '', 'default', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '安卓');
INSERT INTO "public"."sys_dict_data" VALUES (37, 0, 'iOS', 'ios', 'sys_device_type', '', 'default', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, 'iOS');
INSERT INTO "public"."sys_dict_data" VALUES (38, 0, '小程序', 'xcx', 'sys_device_type', '', 'default', 'N', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '小程序');
INSERT INTO "public"."sys_dict_data" VALUES (32133830950913, 0, '开', '开', 'course_status', '', 'primary', NULL, NULL, 1, NULL, NULL, NULL, '');
INSERT INTO "public"."sys_dict_data" VALUES (32133833834497, 1, '关', '关', 'course_status', '', 'primary', NULL, NULL, 1, NULL, NULL, NULL, '关');

-- ----------------------------
-- Table structure for sys_dict_type
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_dict_type";
CREATE TABLE "public"."sys_dict_type" (
  "dict_id" int8 NOT NULL,
  "dict_name" varchar(100) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "dict_type" varchar(100) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "create_dept" int8,
  "create_by" int8,
  "create_time" timestamp(6),
  "update_by" int8,
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying
)
;
COMMENT ON COLUMN "public"."sys_dict_type"."dict_id" IS '字典主键';
COMMENT ON COLUMN "public"."sys_dict_type"."dict_name" IS '字典名称';
COMMENT ON COLUMN "public"."sys_dict_type"."dict_type" IS '字典类型';
COMMENT ON COLUMN "public"."sys_dict_type"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."sys_dict_type"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_dict_type"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_dict_type"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_dict_type"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_dict_type"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_dict_type" IS '字典类型表';

-- ----------------------------
-- Records of sys_dict_type
-- ----------------------------
INSERT INTO "public"."sys_dict_type" VALUES (1, '用户性别', 'sys_user_sex', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '用户性别列表');
INSERT INTO "public"."sys_dict_type" VALUES (2, '菜单状态', 'sys_show_hide', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '菜单状态列表');
INSERT INTO "public"."sys_dict_type" VALUES (3, '系统开关', 'sys_normal_disable', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '系统开关列表');
INSERT INTO "public"."sys_dict_type" VALUES (6, '系统是否', 'sys_yes_no', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '系统是否列表');
INSERT INTO "public"."sys_dict_type" VALUES (7, '通知类型', 'sys_notice_type', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '通知类型列表');
INSERT INTO "public"."sys_dict_type" VALUES (8, '通知状态', 'sys_notice_status', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '通知状态列表');
INSERT INTO "public"."sys_dict_type" VALUES (9, '操作类型', 'sys_oper_type', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '操作类型列表');
INSERT INTO "public"."sys_dict_type" VALUES (10, '系统状态', 'sys_common_status', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '登录状态列表');
INSERT INTO "public"."sys_dict_type" VALUES (11, '授权类型', 'sys_grant_type', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '认证授权类型');
INSERT INTO "public"."sys_dict_type" VALUES (12, '设备类型', 'sys_device_type', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '客户端设备类型');
INSERT INTO "public"."sys_dict_type" VALUES (32133825445889, 'course_status', 'course_status', NULL, 1, '2026-10-08 20:27:12.43081', NULL, NULL, 'course_status');

-- ----------------------------
-- Table structure for sys_job
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_job";
CREATE TABLE "public"."sys_job" (
  "job_id" int8 NOT NULL DEFAULT nextval('sys_job_job_id_seq'::regclass),
  "job_name" varchar(64) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "job_group" varchar(64) COLLATE "pg_catalog"."default" NOT NULL DEFAULT 'DEFAULT'::character varying,
  "invoke_target" varchar(500) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "cron_expression" varchar(255) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "misfire_policy" varchar(20) COLLATE "pg_catalog"."default" NOT NULL DEFAULT '3'::character varying,
  "concurrent" varchar(1) COLLATE "pg_catalog"."default" NOT NULL DEFAULT '1'::character varying,
  "status" varchar(1) COLLATE "pg_catalog"."default" NOT NULL DEFAULT '0'::character varying,
  "create_by" varchar(64) COLLATE "pg_catalog"."default",
  "create_time" timestamp(6),
  "update_by" varchar(64) COLLATE "pg_catalog"."default",
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default"
)
;

-- ----------------------------
-- Records of sys_job
-- ----------------------------

-- ----------------------------
-- Table structure for sys_job_log
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_job_log";
CREATE TABLE "public"."sys_job_log" (
  "job_log_id" int8 NOT NULL DEFAULT nextval('sys_job_log_job_log_id_seq'::regclass),
  "job_name" varchar(64) COLLATE "pg_catalog"."default",
  "job_group" varchar(64) COLLATE "pg_catalog"."default",
  "invoke_target" varchar(500) COLLATE "pg_catalog"."default",
  "job_message" varchar(500) COLLATE "pg_catalog"."default",
  "status" varchar(1) COLLATE "pg_catalog"."default" NOT NULL DEFAULT '0'::character varying,
  "exception_info" text COLLATE "pg_catalog"."default",
  "create_time" timestamp(6)
)
;

-- ----------------------------
-- Records of sys_job_log
-- ----------------------------

-- ----------------------------
-- Table structure for sys_logininfor
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_logininfor";
CREATE TABLE "public"."sys_logininfor" (
  "info_id" int8 NOT NULL,
  "user_name" varchar(50) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "client_key" varchar(32) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "device_type" varchar(32) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "ipaddr" varchar(128) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "login_location" varchar(255) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "browser" varchar(50) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "os" varchar(50) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "status" char(1) COLLATE "pg_catalog"."default" DEFAULT '0'::bpchar,
  "msg" varchar(255) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "login_time" timestamp(6)
)
;
COMMENT ON COLUMN "public"."sys_logininfor"."info_id" IS '访问ID';
COMMENT ON COLUMN "public"."sys_logininfor"."user_name" IS '用户账号';
COMMENT ON COLUMN "public"."sys_logininfor"."client_key" IS '客户端';
COMMENT ON COLUMN "public"."sys_logininfor"."device_type" IS '设备类型';
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
INSERT INTO "public"."sys_logininfor" VALUES (32125418356737, 'admin', '', '', '127.0.0.1', NULL, NULL, 'Windows', '0', '成功', '2026-10-08 02:38:11.596451');


-- ----------------------------
-- Table structure for sys_menu
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_menu";
CREATE TABLE "public"."sys_menu" (
  "menu_id" int8 NOT NULL,
  "menu_name" varchar(50) COLLATE "pg_catalog"."default" NOT NULL,
  "parent_id" int8 DEFAULT 0,
  "order_num" int4 DEFAULT 0,
  "path" varchar(200) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "component" varchar(255) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "query_param" varchar(255) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "is_frame" char(1) COLLATE "pg_catalog"."default" DEFAULT '1'::bpchar,
  "is_cache" char(1) COLLATE "pg_catalog"."default" DEFAULT '0'::bpchar,
  "menu_type" char(1) COLLATE "pg_catalog"."default" DEFAULT ''::bpchar,
  "visible" char(1) COLLATE "pg_catalog"."default" DEFAULT '0'::bpchar,
  "status" char(1) COLLATE "pg_catalog"."default" DEFAULT '0'::bpchar,
  "perms" varchar(100) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "icon" varchar(100) COLLATE "pg_catalog"."default" DEFAULT '#'::character varying,
  "create_dept" int8,
  "create_by" int8,
  "create_time" timestamp(6),
  "update_by" int8,
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default" DEFAULT ''::character varying
)
;
COMMENT ON COLUMN "public"."sys_menu"."menu_id" IS '菜单ID';
COMMENT ON COLUMN "public"."sys_menu"."menu_name" IS '菜单名称';
COMMENT ON COLUMN "public"."sys_menu"."parent_id" IS '父菜单ID';
COMMENT ON COLUMN "public"."sys_menu"."order_num" IS '显示顺序';
COMMENT ON COLUMN "public"."sys_menu"."path" IS '路由地址';
COMMENT ON COLUMN "public"."sys_menu"."component" IS '组件路径';
COMMENT ON COLUMN "public"."sys_menu"."query_param" IS '路由参数';
COMMENT ON COLUMN "public"."sys_menu"."is_frame" IS '是否为外链（0是 1否）';
COMMENT ON COLUMN "public"."sys_menu"."is_cache" IS '是否缓存（0缓存 1不缓存）';
COMMENT ON COLUMN "public"."sys_menu"."menu_type" IS '菜单类型（M目录 C菜单 F按钮）';
COMMENT ON COLUMN "public"."sys_menu"."visible" IS '显示状态（0显示 1隐藏）';
COMMENT ON COLUMN "public"."sys_menu"."status" IS '菜单状态（0正常 1停用）';
COMMENT ON COLUMN "public"."sys_menu"."perms" IS '权限标识';
COMMENT ON COLUMN "public"."sys_menu"."icon" IS '菜单图标';
COMMENT ON COLUMN "public"."sys_menu"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."sys_menu"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_menu"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_menu"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_menu"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_menu"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_menu" IS '菜单权限表';

-- ----------------------------
-- Records of sys_menu
-- ----------------------------
INSERT INTO "public"."sys_menu" VALUES (1, '系统管理', 0, 1, 'system', NULL, '', '1', '0', 'M', '0', '0', '', 'system', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '系统管理目录');
INSERT INTO "public"."sys_menu" VALUES (2, '系统监控', 0, 3, 'monitor', NULL, '', '1', '0', 'M', '0', '0', '', 'monitor', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '系统监控目录');
INSERT INTO "public"."sys_menu" VALUES (3, '系统工具', 0, 4, 'tool', NULL, '', '1', '0', 'M', '0', '0', '', 'tool', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '系统工具目录');
INSERT INTO "public"."sys_menu" VALUES (4, 'PLUS官网', 0, 5, 'https://gitee.com/dromara/RuoYi-Vue-Plus', NULL, '', '0', '0', 'M', '0', '0', '', 'guide', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, 'RuoYi-Vue-Plus官网地址');
INSERT INTO "public"."sys_menu" VALUES (100, '用户管理', 1, 1, 'user', 'system/user/index', '', '1', '0', 'C', '0', '0', 'system:user:list', 'user', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '用户管理菜单');
INSERT INTO "public"."sys_menu" VALUES (101, '角色管理', 1, 2, 'role', 'system/role/index', '', '1', '0', 'C', '0', '0', 'system:role:list', 'peoples', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '角色管理菜单');
INSERT INTO "public"."sys_menu" VALUES (102, '菜单管理', 1, 3, 'menu', 'system/menu/index', '', '1', '0', 'C', '0', '0', 'system:menu:list', 'tree-table', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '菜单管理菜单');
INSERT INTO "public"."sys_menu" VALUES (103, '部门管理', 1, 4, 'dept', 'system/dept/index', '', '1', '0', 'C', '0', '0', 'system:dept:list', 'tree', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '部门管理菜单');
INSERT INTO "public"."sys_menu" VALUES (104, '岗位管理', 1, 5, 'post', 'system/post/index', '', '1', '0', 'C', '0', '0', 'system:post:list', 'post', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '岗位管理菜单');
INSERT INTO "public"."sys_menu" VALUES (105, '字典管理', 1, 6, 'dict', 'system/dict/index', '', '1', '0', 'C', '0', '0', 'system:dict:list', 'dict', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '字典管理菜单');
INSERT INTO "public"."sys_menu" VALUES (106, '参数设置', 1, 7, 'config', 'system/config/index', '', '1', '0', 'C', '0', '0', 'system:config:list', 'edit', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '参数设置菜单');
INSERT INTO "public"."sys_menu" VALUES (107, '通知公告', 1, 8, 'notice', 'system/notice/index', '', '1', '0', 'C', '0', '0', 'system:notice:list', 'message', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '通知公告菜单');
INSERT INTO "public"."sys_menu" VALUES (108, '日志管理', 1, 9, 'log', '', '', '1', '0', 'M', '0', '0', '', 'log', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '日志管理菜单');
INSERT INTO "public"."sys_menu" VALUES (109, '在线用户', 2, 1, 'online', 'monitor/online/index', '', '1', '0', 'C', '0', '0', 'monitor:online:list', 'online', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '在线用户菜单');
INSERT INTO "public"."sys_menu" VALUES (113, '缓存监控', 2, 5, 'cache', 'monitor/cache/index', '', '1', '0', 'C', '0', '0', 'monitor:cache:list', 'redis', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '缓存监控菜单');
INSERT INTO "public"."sys_menu" VALUES (115, '代码生成', 3, 2, 'gen', 'tool/gen/index', '', '1', '0', 'C', '0', '0', 'tool:gen:list', 'code', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '代码生成菜单');
INSERT INTO "public"."sys_menu" VALUES (123, '客户端管理', 1, 11, 'client', 'system/client/index', '', '1', '0', 'C', '0', '0', 'system:client:list', 'international', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '客户端管理菜单');
INSERT INTO "public"."sys_menu" VALUES (116, '修改生成配置', 3, 2, 'gen-edit/index/:tableId', 'tool/gen/editTable', '', '1', '1', 'C', '1', '0', 'tool:gen:edit', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '/tool/gen');
INSERT INTO "public"."sys_menu" VALUES (130, '分配用户', 1, 2, 'role-auth/user/:roleId', 'system/role/authUser', '', '1', '1', 'C', '1', '0', 'system:role:edit', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '/system/role');
INSERT INTO "public"."sys_menu" VALUES (131, '分配角色', 1, 1, 'user-auth/role/:userId', 'system/user/authRole', '', '1', '1', 'C', '1', '0', 'system:user:edit', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '/system/user');
INSERT INTO "public"."sys_menu" VALUES (132, '字典数据', 1, 6, 'dict-data/index/:dictId', 'system/dict/data', '', '1', '1', 'C', '1', '0', 'system:dict:list', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '/system/dict');
INSERT INTO "public"."sys_menu" VALUES (133, '文件配置管理', 1, 10, 'oss-config/index', 'system/oss/config', '', '1', '1', 'C', '1', '0', 'system:ossConfig:list', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '/system/oss');
INSERT INTO "public"."sys_menu" VALUES (117, 'Admin监控', 2, 5, 'Admin', 'monitor/admin/index', '', '1', '0', 'C', '0', '0', 'monitor:admin:list', 'dashboard', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, 'Admin监控菜单');
INSERT INTO "public"."sys_menu" VALUES (118, '文件管理', 1, 10, 'oss', 'system/oss/index', '', '1', '0', 'C', '0', '0', 'system:oss:list', 'upload', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '文件管理菜单');
INSERT INTO "public"."sys_menu" VALUES (120, '任务调度中心', 2, 6, 'snailjob', 'monitor/snailjob/index', '', '1', '0', 'C', '0', '0', 'monitor:snailjob:list', 'job', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, 'SnailJob控制台菜单');
INSERT INTO "public"."sys_menu" VALUES (500, '操作日志', 108, 1, 'operlog', 'monitor/operlog/index', '', '1', '0', 'C', '0', '0', 'monitor:operlog:list', 'form', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '操作日志菜单');
INSERT INTO "public"."sys_menu" VALUES (501, '登录日志', 108, 2, 'logininfor', 'monitor/logininfor/index', '', '1', '0', 'C', '0', '0', 'monitor:logininfor:list', 'logininfor', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '登录日志菜单');
INSERT INTO "public"."sys_menu" VALUES (1001, '用户查询', 100, 1, '', '', '', '1', '0', 'F', '0', '0', 'system:user:query', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1002, '用户新增', 100, 2, '', '', '', '1', '0', 'F', '0', '0', 'system:user:add', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1003, '用户修改', 100, 3, '', '', '', '1', '0', 'F', '0', '0', 'system:user:edit', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1004, '用户删除', 100, 4, '', '', '', '1', '0', 'F', '0', '0', 'system:user:remove', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1005, '用户导出', 100, 5, '', '', '', '1', '0', 'F', '0', '0', 'system:user:export', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1006, '用户导入', 100, 6, '', '', '', '1', '0', 'F', '0', '0', 'system:user:import', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1007, '重置密码', 100, 7, '', '', '', '1', '0', 'F', '0', '0', 'system:user:resetPwd', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1008, '角色查询', 101, 1, '', '', '', '1', '0', 'F', '0', '0', 'system:role:query', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1009, '角色新增', 101, 2, '', '', '', '1', '0', 'F', '0', '0', 'system:role:add', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1010, '角色修改', 101, 3, '', '', '', '1', '0', 'F', '0', '0', 'system:role:edit', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1011, '角色删除', 101, 4, '', '', '', '1', '0', 'F', '0', '0', 'system:role:remove', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1012, '角色导出', 101, 5, '', '', '', '1', '0', 'F', '0', '0', 'system:role:export', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1013, '菜单查询', 102, 1, '', '', '', '1', '0', 'F', '0', '0', 'system:menu:query', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1014, '菜单新增', 102, 2, '', '', '', '1', '0', 'F', '0', '0', 'system:menu:add', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1015, '菜单修改', 102, 3, '', '', '', '1', '0', 'F', '0', '0', 'system:menu:edit', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1016, '菜单删除', 102, 4, '', '', '', '1', '0', 'F', '0', '0', 'system:menu:remove', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1017, '部门查询', 103, 1, '', '', '', '1', '0', 'F', '0', '0', 'system:dept:query', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1018, '部门新增', 103, 2, '', '', '', '1', '0', 'F', '0', '0', 'system:dept:add', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1019, '部门修改', 103, 3, '', '', '', '1', '0', 'F', '0', '0', 'system:dept:edit', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1020, '部门删除', 103, 4, '', '', '', '1', '0', 'F', '0', '0', 'system:dept:remove', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1021, '岗位查询', 104, 1, '', '', '', '1', '0', 'F', '0', '0', 'system:post:query', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1022, '岗位新增', 104, 2, '', '', '', '1', '0', 'F', '0', '0', 'system:post:add', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1023, '岗位修改', 104, 3, '', '', '', '1', '0', 'F', '0', '0', 'system:post:edit', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1024, '岗位删除', 104, 4, '', '', '', '1', '0', 'F', '0', '0', 'system:post:remove', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1025, '岗位导出', 104, 5, '', '', '', '1', '0', 'F', '0', '0', 'system:post:export', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1026, '字典查询', 105, 1, '#', '', '', '1', '0', 'F', '0', '0', 'system:dict:query', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1027, '字典新增', 105, 2, '#', '', '', '1', '0', 'F', '0', '0', 'system:dict:add', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1028, '字典修改', 105, 3, '#', '', '', '1', '0', 'F', '0', '0', 'system:dict:edit', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1029, '字典删除', 105, 4, '#', '', '', '1', '0', 'F', '0', '0', 'system:dict:remove', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1030, '字典导出', 105, 5, '#', '', '', '1', '0', 'F', '0', '0', 'system:dict:export', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1031, '参数查询', 106, 1, '#', '', '', '1', '0', 'F', '0', '0', 'system:config:query', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1032, '参数新增', 106, 2, '#', '', '', '1', '0', 'F', '0', '0', 'system:config:add', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1033, '参数修改', 106, 3, '#', '', '', '1', '0', 'F', '0', '0', 'system:config:edit', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1034, '参数删除', 106, 4, '#', '', '', '1', '0', 'F', '0', '0', 'system:config:remove', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1035, '参数导出', 106, 5, '#', '', '', '1', '0', 'F', '0', '0', 'system:config:export', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1036, '公告查询', 107, 1, '#', '', '', '1', '0', 'F', '0', '0', 'system:notice:query', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1037, '公告新增', 107, 2, '#', '', '', '1', '0', 'F', '0', '0', 'system:notice:add', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1038, '公告修改', 107, 3, '#', '', '', '1', '0', 'F', '0', '0', 'system:notice:edit', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1039, '公告删除', 107, 4, '#', '', '', '1', '0', 'F', '0', '0', 'system:notice:remove', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1040, '操作查询', 500, 1, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:operlog:query', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1041, '操作删除', 500, 2, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:operlog:remove', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1042, '日志导出', 500, 4, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:operlog:export', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1043, '登录查询', 501, 1, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:logininfor:query', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1044, '登录删除', 501, 2, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:logininfor:remove', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1045, '日志导出', 501, 3, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:logininfor:export', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1050, '账户解锁', 501, 4, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:logininfor:unlock', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1046, '在线查询', 109, 1, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:online:query', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1047, '批量强退', 109, 2, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:online:batchLogout', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1048, '单条强退', 109, 3, '#', '', '', '1', '0', 'F', '0', '0', 'monitor:online:forceLogout', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1055, '生成查询', 115, 1, '#', '', '', '1', '0', 'F', '0', '0', 'tool:gen:query', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1056, '生成修改', 115, 2, '#', '', '', '1', '0', 'F', '0', '0', 'tool:gen:edit', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1057, '生成删除', 115, 3, '#', '', '', '1', '0', 'F', '0', '0', 'tool:gen:remove', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1058, '导入代码', 115, 2, '#', '', '', '1', '0', 'F', '0', '0', 'tool:gen:import', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1059, '预览代码', 115, 4, '#', '', '', '1', '0', 'F', '0', '0', 'tool:gen:preview', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1060, '生成代码', 115, 5, '#', '', '', '1', '0', 'F', '0', '0', 'tool:gen:code', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1600, '文件查询', 118, 1, '#', '', '', '1', '0', 'F', '0', '0', 'system:oss:query', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1601, '文件上传', 118, 2, '#', '', '', '1', '0', 'F', '0', '0', 'system:oss:upload', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1602, '文件下载', 118, 3, '#', '', '', '1', '0', 'F', '0', '0', 'system:oss:download', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1603, '文件删除', 118, 4, '#', '', '', '1', '0', 'F', '0', '0', 'system:oss:remove', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1620, '配置列表', 118, 5, '#', '', '', '1', '0', 'F', '0', '0', 'system:ossConfig:list', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1621, '配置添加', 118, 6, '#', '', '', '1', '0', 'F', '0', '0', 'system:ossConfig:add', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1622, '配置编辑', 118, 6, '#', '', '', '1', '0', 'F', '0', '0', 'system:ossConfig:edit', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1623, '配置删除', 118, 6, '#', '', '', '1', '0', 'F', '0', '0', 'system:ossConfig:remove', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1061, '客户端管理查询', 123, 1, '#', '', '', '1', '0', 'F', '0', '0', 'system:client:query', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1062, '客户端管理新增', 123, 2, '#', '', '', '1', '0', 'F', '0', '0', 'system:client:add', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1063, '客户端管理修改', 123, 3, '#', '', '', '1', '0', 'F', '0', '0', 'system:client:edit', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1064, '客户端管理删除', 123, 4, '#', '', '', '1', '0', 'F', '0', '0', 'system:client:remove', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1065, '客户端管理导出', 123, 5, '#', '', '', '1', '0', 'F', '0', '0', 'system:client:export', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1500, '测试单表', 5, 1, 'demo', 'demo/demo/index', '', '1', '0', 'C', '0', '0', 'demo:demo:list', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '测试单表菜单');
INSERT INTO "public"."sys_menu" VALUES (1501, '测试单表查询', 1500, 1, '#', '', '', '1', '0', 'F', '0', '0', 'demo:demo:query', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1502, '测试单表新增', 1500, 2, '#', '', '', '1', '0', 'F', '0', '0', 'demo:demo:add', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1503, '测试单表修改', 1500, 3, '#', '', '', '1', '0', 'F', '0', '0', 'demo:demo:edit', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1504, '测试单表删除', 1500, 4, '#', '', '', '1', '0', 'F', '0', '0', 'demo:demo:remove', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1505, '测试单表导出', 1500, 5, '#', '', '', '1', '0', 'F', '0', '0', 'demo:demo:export', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1506, '测试树表', 5, 1, 'tree', 'demo/tree/index', '', '1', '0', 'C', '0', '0', 'demo:tree:list', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '测试树表菜单');
INSERT INTO "public"."sys_menu" VALUES (1507, '测试树表查询', 1506, 1, '#', '', '', '1', '0', 'F', '0', '0', 'demo:tree:query', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1508, '测试树表新增', 1506, 2, '#', '', '', '1', '0', 'F', '0', '0', 'demo:tree:add', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1509, '测试树表修改', 1506, 3, '#', '', '', '1', '0', 'F', '0', '0', 'demo:tree:edit', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1510, '测试树表删除', 1506, 4, '#', '', '', '1', '0', 'F', '0', '0', 'demo:tree:remove', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_menu" VALUES (1511, '测试树表导出', 1506, 5, '#', '', '', '1', '0', 'F', '0', '0', 'demo:tree:export', '#', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');


-- ----------------------------
-- Table structure for sys_notice
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_notice";
CREATE TABLE "public"."sys_notice" (
  "notice_id" int8 NOT NULL,
  "notice_title" varchar(50) COLLATE "pg_catalog"."default" NOT NULL,
  "notice_type" char(1) COLLATE "pg_catalog"."default" NOT NULL,
  "notice_content" text COLLATE "pg_catalog"."default",
  "status" char(1) COLLATE "pg_catalog"."default" DEFAULT '0'::bpchar,
  "create_dept" int8,
  "create_by" int8,
  "create_time" timestamp(6),
  "update_by" int8,
  "update_time" timestamp(6),
  "remark" varchar(255) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying
)
;
COMMENT ON COLUMN "public"."sys_notice"."notice_id" IS '公告ID';
COMMENT ON COLUMN "public"."sys_notice"."notice_title" IS '公告标题';
COMMENT ON COLUMN "public"."sys_notice"."notice_type" IS '公告类型（1通知 2公告）';
COMMENT ON COLUMN "public"."sys_notice"."notice_content" IS '公告内容';
COMMENT ON COLUMN "public"."sys_notice"."status" IS '公告状态（0正常 1关闭）';
COMMENT ON COLUMN "public"."sys_notice"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."sys_notice"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_notice"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_notice"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_notice"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_notice"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_notice" IS '通知公告表';

-- ----------------------------
-- Records of sys_notice
-- ----------------------------
INSERT INTO "public"."sys_notice" VALUES (1, '温馨提醒：2018-07-01 新版本发布啦', '2', '新版本内容', '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '管理员');
INSERT INTO "public"."sys_notice" VALUES (2, '维护通知：2018-07-01 系统凌晨维护', '1', '维护内容', '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '管理员');
INSERT INTO "public"."sys_notice" VALUES (32130286632961, 'smoke_cb_1791435429', '1', 'refactor verify', '0', NULL, 1, '2026-10-08 12:57:13.262004', NULL, NULL, NULL);
INSERT INTO "public"."sys_notice" VALUES (32130301837313, 'smoke_refactor_updated', '2', 'u', '0', NULL, 1, '2026-10-08 12:59:09.35847', 1, '2026-10-08 13:00:06.792848', NULL);

-- ----------------------------
-- Table structure for sys_oper_log
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_oper_log";
CREATE TABLE "public"."sys_oper_log" (
  "oper_id" int8 NOT NULL,
  "title" varchar(50) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "business_type" int4 DEFAULT 0,
  "method" varchar(100) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "request_method" varchar(10) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "operator_type" int4 DEFAULT 0,
  "oper_name" varchar(50) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "dept_name" varchar(50) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "oper_url" varchar(255) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "oper_ip" varchar(128) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "oper_location" varchar(255) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "oper_param" varchar(4000) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "json_result" varchar(4000) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "status" int4 DEFAULT 0,
  "error_msg" varchar(4000) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "oper_time" timestamp(6),
  "cost_time" int8 DEFAULT 0
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
COMMENT ON COLUMN "public"."sys_oper_log"."cost_time" IS '消耗时间';
COMMENT ON TABLE "public"."sys_oper_log" IS '操作日志记录';

-- ----------------------------
-- Records of sys_oper_log
-- ----------------------------
INSERT INTO "public"."sys_oper_log" VALUES (32125630431234, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 03:05:09.854617', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32125632135170, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 03:05:22.351775', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32125632135172, '退出登录', 6, 'POST /auth/logout', 'POST', 1, 'admin', NULL, '/auth/logout', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 03:05:22.356078', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32125633970178, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 03:05:36.900477', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32125656645634, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 03:08:29.199672', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32125719166977, '用户管理', 3, 'DELETE /system/user/{userIds}', 'DELETE', 1, 'admin', NULL, '/system/user', NULL, NULL, NULL, NULL, 1, NULL, '2026-10-08 03:16:26.929609', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32125778018306, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 03:23:55.223129', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32125822582786, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 03:29:35.520538', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32125832675330, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 03:30:52.594959', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32127574228994, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 07:12:19.780711', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32127594414082, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 07:14:53.693496', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32127599919106, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 07:15:35.038333', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32127619710978, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 07:18:06.225456', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32127639240706, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 07:20:35.289977', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32127660081153, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 07:23:14.000096', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32127680921602, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 07:25:53.523594', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32130264875010, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 12:54:27.441411', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32130286108674, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 12:57:09.191548', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32130291613697, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 1, '登录失败', '2026-10-08 12:57:51.18738', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32130301313026, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 12:59:05.287295', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32130308784130, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 13:00:02.699213', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32132199628802, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 17:00:28.914917', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32132224794626, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 17:03:40.612788', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32132264247298, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 17:08:41.49664', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32132863901698, '退出登录', 6, 'POST /auth/logout', 'POST', 1, '', NULL, '/auth/logout', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 18:24:56.121789', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32132890640386, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 18:28:20.340723', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32132926685186, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 18:32:55.277995', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32132928520194, '退出登录', 6, 'POST /auth/logout', 'POST', 1, 'admin', NULL, '/auth/logout', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 18:33:09.575237', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32132937695234, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 18:34:19.721703', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32132938219522, '退出登录', 6, 'POST /auth/logout', 'POST', 1, 'admin', NULL, '/auth/logout', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 18:34:23.792075', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32133116739586, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 18:57:05.875598', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32133118705666, '退出登录', 6, 'POST /auth/logout', 'POST', 1, 'admin', NULL, '/auth/logout', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 18:57:20.133321', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32133139677186, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 19:00:00.901148', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32133143085058, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 19:00:26.717112', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32133254627330, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 19:14:37.528714', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32133480595457, '用户管理', 2, 'PUT /system/user', 'PUT', 1, 'admin', NULL, '/system/user', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 19:43:21.118591', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32133480988673, '用户管理', 2, 'PUT /system/user', 'PUT', 1, 'admin', NULL, '/system/user', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 19:43:24.987021', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32133622677506, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 20:01:25.915449', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32133788221442, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 20:22:28.37075', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32133890457602, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 20:35:28.756511', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32133968183298, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 20:45:21.096897', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32133994528770, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 20:48:42.771223', 0);
INSERT INTO "public"."sys_oper_log" VALUES (32134021398530, '登录', 5, 'POST /auth/login', 'POST', 1, 'admin', NULL, '/auth/login', NULL, NULL, NULL, NULL, 0, NULL, '2026-10-08 20:52:07.808595', 0);

-- ----------------------------
-- Table structure for sys_oss
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_oss";
CREATE TABLE "public"."sys_oss" (
  "oss_id" int8 NOT NULL,
  "file_name" varchar(255) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "original_name" varchar(255) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "file_suffix" varchar(10) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "url" varchar(500) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "ext1" varchar(500) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "create_dept" int8,
  "create_by" int8,
  "create_time" timestamp(6),
  "update_by" int8,
  "update_time" timestamp(6),
  "service" varchar(20) COLLATE "pg_catalog"."default" DEFAULT 'minio'::character varying
)
;
COMMENT ON COLUMN "public"."sys_oss"."oss_id" IS '对象存储主键';
COMMENT ON COLUMN "public"."sys_oss"."file_name" IS '文件名';
COMMENT ON COLUMN "public"."sys_oss"."original_name" IS '原名';
COMMENT ON COLUMN "public"."sys_oss"."file_suffix" IS '文件后缀名';
COMMENT ON COLUMN "public"."sys_oss"."url" IS 'URL地址';
COMMENT ON COLUMN "public"."sys_oss"."ext1" IS '扩展字段';
COMMENT ON COLUMN "public"."sys_oss"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."sys_oss"."create_by" IS '上传人';
COMMENT ON COLUMN "public"."sys_oss"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_oss"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_oss"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_oss"."service" IS '服务商';
COMMENT ON TABLE "public"."sys_oss" IS 'OSS对象存储表';

-- ----------------------------
-- Records of sys_oss
-- ----------------------------

-- ----------------------------
-- Table structure for sys_oss_config
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_oss_config";
CREATE TABLE "public"."sys_oss_config" (
  "oss_config_id" int8 NOT NULL,
  "config_key" varchar(20) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "access_key" varchar(255) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "secret_key" varchar(255) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "bucket_name" varchar(255) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "prefix" varchar(255) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "endpoint" varchar(255) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "domain" varchar(255) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "is_https" char(1) COLLATE "pg_catalog"."default" DEFAULT 'N'::bpchar,
  "region" varchar(255) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "access_policy" char(1) COLLATE "pg_catalog"."default" NOT NULL DEFAULT '1'::bpchar,
  "status" char(1) COLLATE "pg_catalog"."default" DEFAULT '1'::bpchar,
  "ext1" varchar(255) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "create_dept" int8,
  "create_by" int8,
  "create_time" timestamp(6),
  "update_by" int8,
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default" DEFAULT ''::character varying
)
;
COMMENT ON COLUMN "public"."sys_oss_config"."oss_config_id" IS '主键';
COMMENT ON COLUMN "public"."sys_oss_config"."config_key" IS '配置key';
COMMENT ON COLUMN "public"."sys_oss_config"."access_key" IS 'accessKey';
COMMENT ON COLUMN "public"."sys_oss_config"."secret_key" IS '秘钥';
COMMENT ON COLUMN "public"."sys_oss_config"."bucket_name" IS '桶名称';
COMMENT ON COLUMN "public"."sys_oss_config"."prefix" IS '前缀';
COMMENT ON COLUMN "public"."sys_oss_config"."endpoint" IS '访问站点';
COMMENT ON COLUMN "public"."sys_oss_config"."domain" IS '自定义域名';
COMMENT ON COLUMN "public"."sys_oss_config"."is_https" IS '是否https（Y=是,N=否）';
COMMENT ON COLUMN "public"."sys_oss_config"."region" IS '域';
COMMENT ON COLUMN "public"."sys_oss_config"."access_policy" IS '桶权限类型(0=private 1=public 2=custom)';
COMMENT ON COLUMN "public"."sys_oss_config"."status" IS '是否默认（0=是,1=否）';
COMMENT ON COLUMN "public"."sys_oss_config"."ext1" IS '扩展字段';
COMMENT ON COLUMN "public"."sys_oss_config"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."sys_oss_config"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_oss_config"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_oss_config"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_oss_config"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_oss_config"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_oss_config" IS '对象存储配置表';

-- ----------------------------
-- Records of sys_oss_config
-- ----------------------------
INSERT INTO "public"."sys_oss_config" VALUES (1, 'minio', 'ruoyi', 'ruoyi123', 'ruoyi', '', '127.0.0.1:9000', '', 'N', '', '1', '0', '', 103, 1, '2026-09-30 23:07:47.248766', 1, '2026-09-30 23:07:47.248766', NULL);
INSERT INTO "public"."sys_oss_config" VALUES (2, 'qiniu', 'XXXXXXXXXXXXXXX', 'XXXXXXXXXXXXXXX', 'ruoyi', '', 's3-cn-north-1.qiniucs.com', '', 'N', '', '1', '1', '', 103, 1, '2026-09-30 23:07:47.248766', 1, '2026-09-30 23:07:47.248766', NULL);
INSERT INTO "public"."sys_oss_config" VALUES (3, 'aliyun', 'XXXXXXXXXXXXXXX', 'XXXXXXXXXXXXXXX', 'ruoyi', '', 'oss-cn-beijing.aliyuncs.com', '', 'N', '', '1', '1', '', 103, 1, '2026-09-30 23:07:47.248766', 1, '2026-09-30 23:07:47.248766', NULL);
INSERT INTO "public"."sys_oss_config" VALUES (4, 'qcloud', 'XXXXXXXXXXXXXXX', 'XXXXXXXXXXXXXXX', 'ruoyi-1240000000', '', 'cos.ap-beijing.myqcloud.com', '', 'N', 'ap-beijing', '1', '1', '', 103, 1, '2026-09-30 23:07:47.248766', 1, '2026-09-30 23:07:47.248766', NULL);
INSERT INTO "public"."sys_oss_config" VALUES (5, 'image', 'ruoyi', 'ruoyi123', 'ruoyi', 'image', '127.0.0.1:9000', '', 'N', '', '1', '1', '', 103, 1, '2026-09-30 23:07:47.248766', 1, '2026-09-30 23:07:47.248766', NULL);

-- ----------------------------
-- Table structure for sys_post
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_post";
CREATE TABLE "public"."sys_post" (
  "post_id" int8 NOT NULL,
  "dept_id" int8,
  "post_code" varchar(64) COLLATE "pg_catalog"."default" NOT NULL,
  "post_category" varchar(100) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "post_name" varchar(50) COLLATE "pg_catalog"."default" NOT NULL,
  "post_sort" int4 NOT NULL,
  "status" char(1) COLLATE "pg_catalog"."default" NOT NULL,
  "create_dept" int8,
  "create_by" int8,
  "create_time" timestamp(6),
  "update_by" int8,
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying
)
;
COMMENT ON COLUMN "public"."sys_post"."post_id" IS '岗位ID';
COMMENT ON COLUMN "public"."sys_post"."dept_id" IS '部门id';
COMMENT ON COLUMN "public"."sys_post"."post_code" IS '岗位编码';
COMMENT ON COLUMN "public"."sys_post"."post_category" IS '岗位类别编码';
COMMENT ON COLUMN "public"."sys_post"."post_name" IS '岗位名称';
COMMENT ON COLUMN "public"."sys_post"."post_sort" IS '显示顺序';
COMMENT ON COLUMN "public"."sys_post"."status" IS '状态（0正常 1停用）';
COMMENT ON COLUMN "public"."sys_post"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."sys_post"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_post"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_post"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_post"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_post"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_post" IS '岗位信息表';

-- ----------------------------
-- Records of sys_post
-- ----------------------------
INSERT INTO "public"."sys_post" VALUES (1, 103, 'ceo', NULL, '董事长', 1, '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_post" VALUES (2, 100, 'se', NULL, '项目经理', 2, '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_post" VALUES (3, 100, 'hr', NULL, '人力资源', 3, '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_post" VALUES (4, 100, 'user', NULL, '普通员工', 4, '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');

-- ----------------------------
-- Table structure for sys_role
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_role";
CREATE TABLE "public"."sys_role" (
  "role_id" int8 NOT NULL,
  "role_name" varchar(30) COLLATE "pg_catalog"."default" NOT NULL,
  "role_key" varchar(100) COLLATE "pg_catalog"."default" NOT NULL,
  "role_sort" int4 NOT NULL,
  "data_scope" char(1) COLLATE "pg_catalog"."default" DEFAULT '1'::bpchar,
  "menu_check_strictly" bool DEFAULT true,
  "dept_check_strictly" bool DEFAULT true,
  "status" char(1) COLLATE "pg_catalog"."default" NOT NULL,
  "del_flag" char(1) COLLATE "pg_catalog"."default" DEFAULT '0'::bpchar,
  "create_dept" int8,
  "create_by" int8,
  "create_time" timestamp(6),
  "update_by" int8,
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying
)
;
COMMENT ON COLUMN "public"."sys_role"."role_id" IS '角色ID';
COMMENT ON COLUMN "public"."sys_role"."role_name" IS '角色名称';
COMMENT ON COLUMN "public"."sys_role"."role_key" IS '角色权限字符串';
COMMENT ON COLUMN "public"."sys_role"."role_sort" IS '显示顺序';
COMMENT ON COLUMN "public"."sys_role"."data_scope" IS '数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限 5：仅本人数据权限 6：部门及以下或本人数据权限）';
COMMENT ON COLUMN "public"."sys_role"."menu_check_strictly" IS '菜单树选择项是否关联显示';
COMMENT ON COLUMN "public"."sys_role"."dept_check_strictly" IS '部门树选择项是否关联显示';
COMMENT ON COLUMN "public"."sys_role"."status" IS '角色状态（0正常 1停用）';
COMMENT ON COLUMN "public"."sys_role"."del_flag" IS '删除标志（0代表存在 1代表删除）';
COMMENT ON COLUMN "public"."sys_role"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."sys_role"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_role"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_role"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_role"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_role"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_role" IS '角色信息表';

-- ----------------------------
-- Records of sys_role
-- ----------------------------
INSERT INTO "public"."sys_role" VALUES (1, '超级管理员', 'superadmin', 1, '1', 't', 't', '0', '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '超级管理员');
INSERT INTO "public"."sys_role" VALUES (4, '仅本人', 'test2', 4, '5', 't', 't', '0', '0', 103, 1, '2026-09-30 23:07:47.115885', NULL, NULL, '');
INSERT INTO "public"."sys_role" VALUES (3, '本部门及以下', 'test1', 3, '4', 't', 't', '0', NULL, 103, NULL, NULL, 1, '2026-10-08 19:44:30.244312', '');

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
  "role_id" int8 NOT NULL,
  "menu_id" int8 NOT NULL
)
;
COMMENT ON COLUMN "public"."sys_role_menu"."role_id" IS '角色ID';
COMMENT ON COLUMN "public"."sys_role_menu"."menu_id" IS '菜单ID';
COMMENT ON TABLE "public"."sys_role_menu" IS '角色和菜单关联表';

-- ----------------------------
-- Records of sys_role_menu
-- ----------------------------
INSERT INTO "public"."sys_role_menu" VALUES (4, 5);
INSERT INTO "public"."sys_role_menu" VALUES (4, 1500);
INSERT INTO "public"."sys_role_menu" VALUES (4, 1501);
INSERT INTO "public"."sys_role_menu" VALUES (4, 1502);
INSERT INTO "public"."sys_role_menu" VALUES (4, 1503);
INSERT INTO "public"."sys_role_menu" VALUES (4, 1504);
INSERT INTO "public"."sys_role_menu" VALUES (4, 1505);
INSERT INTO "public"."sys_role_menu" VALUES (4, 1506);
INSERT INTO "public"."sys_role_menu" VALUES (4, 1507);
INSERT INTO "public"."sys_role_menu" VALUES (4, 1508);
INSERT INTO "public"."sys_role_menu" VALUES (4, 1509);
INSERT INTO "public"."sys_role_menu" VALUES (4, 1510);
INSERT INTO "public"."sys_role_menu" VALUES (4, 1511);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1);
INSERT INTO "public"."sys_role_menu" VALUES (3, 100);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1001);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1002);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1003);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1004);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1005);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1006);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1007);
INSERT INTO "public"."sys_role_menu" VALUES (3, 101);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1008);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1009);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1010);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1011);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1012);
INSERT INTO "public"."sys_role_menu" VALUES (3, 102);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1013);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1014);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1015);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1016);
INSERT INTO "public"."sys_role_menu" VALUES (3, 103);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1017);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1018);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1019);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1020);
INSERT INTO "public"."sys_role_menu" VALUES (3, 104);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1021);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1022);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1023);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1024);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1025);
INSERT INTO "public"."sys_role_menu" VALUES (3, 105);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1026);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1027);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1028);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1029);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1030);
INSERT INTO "public"."sys_role_menu" VALUES (3, 106);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1031);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1032);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1033);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1034);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1035);
INSERT INTO "public"."sys_role_menu" VALUES (3, 107);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1036);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1037);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1038);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1039);
INSERT INTO "public"."sys_role_menu" VALUES (3, 108);
INSERT INTO "public"."sys_role_menu" VALUES (3, 500);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1040);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1041);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1042);
INSERT INTO "public"."sys_role_menu" VALUES (3, 501);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1043);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1044);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1045);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1050);
INSERT INTO "public"."sys_role_menu" VALUES (3, 118);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1600);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1601);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1602);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1603);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1620);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1621);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1622);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1623);
INSERT INTO "public"."sys_role_menu" VALUES (3, 123);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1061);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1062);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1063);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1064);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1065);
INSERT INTO "public"."sys_role_menu" VALUES (3, 5);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1506);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1507);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1508);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1509);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1510);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1511);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1500);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1501);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1502);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1503);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1504);
INSERT INTO "public"."sys_role_menu" VALUES (3, 1505);

-- ----------------------------
-- Table structure for sys_social
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_social";
CREATE TABLE "public"."sys_social" (
  "id" int8 NOT NULL,
  "user_id" int8 NOT NULL,
  "auth_id" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "source" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "open_id" varchar(255) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "user_name" varchar(30) COLLATE "pg_catalog"."default" NOT NULL,
  "nick_name" varchar(30) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "email" varchar(255) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "avatar" varchar(500) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "access_token" varchar(2000) COLLATE "pg_catalog"."default" NOT NULL,
  "expire_in" int8,
  "refresh_token" varchar(2000) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "access_code" varchar(255) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "union_id" varchar(255) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "scope" varchar(255) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "token_type" varchar(255) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "id_token" varchar(2000) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "mac_algorithm" varchar(255) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "mac_key" varchar(255) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "code" varchar(255) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "oauth_token" varchar(255) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "oauth_token_secret" varchar(255) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying,
  "create_dept" int8,
  "create_by" int8,
  "create_time" timestamp(6),
  "update_by" int8,
  "update_time" timestamp(6),
  "del_flag" char(1) COLLATE "pg_catalog"."default" DEFAULT '0'::bpchar
)
;
COMMENT ON COLUMN "public"."sys_social"."id" IS '主键';
COMMENT ON COLUMN "public"."sys_social"."user_id" IS '用户ID';
COMMENT ON COLUMN "public"."sys_social"."auth_id" IS '平台+平台唯一id';
COMMENT ON COLUMN "public"."sys_social"."source" IS '用户来源';
COMMENT ON COLUMN "public"."sys_social"."open_id" IS '平台编号唯一id';
COMMENT ON COLUMN "public"."sys_social"."user_name" IS '登录账号';
COMMENT ON COLUMN "public"."sys_social"."nick_name" IS '用户昵称';
COMMENT ON COLUMN "public"."sys_social"."email" IS '用户邮箱';
COMMENT ON COLUMN "public"."sys_social"."avatar" IS '头像地址';
COMMENT ON COLUMN "public"."sys_social"."access_token" IS '用户的授权令牌';
COMMENT ON COLUMN "public"."sys_social"."expire_in" IS '用户的授权令牌的有效期，部分平台可能没有';
COMMENT ON COLUMN "public"."sys_social"."refresh_token" IS '刷新令牌，部分平台可能没有';
COMMENT ON COLUMN "public"."sys_social"."access_code" IS '平台的授权信息，部分平台可能没有';
COMMENT ON COLUMN "public"."sys_social"."union_id" IS '用户的 unionid';
COMMENT ON COLUMN "public"."sys_social"."scope" IS '授予的权限，部分平台可能没有';
COMMENT ON COLUMN "public"."sys_social"."token_type" IS '个别平台的授权信息，部分平台可能没有';
COMMENT ON COLUMN "public"."sys_social"."id_token" IS 'id token，部分平台可能没有';
COMMENT ON COLUMN "public"."sys_social"."mac_algorithm" IS '小米平台用户的附带属性，部分平台可能没有';
COMMENT ON COLUMN "public"."sys_social"."mac_key" IS '小米平台用户的附带属性，部分平台可能没有';
COMMENT ON COLUMN "public"."sys_social"."code" IS '用户的授权code，部分平台可能没有';
COMMENT ON COLUMN "public"."sys_social"."oauth_token" IS 'Twitter平台用户的附带属性，部分平台可能没有';
COMMENT ON COLUMN "public"."sys_social"."oauth_token_secret" IS 'Twitter平台用户的附带属性，部分平台可能没有';
COMMENT ON COLUMN "public"."sys_social"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."sys_social"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_social"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_social"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_social"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_social"."del_flag" IS '删除标志（0代表存在 1代表删除）';
COMMENT ON TABLE "public"."sys_social" IS '社会化关系表';

-- ----------------------------
-- Records of sys_social
-- ----------------------------

-- ----------------------------
-- Table structure for sys_trash
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_trash";
CREATE TABLE "public"."sys_trash" (
  "id" varchar(64) COLLATE "pg_catalog"."default" NOT NULL,
  "table_name" varchar(100) COLLATE "pg_catalog"."default",
  "data" text COLLATE "pg_catalog"."default",
  "create_time" timestamp(6)
)
;

-- ----------------------------
-- Records of sys_trash
-- ----------------------------
INSERT INTO "public"."sys_trash" VALUES ('2a8196a76d5c42909bef62b8d2bb3b29', 'demo_course', '{"course_id":32047931080707,"course_code":"CS101","course_name":"Python入门冒烟","teacher":"老杨","credit":3.5,"open_status":"0","start_date":"2026-10-01","remark":"codegen 冒烟测试","create_by":0,"create_time":"2026-10-01T06:25:10.243676","update_by":null,"update_time":null}', '2026-10-01 06:25:10.311989');
INSERT INTO "public"."sys_trash" VALUES ('20556f3ecb4c43628a880b3e116e42f6', 'demo_course', '{"course_id":32047997403139,"course_code":"CS101","course_name":"Python入门冒烟","teacher":"小云","credit":4.0,"open_status":"0","start_date":"2026-10-01","remark":"codegen 冒烟测试","create_by":0,"create_time":"2026-10-01T06:33:36.985761","update_by":0,"update_time":"2026-10-01T06:33:37.000771"}', '2026-10-01 06:33:37.039806');
INSERT INTO "public"."sys_trash" VALUES ('c75f8d30a8dd480dbbc173551a2304bc', 'demo_course', '{"course_id":32048005005315,"course_code":"CS101","course_name":"Python入门冒烟","teacher":"小云","credit":4.0,"open_status":"0","start_date":"2026-10-01","remark":"codegen 冒烟测试","create_by":0,"create_time":"2026-10-01T06:34:34.809618","update_by":0,"update_time":"2026-10-01T06:34:34.822952"}', '2026-10-01 06:34:34.86751');
INSERT INTO "public"."sys_trash" VALUES ('f23418c772d54d2d833faa52a7c0b105', 'demo_course', '{"course_id":32048815292419,"course_code":"CS101","course_name":"Python入门冒烟","teacher":"小云","credit":4.0,"open_status":"0","start_date":"2026-10-01","remark":"codegen 冒烟测试","create_by":0,"create_time":"2026-10-01T08:17:36.982221","update_by":0,"update_time":"2026-10-01T08:17:36.995550"}', '2026-10-01 08:17:37.027044');
INSERT INTO "public"."sys_trash" VALUES ('d37c137c10b7406fa31a0eff1773fab4', 'demo_course', '{"course_id":32049721131011,"course_code":"CS101","course_name":"Python入门冒烟","teacher":"小云","credit":4.0,"open_status":"0","start_date":"2026-10-01","remark":"codegen 冒烟测试","create_by":0,"create_time":"2026-10-01T10:12:47.266480","update_by":0,"update_time":"2026-10-01T10:12:47.286258"}', '2026-10-01 10:12:47.338422');
INSERT INTO "public"."sys_trash" VALUES ('406c9225320f4fdea1eb61830e31f06e', 'sys_notice', '{"notice_id":32132202774529,"notice_title":"smoke_fe_be_check","notice_content":"full stack smoke","notice_type":"1","status":"0","create_by":1,"create_time":"2026-10-08T17:00:52.327575","update_by":null,"update_time":null,"remark":null}', '2026-10-08 17:03:44.687229');

-- ----------------------------
-- Table structure for sys_user
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_user";
CREATE TABLE "public"."sys_user" (
  "user_id" int8 NOT NULL,
  "dept_id" int8,
  "user_name" varchar(30) COLLATE "pg_catalog"."default" NOT NULL,
  "nick_name" varchar(30) COLLATE "pg_catalog"."default" NOT NULL,
  "user_type" varchar(10) COLLATE "pg_catalog"."default" DEFAULT 'sys_user'::character varying,
  "email" varchar(50) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "phonenumber" varchar(11) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "sex" char(1) COLLATE "pg_catalog"."default" DEFAULT '0'::bpchar,
  "avatar" int8,
  "password" varchar(100) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "status" char(1) COLLATE "pg_catalog"."default" DEFAULT '0'::bpchar,
  "del_flag" char(1) COLLATE "pg_catalog"."default" DEFAULT '0'::bpchar,
  "login_ip" varchar(128) COLLATE "pg_catalog"."default" DEFAULT ''::character varying,
  "login_date" timestamp(6),
  "create_dept" int8,
  "create_by" int8,
  "create_time" timestamp(6),
  "update_by" int8,
  "update_time" timestamp(6),
  "remark" varchar(500) COLLATE "pg_catalog"."default" DEFAULT NULL::character varying
)
;
COMMENT ON COLUMN "public"."sys_user"."user_id" IS '用户ID';
COMMENT ON COLUMN "public"."sys_user"."dept_id" IS '部门ID';
COMMENT ON COLUMN "public"."sys_user"."user_name" IS '用户账号';
COMMENT ON COLUMN "public"."sys_user"."nick_name" IS '用户昵称';
COMMENT ON COLUMN "public"."sys_user"."user_type" IS '用户类型（sys_user系统用户）';
COMMENT ON COLUMN "public"."sys_user"."email" IS '用户邮箱';
COMMENT ON COLUMN "public"."sys_user"."phonenumber" IS '手机号码';
COMMENT ON COLUMN "public"."sys_user"."sex" IS '用户性别（0男 1女 2未知）';
COMMENT ON COLUMN "public"."sys_user"."avatar" IS '头像地址';
COMMENT ON COLUMN "public"."sys_user"."password" IS '密码';
COMMENT ON COLUMN "public"."sys_user"."status" IS '账号状态（0正常 1停用）';
COMMENT ON COLUMN "public"."sys_user"."del_flag" IS '删除标志（0代表存在 1代表删除）';
COMMENT ON COLUMN "public"."sys_user"."login_ip" IS '最后登陆IP';
COMMENT ON COLUMN "public"."sys_user"."login_date" IS '最后登陆时间';
COMMENT ON COLUMN "public"."sys_user"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."sys_user"."create_by" IS '创建者';
COMMENT ON COLUMN "public"."sys_user"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."sys_user"."update_by" IS '更新者';
COMMENT ON COLUMN "public"."sys_user"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."sys_user"."remark" IS '备注';
COMMENT ON TABLE "public"."sys_user" IS '用户信息表';

-- ----------------------------
-- Records of sys_user
-- ----------------------------
INSERT INTO "public"."sys_user" VALUES (3, 108, 'test', '本部门及以下 密码666666', 'sys_user', '', '', '0', NULL, '$2a$10$b8yUzN0C71sbz.PhNOCgJe.Tu1yWC3RNrTyjSQ8p1W0.aaUXUJ.Ne', '0', '0', '127.0.0.1', '2026-09-30 23:07:47.115885', 103, 1, '2026-09-30 23:07:47.115885', 3, '2026-09-30 23:07:47.115885', NULL);
INSERT INTO "public"."sys_user" VALUES (1, 103, 'admin', '疯狂的狮子Li', 'sys_user', 'crazyLionLi@163.com', '15888888888', '1', NULL, 'b227bff0d28823d4599a39a5b55725b0811c9c13184087e9a122eb572e6ff139', '0', '0', '127.0.0.1', '2026-09-30 23:07:47.115885', 103, 1, '2026-09-30 23:07:47.115885', NULL, '2026-10-01 00:49:57.029637', '管理员');
INSERT INTO "public"."sys_user" VALUES (4, 101, 'test1', '仅本人 密码666666', 'sys_user', '', '', '0', NULL, '$2a$10$b8yUzN0C71sbz.PhNOCgJe.Tu1yWC3RNrTyjSQ8p1W0.aaUXUJ.Ne', '0', '0', '127.0.0.1', '2026-09-30 23:07:47.115885', 103, 1, '2026-09-30 23:07:47.115885', 4, '2026-10-08 19:43:24.970833', NULL);

-- ----------------------------
-- Table structure for sys_user_post
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_user_post";
CREATE TABLE "public"."sys_user_post" (
  "user_id" int8 NOT NULL,
  "post_id" int8 NOT NULL
)
;
COMMENT ON COLUMN "public"."sys_user_post"."user_id" IS '用户ID';
COMMENT ON COLUMN "public"."sys_user_post"."post_id" IS '岗位ID';
COMMENT ON TABLE "public"."sys_user_post" IS '用户与岗位关联表';

-- ----------------------------
-- Records of sys_user_post
-- ----------------------------
INSERT INTO "public"."sys_user_post" VALUES (1, 1);

-- ----------------------------
-- Table structure for sys_user_role
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_user_role";
CREATE TABLE "public"."sys_user_role" (
  "user_id" int8 NOT NULL,
  "role_id" int8 NOT NULL
)
;
COMMENT ON COLUMN "public"."sys_user_role"."user_id" IS '用户ID';
COMMENT ON COLUMN "public"."sys_user_role"."role_id" IS '角色ID';
COMMENT ON TABLE "public"."sys_user_role" IS '用户和角色关联表';

-- ----------------------------
-- Records of sys_user_role
-- ----------------------------
INSERT INTO "public"."sys_user_role" VALUES (1, 1);
INSERT INTO "public"."sys_user_role" VALUES (3, 3);
INSERT INTO "public"."sys_user_role" VALUES (0, 0);
INSERT INTO "public"."sys_user_role" VALUES (4, 1);
INSERT INTO "public"."sys_user_role" VALUES (4, 3);
INSERT INTO "public"."sys_user_role" VALUES (4, 4);

-- ----------------------------
-- Table structure for test_demo
-- ----------------------------
DROP TABLE IF EXISTS "public"."test_demo";
CREATE TABLE "public"."test_demo" (
  "id" int8,
  "dept_id" int8,
  "user_id" int8,
  "order_num" int4 DEFAULT 0,
  "test_key" varchar(255) COLLATE "pg_catalog"."default",
  "value" varchar(255) COLLATE "pg_catalog"."default",
  "version" int4 DEFAULT 0,
  "create_dept" int8,
  "create_time" timestamp(6),
  "create_by" int8,
  "update_time" timestamp(6),
  "update_by" int8,
  "del_flag" int4 DEFAULT 0
)
;
COMMENT ON COLUMN "public"."test_demo"."id" IS '主键';
COMMENT ON COLUMN "public"."test_demo"."dept_id" IS '部门id';
COMMENT ON COLUMN "public"."test_demo"."user_id" IS '用户id';
COMMENT ON COLUMN "public"."test_demo"."order_num" IS '排序号';
COMMENT ON COLUMN "public"."test_demo"."test_key" IS 'key键';
COMMENT ON COLUMN "public"."test_demo"."value" IS '值';
COMMENT ON COLUMN "public"."test_demo"."version" IS '版本';
COMMENT ON COLUMN "public"."test_demo"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."test_demo"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."test_demo"."create_by" IS '创建人';
COMMENT ON COLUMN "public"."test_demo"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."test_demo"."update_by" IS '更新人';
COMMENT ON COLUMN "public"."test_demo"."del_flag" IS '删除标志';
COMMENT ON TABLE "public"."test_demo" IS '测试单表';

-- ----------------------------
-- Records of test_demo
-- ----------------------------
INSERT INTO "public"."test_demo" VALUES (1, 102, 4, 1, '测试数据权限', '测试', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_demo" VALUES (2, 102, 3, 2, '子节点1', '111', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_demo" VALUES (3, 102, 3, 3, '子节点2', '222', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_demo" VALUES (4, 108, 4, 4, '测试数据', 'demo', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_demo" VALUES (5, 108, 3, 13, '子节点11', '1111', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_demo" VALUES (6, 108, 3, 12, '子节点22', '2222', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_demo" VALUES (7, 108, 3, 11, '子节点33', '3333', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_demo" VALUES (8, 108, 3, 10, '子节点44', '4444', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_demo" VALUES (9, 108, 3, 9, '子节点55', '5555', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_demo" VALUES (10, 108, 3, 8, '子节点66', '6666', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_demo" VALUES (11, 108, 3, 7, '子节点77', '7777', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_demo" VALUES (12, 108, 3, 6, '子节点88', '8888', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_demo" VALUES (13, 108, 3, 5, '子节点99', '9999', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);

-- ----------------------------
-- Table structure for test_tree
-- ----------------------------
DROP TABLE IF EXISTS "public"."test_tree";
CREATE TABLE "public"."test_tree" (
  "id" int8,
  "parent_id" int8 DEFAULT 0,
  "dept_id" int8,
  "user_id" int8,
  "tree_name" varchar(255) COLLATE "pg_catalog"."default",
  "version" int4 DEFAULT 0,
  "create_dept" int8,
  "create_time" timestamp(6),
  "create_by" int8,
  "update_time" timestamp(6),
  "update_by" int8,
  "del_flag" int4 DEFAULT 0
)
;
COMMENT ON COLUMN "public"."test_tree"."id" IS '主键';
COMMENT ON COLUMN "public"."test_tree"."parent_id" IS '父id';
COMMENT ON COLUMN "public"."test_tree"."dept_id" IS '部门id';
COMMENT ON COLUMN "public"."test_tree"."user_id" IS '用户id';
COMMENT ON COLUMN "public"."test_tree"."tree_name" IS '值';
COMMENT ON COLUMN "public"."test_tree"."version" IS '版本';
COMMENT ON COLUMN "public"."test_tree"."create_dept" IS '创建部门';
COMMENT ON COLUMN "public"."test_tree"."create_time" IS '创建时间';
COMMENT ON COLUMN "public"."test_tree"."create_by" IS '创建人';
COMMENT ON COLUMN "public"."test_tree"."update_time" IS '更新时间';
COMMENT ON COLUMN "public"."test_tree"."update_by" IS '更新人';
COMMENT ON COLUMN "public"."test_tree"."del_flag" IS '删除标志';
COMMENT ON TABLE "public"."test_tree" IS '测试树表';

-- ----------------------------
-- Records of test_tree
-- ----------------------------
INSERT INTO "public"."test_tree" VALUES (1, 0, 102, 4, '测试数据权限', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_tree" VALUES (2, 1, 102, 3, '子节点1', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_tree" VALUES (3, 2, 102, 3, '子节点2', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_tree" VALUES (4, 0, 108, 4, '测试树1', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_tree" VALUES (5, 4, 108, 3, '子节点11', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_tree" VALUES (6, 4, 108, 3, '子节点22', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_tree" VALUES (7, 4, 108, 3, '子节点33', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_tree" VALUES (8, 5, 108, 3, '子节点44', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_tree" VALUES (9, 6, 108, 3, '子节点55', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_tree" VALUES (10, 7, 108, 3, '子节点66', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_tree" VALUES (11, 7, 108, 3, '子节点77', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_tree" VALUES (12, 10, 108, 3, '子节点88', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);
INSERT INTO "public"."test_tree" VALUES (13, 10, 108, 3, '子节点99', 0, 103, '2026-09-30 23:07:47.248766', 1, NULL, NULL, 0);

-- ----------------------------
-- Function structure for akeys
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."akeys"("public"."hstore");
CREATE FUNCTION "public"."akeys"("public"."hstore")
  RETURNS "pg_catalog"."_text" AS '$libdir/hstore', 'hstore_akeys'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for armor
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."armor"(bytea, _text, _text);
CREATE FUNCTION "public"."armor"(bytea, _text, _text)
  RETURNS "pg_catalog"."text" AS '$libdir/pgcrypto', 'pg_armor'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for armor
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."armor"(bytea);
CREATE FUNCTION "public"."armor"(bytea)
  RETURNS "pg_catalog"."text" AS '$libdir/pgcrypto', 'pg_armor'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for avals
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."avals"("public"."hstore");
CREATE FUNCTION "public"."avals"("public"."hstore")
  RETURNS "pg_catalog"."_text" AS '$libdir/hstore', 'hstore_avals'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for cast_varchar_to_timestamp
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."cast_varchar_to_timestamp"(varchar);
CREATE FUNCTION "public"."cast_varchar_to_timestamp"(varchar)
  RETURNS "pg_catalog"."timestamptz" AS $BODY$
select to_timestamp($1, 'yyyy-mm-dd hh24:mi:ss');
$BODY$
  LANGUAGE sql VOLATILE STRICT
  COST 100;

-- ----------------------------
-- Function structure for crypt
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."crypt"(text, text);
CREATE FUNCTION "public"."crypt"(text, text)
  RETURNS "pg_catalog"."text" AS '$libdir/pgcrypto', 'pg_crypt'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for dearmor
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."dearmor"(text);
CREATE FUNCTION "public"."dearmor"(text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pg_dearmor'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for decrypt
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."decrypt"(bytea, bytea, text);
CREATE FUNCTION "public"."decrypt"(bytea, bytea, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pg_decrypt'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for decrypt_iv
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."decrypt_iv"(bytea, bytea, bytea, text);
CREATE FUNCTION "public"."decrypt_iv"(bytea, bytea, bytea, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pg_decrypt_iv'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for defined
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."defined"("public"."hstore", text);
CREATE FUNCTION "public"."defined"("public"."hstore", text)
  RETURNS "pg_catalog"."bool" AS '$libdir/hstore', 'hstore_defined'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for delete
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."delete"("public"."hstore", text);
CREATE FUNCTION "public"."delete"("public"."hstore", text)
  RETURNS "public"."hstore" AS '$libdir/hstore', 'hstore_delete'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for delete
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."delete"("public"."hstore", "public"."hstore");
CREATE FUNCTION "public"."delete"("public"."hstore", "public"."hstore")
  RETURNS "public"."hstore" AS '$libdir/hstore', 'hstore_delete_hstore'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for delete
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."delete"("public"."hstore", _text);
CREATE FUNCTION "public"."delete"("public"."hstore", _text)
  RETURNS "public"."hstore" AS '$libdir/hstore', 'hstore_delete_array'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for digest
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."digest"(text, text);
CREATE FUNCTION "public"."digest"(text, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pg_digest'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for digest
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."digest"(bytea, text);
CREATE FUNCTION "public"."digest"(bytea, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pg_digest'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for each
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."each"("hs" "public"."hstore", OUT "key" text, OUT "value" text);
CREATE FUNCTION "public"."each"(IN "hs" "public"."hstore", OUT "key" text, OUT "value" text)
  RETURNS SETOF "pg_catalog"."record" AS '$libdir/hstore', 'hstore_each'
  LANGUAGE c IMMUTABLE STRICT
  COST 1
  ROWS 1000;

-- ----------------------------
-- Function structure for encrypt
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."encrypt"(bytea, bytea, text);
CREATE FUNCTION "public"."encrypt"(bytea, bytea, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pg_encrypt'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for encrypt_iv
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."encrypt_iv"(bytea, bytea, bytea, text);
CREATE FUNCTION "public"."encrypt_iv"(bytea, bytea, bytea, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pg_encrypt_iv'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for exist
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."exist"("public"."hstore", text);
CREATE FUNCTION "public"."exist"("public"."hstore", text)
  RETURNS "pg_catalog"."bool" AS '$libdir/hstore', 'hstore_exists'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for exists_all
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."exists_all"("public"."hstore", _text);
CREATE FUNCTION "public"."exists_all"("public"."hstore", _text)
  RETURNS "pg_catalog"."bool" AS '$libdir/hstore', 'hstore_exists_all'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for exists_any
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."exists_any"("public"."hstore", _text);
CREATE FUNCTION "public"."exists_any"("public"."hstore", _text)
  RETURNS "pg_catalog"."bool" AS '$libdir/hstore', 'hstore_exists_any'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for fetchval
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."fetchval"("public"."hstore", text);
CREATE FUNCTION "public"."fetchval"("public"."hstore", text)
  RETURNS "pg_catalog"."text" AS '$libdir/hstore', 'hstore_fetchval'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for gen_random_bytes
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."gen_random_bytes"(int4);
CREATE FUNCTION "public"."gen_random_bytes"(int4)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pg_random_bytes'
  LANGUAGE c VOLATILE STRICT
  COST 1;

-- ----------------------------
-- Function structure for gen_random_uuid
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."gen_random_uuid"();
CREATE FUNCTION "public"."gen_random_uuid"()
  RETURNS "pg_catalog"."uuid" AS '$libdir/pgcrypto', 'pg_random_uuid'
  LANGUAGE c VOLATILE
  COST 1;

-- ----------------------------
-- Function structure for gen_salt
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."gen_salt"(text);
CREATE FUNCTION "public"."gen_salt"(text)
  RETURNS "pg_catalog"."text" AS '$libdir/pgcrypto', 'pg_gen_salt'
  LANGUAGE c VOLATILE STRICT
  COST 1;

-- ----------------------------
-- Function structure for gen_salt
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."gen_salt"(text, int4);
CREATE FUNCTION "public"."gen_salt"(text, int4)
  RETURNS "pg_catalog"."text" AS '$libdir/pgcrypto', 'pg_gen_salt_rounds'
  LANGUAGE c VOLATILE STRICT
  COST 1;

-- ----------------------------
-- Function structure for ghstore_compress
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."ghstore_compress"(internal);
CREATE FUNCTION "public"."ghstore_compress"(internal)
  RETURNS "pg_catalog"."internal" AS '$libdir/hstore', 'ghstore_compress'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for ghstore_consistent
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."ghstore_consistent"(internal, "public"."hstore", int2, oid, internal);
CREATE FUNCTION "public"."ghstore_consistent"(internal, "public"."hstore", int2, oid, internal)
  RETURNS "pg_catalog"."bool" AS '$libdir/hstore', 'ghstore_consistent'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for ghstore_decompress
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."ghstore_decompress"(internal);
CREATE FUNCTION "public"."ghstore_decompress"(internal)
  RETURNS "pg_catalog"."internal" AS '$libdir/hstore', 'ghstore_decompress'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for ghstore_in
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."ghstore_in"(cstring);
CREATE FUNCTION "public"."ghstore_in"(cstring)
  RETURNS "public"."ghstore" AS '$libdir/hstore', 'ghstore_in'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for ghstore_options
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."ghstore_options"(internal);
CREATE FUNCTION "public"."ghstore_options"(internal)
  RETURNS "pg_catalog"."void" AS '$libdir/hstore', 'ghstore_options'
  LANGUAGE c IMMUTABLE
  COST 1;

-- ----------------------------
-- Function structure for ghstore_out
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."ghstore_out"("public"."ghstore");
CREATE FUNCTION "public"."ghstore_out"("public"."ghstore")
  RETURNS "pg_catalog"."cstring" AS '$libdir/hstore', 'ghstore_out'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for ghstore_penalty
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."ghstore_penalty"(internal, internal, internal);
CREATE FUNCTION "public"."ghstore_penalty"(internal, internal, internal)
  RETURNS "pg_catalog"."internal" AS '$libdir/hstore', 'ghstore_penalty'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for ghstore_picksplit
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."ghstore_picksplit"(internal, internal);
CREATE FUNCTION "public"."ghstore_picksplit"(internal, internal)
  RETURNS "pg_catalog"."internal" AS '$libdir/hstore', 'ghstore_picksplit'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for ghstore_same
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."ghstore_same"("public"."ghstore", "public"."ghstore", internal);
CREATE FUNCTION "public"."ghstore_same"("public"."ghstore", "public"."ghstore", internal)
  RETURNS "pg_catalog"."internal" AS '$libdir/hstore', 'ghstore_same'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for ghstore_union
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."ghstore_union"(internal, internal);
CREATE FUNCTION "public"."ghstore_union"(internal, internal)
  RETURNS "public"."ghstore" AS '$libdir/hstore', 'ghstore_union'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for gin_consistent_hstore
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."gin_consistent_hstore"(internal, int2, "public"."hstore", int4, internal, internal);
CREATE FUNCTION "public"."gin_consistent_hstore"(internal, int2, "public"."hstore", int4, internal, internal)
  RETURNS "pg_catalog"."bool" AS '$libdir/hstore', 'gin_consistent_hstore'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for gin_extract_hstore
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."gin_extract_hstore"("public"."hstore", internal);
CREATE FUNCTION "public"."gin_extract_hstore"("public"."hstore", internal)
  RETURNS "pg_catalog"."internal" AS '$libdir/hstore', 'gin_extract_hstore'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for gin_extract_hstore_query
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."gin_extract_hstore_query"("public"."hstore", internal, int2, internal, internal);
CREATE FUNCTION "public"."gin_extract_hstore_query"("public"."hstore", internal, int2, internal, internal)
  RETURNS "pg_catalog"."internal" AS '$libdir/hstore', 'gin_extract_hstore_query'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hmac
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hmac"(text, text, text);
CREATE FUNCTION "public"."hmac"(text, text, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pg_hmac'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hmac
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hmac"(bytea, bytea, text);
CREATE FUNCTION "public"."hmac"(bytea, bytea, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pg_hmac'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hs_concat
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hs_concat"("public"."hstore", "public"."hstore");
CREATE FUNCTION "public"."hs_concat"("public"."hstore", "public"."hstore")
  RETURNS "public"."hstore" AS '$libdir/hstore', 'hstore_concat'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hs_contained
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hs_contained"("public"."hstore", "public"."hstore");
CREATE FUNCTION "public"."hs_contained"("public"."hstore", "public"."hstore")
  RETURNS "pg_catalog"."bool" AS '$libdir/hstore', 'hstore_contained'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hs_contains
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hs_contains"("public"."hstore", "public"."hstore");
CREATE FUNCTION "public"."hs_contains"("public"."hstore", "public"."hstore")
  RETURNS "pg_catalog"."bool" AS '$libdir/hstore', 'hstore_contains'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore"(_text, _text);
CREATE FUNCTION "public"."hstore"(_text, _text)
  RETURNS "public"."hstore" AS '$libdir/hstore', 'hstore_from_arrays'
  LANGUAGE c IMMUTABLE
  COST 1;

-- ----------------------------
-- Function structure for hstore
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore"(record);
CREATE FUNCTION "public"."hstore"(record)
  RETURNS "public"."hstore" AS '$libdir/hstore', 'hstore_from_record'
  LANGUAGE c IMMUTABLE
  COST 1;

-- ----------------------------
-- Function structure for hstore
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore"(text, text);
CREATE FUNCTION "public"."hstore"(text, text)
  RETURNS "public"."hstore" AS '$libdir/hstore', 'hstore_from_text'
  LANGUAGE c IMMUTABLE
  COST 1;

-- ----------------------------
-- Function structure for hstore
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore"(_text);
CREATE FUNCTION "public"."hstore"(_text)
  RETURNS "public"."hstore" AS '$libdir/hstore', 'hstore_from_array'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_cmp
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_cmp"("public"."hstore", "public"."hstore");
CREATE FUNCTION "public"."hstore_cmp"("public"."hstore", "public"."hstore")
  RETURNS "pg_catalog"."int4" AS '$libdir/hstore', 'hstore_cmp'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_eq
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_eq"("public"."hstore", "public"."hstore");
CREATE FUNCTION "public"."hstore_eq"("public"."hstore", "public"."hstore")
  RETURNS "pg_catalog"."bool" AS '$libdir/hstore', 'hstore_eq'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_ge
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_ge"("public"."hstore", "public"."hstore");
CREATE FUNCTION "public"."hstore_ge"("public"."hstore", "public"."hstore")
  RETURNS "pg_catalog"."bool" AS '$libdir/hstore', 'hstore_ge'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_gt
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_gt"("public"."hstore", "public"."hstore");
CREATE FUNCTION "public"."hstore_gt"("public"."hstore", "public"."hstore")
  RETURNS "pg_catalog"."bool" AS '$libdir/hstore', 'hstore_gt'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_hash
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_hash"("public"."hstore");
CREATE FUNCTION "public"."hstore_hash"("public"."hstore")
  RETURNS "pg_catalog"."int4" AS '$libdir/hstore', 'hstore_hash'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_hash_extended
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_hash_extended"("public"."hstore", int8);
CREATE FUNCTION "public"."hstore_hash_extended"("public"."hstore", int8)
  RETURNS "pg_catalog"."int8" AS '$libdir/hstore', 'hstore_hash_extended'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_in
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_in"(cstring);
CREATE FUNCTION "public"."hstore_in"(cstring)
  RETURNS "public"."hstore" AS '$libdir/hstore', 'hstore_in'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_le
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_le"("public"."hstore", "public"."hstore");
CREATE FUNCTION "public"."hstore_le"("public"."hstore", "public"."hstore")
  RETURNS "pg_catalog"."bool" AS '$libdir/hstore', 'hstore_le'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_lt
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_lt"("public"."hstore", "public"."hstore");
CREATE FUNCTION "public"."hstore_lt"("public"."hstore", "public"."hstore")
  RETURNS "pg_catalog"."bool" AS '$libdir/hstore', 'hstore_lt'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_ne
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_ne"("public"."hstore", "public"."hstore");
CREATE FUNCTION "public"."hstore_ne"("public"."hstore", "public"."hstore")
  RETURNS "pg_catalog"."bool" AS '$libdir/hstore', 'hstore_ne'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_out
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_out"("public"."hstore");
CREATE FUNCTION "public"."hstore_out"("public"."hstore")
  RETURNS "pg_catalog"."cstring" AS '$libdir/hstore', 'hstore_out'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_recv
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_recv"(internal);
CREATE FUNCTION "public"."hstore_recv"(internal)
  RETURNS "public"."hstore" AS '$libdir/hstore', 'hstore_recv'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_send
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_send"("public"."hstore");
CREATE FUNCTION "public"."hstore_send"("public"."hstore")
  RETURNS "pg_catalog"."bytea" AS '$libdir/hstore', 'hstore_send'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_subscript_handler
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_subscript_handler"(internal);
CREATE FUNCTION "public"."hstore_subscript_handler"(internal)
  RETURNS "pg_catalog"."internal" AS '$libdir/hstore', 'hstore_subscript_handler'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_to_array
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_to_array"("public"."hstore");
CREATE FUNCTION "public"."hstore_to_array"("public"."hstore")
  RETURNS "pg_catalog"."_text" AS '$libdir/hstore', 'hstore_to_array'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_to_json
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_to_json"("public"."hstore");
CREATE FUNCTION "public"."hstore_to_json"("public"."hstore")
  RETURNS "pg_catalog"."json" AS '$libdir/hstore', 'hstore_to_json'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_to_json_loose
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_to_json_loose"("public"."hstore");
CREATE FUNCTION "public"."hstore_to_json_loose"("public"."hstore")
  RETURNS "pg_catalog"."json" AS '$libdir/hstore', 'hstore_to_json_loose'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_to_jsonb
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_to_jsonb"("public"."hstore");
CREATE FUNCTION "public"."hstore_to_jsonb"("public"."hstore")
  RETURNS "pg_catalog"."jsonb" AS '$libdir/hstore', 'hstore_to_jsonb'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_to_jsonb_loose
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_to_jsonb_loose"("public"."hstore");
CREATE FUNCTION "public"."hstore_to_jsonb_loose"("public"."hstore")
  RETURNS "pg_catalog"."jsonb" AS '$libdir/hstore', 'hstore_to_jsonb_loose'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_to_matrix
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_to_matrix"("public"."hstore");
CREATE FUNCTION "public"."hstore_to_matrix"("public"."hstore")
  RETURNS "pg_catalog"."_text" AS '$libdir/hstore', 'hstore_to_matrix'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for hstore_version_diag
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."hstore_version_diag"("public"."hstore");
CREATE FUNCTION "public"."hstore_version_diag"("public"."hstore")
  RETURNS "pg_catalog"."int4" AS '$libdir/hstore', 'hstore_version_diag'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for isdefined
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."isdefined"("public"."hstore", text);
CREATE FUNCTION "public"."isdefined"("public"."hstore", text)
  RETURNS "pg_catalog"."bool" AS '$libdir/hstore', 'hstore_defined'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for isexists
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."isexists"("public"."hstore", text);
CREATE FUNCTION "public"."isexists"("public"."hstore", text)
  RETURNS "pg_catalog"."bool" AS '$libdir/hstore', 'hstore_exists'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_armor_headers
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_armor_headers"(text, OUT "key" text, OUT "value" text);
CREATE FUNCTION "public"."pgp_armor_headers"(IN text, OUT "key" text, OUT "value" text)
  RETURNS SETOF "pg_catalog"."record" AS '$libdir/pgcrypto', 'pgp_armor_headers'
  LANGUAGE c IMMUTABLE STRICT
  COST 1
  ROWS 1000;

-- ----------------------------
-- Function structure for pgp_key_id
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_key_id"(bytea);
CREATE FUNCTION "public"."pgp_key_id"(bytea)
  RETURNS "pg_catalog"."text" AS '$libdir/pgcrypto', 'pgp_key_id_w'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_pub_decrypt
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_pub_decrypt"(bytea, bytea);
CREATE FUNCTION "public"."pgp_pub_decrypt"(bytea, bytea)
  RETURNS "pg_catalog"."text" AS '$libdir/pgcrypto', 'pgp_pub_decrypt_text'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_pub_decrypt
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_pub_decrypt"(bytea, bytea, text);
CREATE FUNCTION "public"."pgp_pub_decrypt"(bytea, bytea, text)
  RETURNS "pg_catalog"."text" AS '$libdir/pgcrypto', 'pgp_pub_decrypt_text'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_pub_decrypt
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_pub_decrypt"(bytea, bytea, text, text);
CREATE FUNCTION "public"."pgp_pub_decrypt"(bytea, bytea, text, text)
  RETURNS "pg_catalog"."text" AS '$libdir/pgcrypto', 'pgp_pub_decrypt_text'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_pub_decrypt_bytea
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_pub_decrypt_bytea"(bytea, bytea, text);
CREATE FUNCTION "public"."pgp_pub_decrypt_bytea"(bytea, bytea, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pgp_pub_decrypt_bytea'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_pub_decrypt_bytea
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_pub_decrypt_bytea"(bytea, bytea);
CREATE FUNCTION "public"."pgp_pub_decrypt_bytea"(bytea, bytea)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pgp_pub_decrypt_bytea'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_pub_decrypt_bytea
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_pub_decrypt_bytea"(bytea, bytea, text, text);
CREATE FUNCTION "public"."pgp_pub_decrypt_bytea"(bytea, bytea, text, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pgp_pub_decrypt_bytea'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_pub_encrypt
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_pub_encrypt"(text, bytea, text);
CREATE FUNCTION "public"."pgp_pub_encrypt"(text, bytea, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pgp_pub_encrypt_text'
  LANGUAGE c VOLATILE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_pub_encrypt
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_pub_encrypt"(text, bytea);
CREATE FUNCTION "public"."pgp_pub_encrypt"(text, bytea)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pgp_pub_encrypt_text'
  LANGUAGE c VOLATILE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_pub_encrypt_bytea
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_pub_encrypt_bytea"(bytea, bytea, text);
CREATE FUNCTION "public"."pgp_pub_encrypt_bytea"(bytea, bytea, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pgp_pub_encrypt_bytea'
  LANGUAGE c VOLATILE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_pub_encrypt_bytea
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_pub_encrypt_bytea"(bytea, bytea);
CREATE FUNCTION "public"."pgp_pub_encrypt_bytea"(bytea, bytea)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pgp_pub_encrypt_bytea'
  LANGUAGE c VOLATILE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_sym_decrypt
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_sym_decrypt"(bytea, text);
CREATE FUNCTION "public"."pgp_sym_decrypt"(bytea, text)
  RETURNS "pg_catalog"."text" AS '$libdir/pgcrypto', 'pgp_sym_decrypt_text'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_sym_decrypt
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_sym_decrypt"(bytea, text, text);
CREATE FUNCTION "public"."pgp_sym_decrypt"(bytea, text, text)
  RETURNS "pg_catalog"."text" AS '$libdir/pgcrypto', 'pgp_sym_decrypt_text'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_sym_decrypt_bytea
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_sym_decrypt_bytea"(bytea, text);
CREATE FUNCTION "public"."pgp_sym_decrypt_bytea"(bytea, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pgp_sym_decrypt_bytea'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_sym_decrypt_bytea
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_sym_decrypt_bytea"(bytea, text, text);
CREATE FUNCTION "public"."pgp_sym_decrypt_bytea"(bytea, text, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pgp_sym_decrypt_bytea'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_sym_encrypt
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_sym_encrypt"(text, text);
CREATE FUNCTION "public"."pgp_sym_encrypt"(text, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pgp_sym_encrypt_text'
  LANGUAGE c VOLATILE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_sym_encrypt
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_sym_encrypt"(text, text, text);
CREATE FUNCTION "public"."pgp_sym_encrypt"(text, text, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pgp_sym_encrypt_text'
  LANGUAGE c VOLATILE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_sym_encrypt_bytea
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_sym_encrypt_bytea"(bytea, text);
CREATE FUNCTION "public"."pgp_sym_encrypt_bytea"(bytea, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pgp_sym_encrypt_bytea'
  LANGUAGE c VOLATILE STRICT
  COST 1;

-- ----------------------------
-- Function structure for pgp_sym_encrypt_bytea
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."pgp_sym_encrypt_bytea"(bytea, text, text);
CREATE FUNCTION "public"."pgp_sym_encrypt_bytea"(bytea, text, text)
  RETURNS "pg_catalog"."bytea" AS '$libdir/pgcrypto', 'pgp_sym_encrypt_bytea'
  LANGUAGE c VOLATILE STRICT
  COST 1;

-- ----------------------------
-- Function structure for populate_record
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."populate_record"(anyelement, "public"."hstore");
CREATE FUNCTION "public"."populate_record"(anyelement, "public"."hstore")
  RETURNS "pg_catalog"."anyelement" AS '$libdir/hstore', 'hstore_populate_record'
  LANGUAGE c IMMUTABLE
  COST 1;

-- ----------------------------
-- Function structure for skeys
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."skeys"("public"."hstore");
CREATE FUNCTION "public"."skeys"("public"."hstore")
  RETURNS SETOF "pg_catalog"."text" AS '$libdir/hstore', 'hstore_skeys'
  LANGUAGE c IMMUTABLE STRICT
  COST 1
  ROWS 1000;

-- ----------------------------
-- Function structure for slice
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."slice"("public"."hstore", _text);
CREATE FUNCTION "public"."slice"("public"."hstore", _text)
  RETURNS "public"."hstore" AS '$libdir/hstore', 'hstore_slice_to_hstore'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for slice_array
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."slice_array"("public"."hstore", _text);
CREATE FUNCTION "public"."slice_array"("public"."hstore", _text)
  RETURNS "pg_catalog"."_text" AS '$libdir/hstore', 'hstore_slice_to_array'
  LANGUAGE c IMMUTABLE STRICT
  COST 1;

-- ----------------------------
-- Function structure for svals
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."svals"("public"."hstore");
CREATE FUNCTION "public"."svals"("public"."hstore")
  RETURNS SETOF "pg_catalog"."text" AS '$libdir/hstore', 'hstore_svals'
  LANGUAGE c IMMUTABLE STRICT
  COST 1
  ROWS 1000;

-- ----------------------------
-- Function structure for tconvert
-- ----------------------------
DROP FUNCTION IF EXISTS "public"."tconvert"(text, text);
CREATE FUNCTION "public"."tconvert"(text, text)
  RETURNS "public"."hstore" AS '$libdir/hstore', 'hstore_from_text'
  LANGUAGE c IMMUTABLE
  COST 1;

-- ----------------------------
-- Alter sequences owned by
-- ----------------------------
ALTER SEQUENCE "public"."casbin_rule_id_seq"
OWNED BY "public"."casbin_rule"."id";
SELECT setval('"public"."casbin_rule_id_seq"', 1, false);

-- ----------------------------
-- Alter sequences owned by
-- ----------------------------
ALTER SEQUENCE "public"."sys_job_job_id_seq"
OWNED BY "public"."sys_job"."job_id";
SELECT setval('"public"."sys_job_job_id_seq"', 8, true);

-- ----------------------------
-- Alter sequences owned by
-- ----------------------------
ALTER SEQUENCE "public"."sys_job_log_job_log_id_seq"
OWNED BY "public"."sys_job_log"."job_log_id";
SELECT setval('"public"."sys_job_log_job_log_id_seq"', 3, true);

-- ----------------------------
-- Uniques structure for table casbin_rule
-- ----------------------------
ALTER TABLE "public"."casbin_rule" ADD CONSTRAINT "unique_key_sea_orm_adapter" UNIQUE ("ptype", "v0", "v1", "v2", "v3", "v4", "v5");

-- ----------------------------
-- Primary Key structure for table casbin_rule
-- ----------------------------
ALTER TABLE "public"."casbin_rule" ADD CONSTRAINT "casbin_rule_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Primary Key structure for table demo_course
-- ----------------------------
ALTER TABLE "public"."demo_course" ADD CONSTRAINT "pk_demo_course" PRIMARY KEY ("course_id");

-- ----------------------------
-- Primary Key structure for table gen_table
-- ----------------------------
ALTER TABLE "public"."gen_table" ADD CONSTRAINT "gen_table_pk" PRIMARY KEY ("table_id");

-- ----------------------------
-- Indexes structure for table gen_table_column
-- ----------------------------
CREATE INDEX "idx_gen_table_column_table_id" ON "public"."gen_table_column" USING btree (
  "table_id" "pg_catalog"."int8_ops" ASC NULLS LAST
);

-- ----------------------------
-- Primary Key structure for table gen_table_column
-- ----------------------------
ALTER TABLE "public"."gen_table_column" ADD CONSTRAINT "gen_table_column_pk" PRIMARY KEY ("column_id");

-- ----------------------------
-- Primary Key structure for table sys_client
-- ----------------------------
ALTER TABLE "public"."sys_client" ADD CONSTRAINT "sys_client_pk" PRIMARY KEY ("id");

-- ----------------------------
-- Primary Key structure for table sys_config
-- ----------------------------
ALTER TABLE "public"."sys_config" ADD CONSTRAINT "sys_config_pk" PRIMARY KEY ("config_id");

-- ----------------------------
-- Primary Key structure for table sys_dept
-- ----------------------------
ALTER TABLE "public"."sys_dept" ADD CONSTRAINT "sys_dept_pk" PRIMARY KEY ("dept_id");

-- ----------------------------
-- Primary Key structure for table sys_dict_data
-- ----------------------------
ALTER TABLE "public"."sys_dict_data" ADD CONSTRAINT "sys_dict_data_pk" PRIMARY KEY ("dict_code");

-- ----------------------------
-- Indexes structure for table sys_dict_type
-- ----------------------------
CREATE UNIQUE INDEX "sys_dict_type_index1" ON "public"."sys_dict_type" USING btree (
  "dict_type" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
);

-- ----------------------------
-- Primary Key structure for table sys_dict_type
-- ----------------------------
ALTER TABLE "public"."sys_dict_type" ADD CONSTRAINT "sys_dict_type_pk" PRIMARY KEY ("dict_id");

-- ----------------------------
-- Primary Key structure for table sys_job
-- ----------------------------
ALTER TABLE "public"."sys_job" ADD CONSTRAINT "sys_job_pkey" PRIMARY KEY ("job_id");

-- ----------------------------
-- Primary Key structure for table sys_job_log
-- ----------------------------
ALTER TABLE "public"."sys_job_log" ADD CONSTRAINT "sys_job_log_pkey" PRIMARY KEY ("job_log_id");

-- ----------------------------
-- Indexes structure for table sys_logininfor
-- ----------------------------
CREATE INDEX "idx_sys_logininfor_lt" ON "public"."sys_logininfor" USING btree (
  "login_time" "pg_catalog"."timestamp_ops" ASC NULLS LAST
);
CREATE INDEX "idx_sys_logininfor_s" ON "public"."sys_logininfor" USING btree (
  "status" COLLATE "pg_catalog"."default" "pg_catalog"."bpchar_ops" ASC NULLS LAST
);

-- ----------------------------
-- Primary Key structure for table sys_logininfor
-- ----------------------------
ALTER TABLE "public"."sys_logininfor" ADD CONSTRAINT "sys_logininfor_pk" PRIMARY KEY ("info_id");

-- ----------------------------
-- Primary Key structure for table sys_menu
-- ----------------------------
ALTER TABLE "public"."sys_menu" ADD CONSTRAINT "sys_menu_pk" PRIMARY KEY ("menu_id");

-- ----------------------------
-- Primary Key structure for table sys_notice
-- ----------------------------
ALTER TABLE "public"."sys_notice" ADD CONSTRAINT "sys_notice_pk" PRIMARY KEY ("notice_id");

-- ----------------------------
-- Indexes structure for table sys_oper_log
-- ----------------------------
CREATE INDEX "idx_sys_oper_log_bt" ON "public"."sys_oper_log" USING btree (
  "business_type" "pg_catalog"."int4_ops" ASC NULLS LAST
);
CREATE INDEX "idx_sys_oper_log_ot" ON "public"."sys_oper_log" USING btree (
  "oper_time" "pg_catalog"."timestamp_ops" ASC NULLS LAST
);
CREATE INDEX "idx_sys_oper_log_s" ON "public"."sys_oper_log" USING btree (
  "status" "pg_catalog"."int4_ops" ASC NULLS LAST
);

-- ----------------------------
-- Primary Key structure for table sys_oper_log
-- ----------------------------
ALTER TABLE "public"."sys_oper_log" ADD CONSTRAINT "sys_oper_log_pk" PRIMARY KEY ("oper_id");

-- ----------------------------
-- Primary Key structure for table sys_oss
-- ----------------------------
ALTER TABLE "public"."sys_oss" ADD CONSTRAINT "sys_oss_pk" PRIMARY KEY ("oss_id");

-- ----------------------------
-- Primary Key structure for table sys_oss_config
-- ----------------------------
ALTER TABLE "public"."sys_oss_config" ADD CONSTRAINT "sys_oss_config_pk" PRIMARY KEY ("oss_config_id");

-- ----------------------------
-- Primary Key structure for table sys_post
-- ----------------------------
ALTER TABLE "public"."sys_post" ADD CONSTRAINT "sys_post_pk" PRIMARY KEY ("post_id");

-- ----------------------------
-- Primary Key structure for table sys_role
-- ----------------------------
ALTER TABLE "public"."sys_role" ADD CONSTRAINT "sys_role_pk" PRIMARY KEY ("role_id");

-- ----------------------------
-- Primary Key structure for table sys_role_dept
-- ----------------------------
ALTER TABLE "public"."sys_role_dept" ADD CONSTRAINT "sys_role_dept_pk" PRIMARY KEY ("role_id", "dept_id");

-- ----------------------------
-- Primary Key structure for table sys_role_menu
-- ----------------------------
ALTER TABLE "public"."sys_role_menu" ADD CONSTRAINT "sys_role_menu_pk" PRIMARY KEY ("role_id", "menu_id");

-- ----------------------------
-- Primary Key structure for table sys_social
-- ----------------------------
ALTER TABLE "public"."sys_social" ADD CONSTRAINT "pk_sys_social" PRIMARY KEY ("id");

-- ----------------------------
-- Primary Key structure for table sys_trash
-- ----------------------------
ALTER TABLE "public"."sys_trash" ADD CONSTRAINT "sys_trash_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Primary Key structure for table sys_user
-- ----------------------------
ALTER TABLE "public"."sys_user" ADD CONSTRAINT "sys_user_pk" PRIMARY KEY ("user_id");

-- ----------------------------
-- Primary Key structure for table sys_user_post
-- ----------------------------
ALTER TABLE "public"."sys_user_post" ADD CONSTRAINT "sys_user_post_pk" PRIMARY KEY ("user_id", "post_id");

-- ----------------------------
-- Primary Key structure for table sys_user_role
-- ----------------------------
ALTER TABLE "public"."sys_user_role" ADD CONSTRAINT "sys_user_role_pk" PRIMARY KEY ("user_id", "role_id");
