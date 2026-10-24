-- ============================================================
-- tool/gen 代码生成器配置表（PostgreSQL）
--
-- 表结构严格对齐官方 RuoYi-Vue-Plus-5.X 的
--   ref/RuoYi-Vue-Plus-5.X/RuoYi-Vue-Plus-Single-5.X/script/sql/postgres/postgres_ry_vue_5.X.sql
-- （表 18 代码生成业务表 / 表 19 代码生成业务表字段）。
--
-- 说明：
--   1) gen_table.parent_menu_id 在官方为 @TableField(exist=false) 瞬态字段，
--      源自 options JSON，并非数据库列，故本 DDL 不包含该列。
--   2) 下方「迁移段」对已有库做幂等补齐：新增缺失列、删除非官方列 parent_menu_id。
--   3) 整体可重复执行。
-- ============================================================

CREATE TABLE IF NOT EXISTS gen_table
(
    table_id          int8,
    data_name         varchar(200)  default ''::varchar,
    table_name        varchar(200)  default ''::varchar,
    table_comment     varchar(500)  default ''::varchar,
    sub_table_name    varchar(64)   default ''::varchar,
    sub_table_fk_name varchar(64)   default ''::varchar,
    class_name        varchar(100)  default ''::varchar,
    tpl_category      varchar(200)  default 'crud'::varchar,
    package_name      varchar(100)  default null::varchar,
    module_name       varchar(30)   default null::varchar,
    business_name     varchar(30)   default null::varchar,
    function_name     varchar(50)   default null::varchar,
    function_author   varchar(50)   default null::varchar,
    gen_type          char          default '0'::bpchar not null,
    gen_path          varchar(200)  default '/'::varchar,
    options           varchar(1000) default null::varchar,
    create_dept       int8,
    create_by         int8,
    create_time       timestamp,
    update_by         int8,
    update_time       timestamp,
    remark            varchar(500)  default null::varchar,
    constraint gen_table_pk primary key (table_id)
);

COMMENT ON TABLE gen_table IS '代码生成业务表';
COMMENT ON COLUMN gen_table.table_id IS '编号';
COMMENT ON COLUMN gen_table.data_name IS '数据源名称';
COMMENT ON COLUMN gen_table.table_name IS '表名称';
COMMENT ON COLUMN gen_table.table_comment IS '表描述';
COMMENT ON COLUMN gen_table.sub_table_name IS '关联子表的表名';
COMMENT ON COLUMN gen_table.sub_table_fk_name IS '子表关联的外键名';
COMMENT ON COLUMN gen_table.class_name IS '实体类名称';
COMMENT ON COLUMN gen_table.tpl_category IS '使用的模板（CRUD单表操作 TREE树表操作）';
COMMENT ON COLUMN gen_table.package_name IS '生成包路径';
COMMENT ON COLUMN gen_table.module_name IS '生成模块名';
COMMENT ON COLUMN gen_table.business_name IS '生成业务名';
COMMENT ON COLUMN gen_table.function_name IS '生成功能名';
COMMENT ON COLUMN gen_table.function_author IS '生成功能作者';
COMMENT ON COLUMN gen_table.gen_type IS '生成代码方式（0zip压缩包 1自定义路径）';
COMMENT ON COLUMN gen_table.gen_path IS '生成路径（不填默认项目路径）';
COMMENT ON COLUMN gen_table.options IS '其它生成选项';
COMMENT ON COLUMN gen_table.create_dept IS '创建部门';
COMMENT ON COLUMN gen_table.create_by IS '创建者';
COMMENT ON COLUMN gen_table.create_time IS '创建时间';
COMMENT ON COLUMN gen_table.update_by IS '更新者';
COMMENT ON COLUMN gen_table.update_time IS '更新时间';
COMMENT ON COLUMN gen_table.remark IS '备注';

CREATE TABLE IF NOT EXISTS gen_table_column
(
    column_id      int8,
    table_id       int8,
    column_name    varchar(200) default null::varchar,
    column_comment varchar(500) default null::varchar,
    column_type    varchar(100) default null::varchar,
    java_type      varchar(500) default null::varchar,
    java_field     varchar(200) default null::varchar,
    is_pk          char         default null::bpchar,
    is_increment   char         default null::bpchar,
    is_required    char         default null::bpchar,
    is_insert      char         default null::bpchar,
    is_edit        char         default null::bpchar,
    is_list        char         default null::bpchar,
    is_query       char         default null::bpchar,
    query_type     varchar(200) default 'EQ'::varchar,
    html_type      varchar(200) default null::varchar,
    dict_type      varchar(200) default ''::varchar,
    sort           int4,
    create_dept    int8,
    create_by      int8,
    create_time    timestamp,
    update_by      int8,
    update_time    timestamp,
    constraint gen_table_column_pk primary key (column_id)
);

COMMENT ON TABLE gen_table_column IS '代码生成业务表字段';
COMMENT ON COLUMN gen_table_column.column_id IS '编号';
COMMENT ON COLUMN gen_table_column.table_id IS '归属表编号';
COMMENT ON COLUMN gen_table_column.column_name IS '列名称';
COMMENT ON COLUMN gen_table_column.column_comment IS '列描述';
COMMENT ON COLUMN gen_table_column.column_type IS '列类型';
COMMENT ON COLUMN gen_table_column.java_type IS 'JAVA类型';
COMMENT ON COLUMN gen_table_column.java_field IS 'JAVA字段名';
COMMENT ON COLUMN gen_table_column.is_pk IS '是否主键（1是）';
COMMENT ON COLUMN gen_table_column.is_increment IS '是否自增（1是）';
COMMENT ON COLUMN gen_table_column.is_required IS '是否必填（1是）';
COMMENT ON COLUMN gen_table_column.is_insert IS '是否为插入字段（1是）';
COMMENT ON COLUMN gen_table_column.is_edit IS '是否编辑字段（1是）';
COMMENT ON COLUMN gen_table_column.is_list IS '是否列表字段（1是）';
COMMENT ON COLUMN gen_table_column.is_query IS '是否查询字段（1是）';
COMMENT ON COLUMN gen_table_column.query_type IS '查询方式（等于、不等于、大于、小于、范围）';
COMMENT ON COLUMN gen_table_column.html_type IS '显示类型（文本框、文本域、下拉框、复选框、单选框、日期控件）';
COMMENT ON COLUMN gen_table_column.dict_type IS '字典类型';
COMMENT ON COLUMN gen_table_column.sort IS '排序';
COMMENT ON COLUMN gen_table_column.create_dept IS '创建部门';
COMMENT ON COLUMN gen_table_column.create_by IS '创建者';
COMMENT ON COLUMN gen_table_column.create_time IS '创建时间';
COMMENT ON COLUMN gen_table_column.update_by IS '更新者';
COMMENT ON COLUMN gen_table_column.update_time IS '更新时间';

CREATE INDEX IF NOT EXISTS idx_gen_table_column_table_id ON gen_table_column (table_id);

-- ============================================================
-- 迁移段：对按旧版（含 parent_menu_id 列）已建库的实例做幂等补齐/对齐。
-- 全新库执行上面 CREATE TABLE IF NOT EXISTS 后，以下 ALTER 均为 no-op。
-- ============================================================

-- gen_table：补齐官方新增列
ALTER TABLE gen_table ADD COLUMN IF NOT EXISTS sub_table_name    varchar(64)  default ''::varchar;
ALTER TABLE gen_table ADD COLUMN IF NOT EXISTS sub_table_fk_name varchar(64)  default ''::varchar;
ALTER TABLE gen_table ADD COLUMN IF NOT EXISTS gen_type         char         default '0'::bpchar not null;
ALTER TABLE gen_table ADD COLUMN IF NOT EXISTS gen_path         varchar(200) default '/'::varchar;
ALTER TABLE gen_table ADD COLUMN IF NOT EXISTS create_dept      int8;

-- gen_table：删除非官方瞬态列 parent_menu_id（官方由 options JSON 承载）
ALTER TABLE gen_table DROP COLUMN IF EXISTS parent_menu_id;

-- gen_table_column：补齐官方新增列
ALTER TABLE gen_table_column ADD COLUMN IF NOT EXISTS java_type   varchar(500) default null::varchar;
ALTER TABLE gen_table_column ADD COLUMN IF NOT EXISTS create_dept int8;
