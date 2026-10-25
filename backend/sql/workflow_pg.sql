-- ============================================================================
-- 工作流模块初始化（PostgreSQL）
--
-- 内容：
--   1. warm-flow 引擎表（flow_definition / flow_node / flow_skip / flow_instance /
--      flow_task / flow_his_task / flow_user / flow_form）
--   2. 业务表 flow_category（流程分类）、test_leave（请假单）
--   3. 工作流菜单 + 角色授权
--
-- 说明：
--   - 与 workflow_mysql.sql 是同一份结构的双方言版本，表名/列名/长度逐列对齐，
--     改一边务必同步另一边；
--   - 脚本可重复执行（IF NOT EXISTS / WHERE NOT EXISTS）；
--   - 示例流程 leave1 不在这里插数据：后端启动时由
--     `modules::workflow::service::demo` 自动导入并发布；
--   - 末尾会清空 casbin_rule，使下次启动按最新菜单重新播种权限策略。
-- ============================================================================

-- ---------------------------------------------------------------------------
-- 0. 兼容修正：官方 DDL 把布尔型/类型列写成了 int2，
--    而 SeaORM 实体是 i32（INT4），PostgreSQL 不会隐式转换，解码会报
--    "mismatched types; Rust type Option<i32> is not compatible with SQL type INT2"。
--    表已存在时用下面的 ALTER 就地改宽，不存在则忽略。
-- ---------------------------------------------------------------------------
ALTER TABLE IF EXISTS flow_definition  ALTER COLUMN is_publish      TYPE integer,
                                       ALTER COLUMN activity_status TYPE integer;
ALTER TABLE IF EXISTS flow_node        ALTER COLUMN node_type       TYPE integer;
ALTER TABLE IF EXISTS flow_skip        ALTER COLUMN now_node_type   TYPE integer,
                                       ALTER COLUMN next_node_type  TYPE integer;
ALTER TABLE IF EXISTS flow_instance    ALTER COLUMN node_type       TYPE integer,
                                       ALTER COLUMN activity_status TYPE integer;
ALTER TABLE IF EXISTS flow_task        ALTER COLUMN node_type       TYPE integer;
ALTER TABLE IF EXISTS flow_his_task    ALTER COLUMN node_type       TYPE integer,
                                       ALTER COLUMN cooperate_type  TYPE integer;
ALTER TABLE IF EXISTS flow_form        ALTER COLUMN is_publish      TYPE integer,
                                       ALTER COLUMN form_type       TYPE integer;

-- ---------------------------------------------------------------------------
-- 1. warm-flow 引擎表
--    表名/列名与 rust-warm-flow 的 SeaORM 实体一一对应；
--    类型按实体调整：i32 => integer，String => varchar（不要用 bpchar）。
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS flow_definition
(
    id              int8         NOT NULL,
    flow_code       varchar(40)  NOT NULL,
    flow_name       varchar(100) NOT NULL,
    model_value     varchar(40)  NOT NULL DEFAULT 'CLASSICS',
    category        varchar(100) NULL,
    "version"       varchar(20)  NOT NULL,
    is_publish      integer      NOT NULL DEFAULT 0,
    form_custom     varchar(1)   NULL     DEFAULT 'N',
    form_path       varchar(100) NULL,
    activity_status integer      NOT NULL DEFAULT 1,
    listener_type   varchar(100) NULL,
    listener_path   varchar(400) NULL,
    ext             varchar(500) NULL,
    create_time     timestamp    NULL,
    create_by       varchar(64)  NULL     DEFAULT '',
    update_time     timestamp    NULL,
    update_by       varchar(64)  NULL     DEFAULT '',
    del_flag        varchar(1)   NULL     DEFAULT '0',
    tenant_id       varchar(40)  NULL,
    CONSTRAINT flow_definition_pkey PRIMARY KEY (id)
);
CREATE INDEX IF NOT EXISTS idx_definition_code ON flow_definition (flow_code);
COMMENT ON TABLE flow_definition IS '流程定义表';

CREATE TABLE IF NOT EXISTS flow_node
(
    id              int8         NOT NULL,
    node_type       integer      NOT NULL,
    definition_id   int8         NOT NULL,
    node_code       varchar(100) NOT NULL,
    node_name       varchar(100) NULL,
    permission_flag varchar(200) NULL,
    node_ratio      varchar(200) NULL,
    coordinate      varchar(100) NULL,
    any_node_skip   varchar(100) NULL,
    listener_type   varchar(100) NULL,
    listener_path   varchar(400) NULL,
    form_custom     varchar(1)   NULL DEFAULT 'N',
    form_path       varchar(100) NULL,
    "version"       varchar(20)  NOT NULL,
    create_time     timestamp    NULL,
    create_by       varchar(64)  NULL DEFAULT '',
    update_time     timestamp    NULL,
    update_by       varchar(64)  NULL DEFAULT '',
    ext             text         NULL,
    del_flag        varchar(1)   NULL DEFAULT '0',
    tenant_id       varchar(40)  NULL,
    CONSTRAINT flow_node_pkey PRIMARY KEY (id)
);
CREATE INDEX IF NOT EXISTS idx_node_definition ON flow_node (definition_id);
COMMENT ON TABLE flow_node IS '流程节点表';

CREATE TABLE IF NOT EXISTS flow_skip
(
    id             int8         NOT NULL,
    definition_id  int8         NOT NULL,
    now_node_code  varchar(100) NOT NULL,
    now_node_type  integer      NULL,
    next_node_code varchar(100) NOT NULL,
    next_node_type integer      NULL,
    skip_name      varchar(100) NULL,
    skip_type      varchar(40)  NULL,
    skip_condition varchar(200) NULL,
    coordinate     varchar(100) NULL,
    create_time    timestamp    NULL,
    create_by      varchar(64)  NULL DEFAULT '',
    update_time    timestamp    NULL,
    update_by      varchar(64)  NULL DEFAULT '',
    del_flag       varchar(1)   NULL DEFAULT '0',
    tenant_id      varchar(40)  NULL,
    CONSTRAINT flow_skip_pkey PRIMARY KEY (id)
);
CREATE INDEX IF NOT EXISTS idx_skip_definition ON flow_skip (definition_id);
COMMENT ON TABLE flow_skip IS '节点跳转关联表';

CREATE TABLE IF NOT EXISTS flow_instance
(
    id              int8         NOT NULL,
    definition_id   int8         NOT NULL,
    business_id     varchar(40)  NOT NULL,
    node_type       integer      NOT NULL,
    node_code       varchar(40)  NOT NULL,
    node_name       varchar(100) NULL,
    variable        text         NULL,
    flow_status     varchar(20)  NOT NULL,
    activity_status integer      NOT NULL DEFAULT 1,
    def_json        text         NULL,
    create_time     timestamp    NULL,
    create_by       varchar(64)  NULL DEFAULT '',
    update_time     timestamp    NULL,
    update_by       varchar(64)  NULL DEFAULT '',
    ext             varchar(500) NULL,
    del_flag        varchar(1)   NULL DEFAULT '0',
    tenant_id       varchar(40)  NULL,
    CONSTRAINT flow_instance_pkey PRIMARY KEY (id)
);
CREATE INDEX IF NOT EXISTS idx_instance_business ON flow_instance (business_id);
CREATE INDEX IF NOT EXISTS idx_instance_definition ON flow_instance (definition_id);
COMMENT ON TABLE flow_instance IS '流程实例表';

CREATE TABLE IF NOT EXISTS flow_task
(
    id            int8         NOT NULL,
    definition_id int8         NOT NULL,
    instance_id   int8         NOT NULL,
    node_code     varchar(100) NOT NULL,
    node_name     varchar(100) NULL,
    node_type     integer      NOT NULL,
    flow_status   varchar(20)  NOT NULL,
    form_custom   varchar(1)   NULL DEFAULT 'N',
    form_path     varchar(100) NULL,
    create_time   timestamp    NULL,
    create_by     varchar(64)  NULL DEFAULT '',
    update_time   timestamp    NULL,
    update_by     varchar(64)  NULL DEFAULT '',
    del_flag      varchar(1)   NULL DEFAULT '0',
    tenant_id     varchar(40)  NULL,
    CONSTRAINT flow_task_pkey PRIMARY KEY (id)
);
CREATE INDEX IF NOT EXISTS idx_task_instance ON flow_task (instance_id);
COMMENT ON TABLE flow_task IS '待办任务表';

CREATE TABLE IF NOT EXISTS flow_his_task
(
    id               int8         NOT NULL,
    definition_id    int8         NOT NULL,
    instance_id      int8         NOT NULL,
    task_id          int8         NOT NULL,
    node_code        varchar(100) NULL,
    node_name        varchar(100) NULL,
    node_type        integer      NULL,
    target_node_code varchar(200) NULL,
    target_node_name varchar(200) NULL,
    approver         varchar(40)  NULL,
    cooperate_type   integer      NOT NULL DEFAULT 0,
    collaborator     varchar(500) NULL,
    skip_type        varchar(20)  NULL,
    flow_status      varchar(20)  NOT NULL,
    form_custom      varchar(1)   NULL     DEFAULT 'N',
    form_path        varchar(100) NULL,
    ext              text         NULL,
    message          varchar(500) NULL,
    variable         text         NULL,
    create_time      timestamp    NULL,
    update_time      timestamp    NULL,
    del_flag         varchar(1)   NULL DEFAULT '0',
    tenant_id        varchar(40)  NULL,
    CONSTRAINT flow_his_task_pkey PRIMARY KEY (id)
);
CREATE INDEX IF NOT EXISTS idx_his_instance ON flow_his_task (instance_id);
CREATE INDEX IF NOT EXISTS idx_his_task ON flow_his_task (task_id);
CREATE INDEX IF NOT EXISTS idx_his_approver ON flow_his_task (approver);
COMMENT ON TABLE flow_his_task IS '历史任务记录表';

CREATE TABLE IF NOT EXISTS flow_user
(
    id           int8        NOT NULL,
    "type"       varchar(1)  NOT NULL,
    processed_by varchar(80) NULL,
    associated   int8        NOT NULL,
    create_time  timestamp   NULL,
    create_by    varchar(64) NULL DEFAULT '',
    update_time  timestamp   NULL,
    update_by    varchar(64) NULL DEFAULT '',
    del_flag     varchar(1)  NULL DEFAULT '0',
    tenant_id    varchar(40) NULL,
    CONSTRAINT flow_user_pk PRIMARY KEY (id)
);
CREATE INDEX IF NOT EXISTS user_processed_type ON flow_user (processed_by, type);
CREATE INDEX IF NOT EXISTS user_associated_idx ON flow_user (associated);
COMMENT ON TABLE flow_user IS '流程用户表';

CREATE TABLE IF NOT EXISTS flow_form
(
    id           int8         NOT NULL,
    form_code    varchar(40)  NOT NULL,
    form_name    varchar(100) NOT NULL,
    "version"    varchar(20)  NOT NULL,
    is_publish   integer      NOT NULL DEFAULT 0,
    form_type    integer      NULL     DEFAULT 0,
    form_path    varchar(100) NULL,
    form_content text         NULL,
    ext          varchar(400) NULL,
    create_time  timestamp    NULL,
    update_time  timestamp    NULL,
    del_flag     varchar(1)   NULL DEFAULT '0',
    tenant_id    varchar(40)  NULL,
    CONSTRAINT flow_form_pkey PRIMARY KEY (id)
);
COMMENT ON TABLE flow_form IS '流程表单表';

-- ---------------------------------------------------------------------------
-- 2. 业务表
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS flow_category
(
    category_id   int8        NOT NULL,
    tenant_id     varchar(20) NULL DEFAULT '000000',
    parent_id     int8        NULL DEFAULT 0,
    ancestors     varchar(500) NULL DEFAULT '',
    category_name varchar(30) NOT NULL,
    order_num     int4        NULL DEFAULT 0,
    del_flag      bpchar(1)   NULL DEFAULT '0',
    create_dept   int8        NULL,
    create_by     int8        NULL,
    create_time   timestamp   NULL,
    update_by     int8        NULL,
    update_time   timestamp   NULL,
    CONSTRAINT flow_category_pkey PRIMARY KEY (category_id)
);
COMMENT ON TABLE flow_category IS '流程分类';

INSERT INTO flow_category (category_id, parent_id, ancestors, category_name, order_num, del_flag, create_by, create_time)
VALUES (100, 0,   '0',         'OA审批',   0, '0', 1, now()),
       (101, 100, '0,100',     '假勤管理', 0, '0', 1, now()),
       (102, 100, '0,100',     '人事管理', 1, '0', 1, now()),
       (103, 101, '0,100,101', '请假',     0, '0', 1, now()),
       (104, 101, '0,100,101', '出差',     1, '0', 1, now()),
       (105, 101, '0,100,101', '加班',     2, '0', 1, now()),
       (106, 101, '0,100,101', '换班',     3, '0', 1, now()),
       (107, 101, '0,100,101', '外出',     4, '0', 1, now()),
       (108, 102, '0,100,102', '转正',     1, '0', 1, now()),
       (109, 102, '0,100,102', '离职',     2, '0', 1, now())
ON CONFLICT (category_id) DO NOTHING;

CREATE TABLE IF NOT EXISTS test_leave
(
    id          int8         NOT NULL,
    tenant_id   varchar(20)  NULL DEFAULT '000000',
    apply_code  varchar(50)  NULL,
    leave_type  varchar(255) NOT NULL,
    start_date  timestamp    NOT NULL,
    end_date    timestamp    NOT NULL,
    leave_days  int4         NOT NULL,
    remark      varchar(255) NULL,
    status      varchar(255) NULL,
    create_dept int8         NULL,
    create_by   int8         NULL,
    create_time timestamp    NULL,
    update_by   int8         NULL,
    update_time timestamp    NULL,
    CONSTRAINT test_leave_pkey PRIMARY KEY (id)
);
COMMENT ON TABLE test_leave IS '请假申请表（工作流接入示例）';

-- ---------------------------------------------------------------------------
-- 3. 菜单（menu_id 11616 起，与 Java 版 ruoyi-workflow 保持一致）
--    组件路径对应 plus-ui-5.X/src/views/workflow/**
-- ---------------------------------------------------------------------------
INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query_param, is_frame, is_cache, menu_type, visible, status, perms, icon, create_dept, create_by, create_time, update_by, update_time, remark)
SELECT 11616, '工作流', 0, 6, 'workflow', '', NULL, '1', '0', 'M', '0', '0', '', 'workflow', 103, 1, now(), NULL, NULL, '工作流目录'
WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE menu_id = 11616);

INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query_param, is_frame, is_cache, menu_type, visible, status, perms, icon, create_dept, create_by, create_time, update_by, update_time, remark)
SELECT 11617, '流程分类', 11616, 1, 'category', 'workflow/category/index', NULL, '1', '0', 'C', '0', '0', 'workflow:category:list', 'tree', 103, 1, now(), NULL, NULL, '流程分类管理'
WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE menu_id = 11617);

INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query_param, is_frame, is_cache, menu_type, visible, status, perms, icon, create_dept, create_by, create_time, update_by, update_time, remark)
SELECT 11620, '流程定义', 11616, 2, 'definition', 'workflow/definition/index', NULL, '1', '0', 'C', '0', '0', 'workflow:definition:list', 'component', 103, 1, now(), NULL, NULL, '流程定义与设计器入口'
WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE menu_id = 11620);

INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query_param, is_frame, is_cache, menu_type, visible, status, perms, icon, create_dept, create_by, create_time, update_by, update_time, remark)
SELECT 11621, '流程实例', 11616, 3, 'instance', 'workflow/instance/index', NULL, '1', '0', 'C', '0', '0', 'workflow:instance:list', 'documentation', 103, 1, now(), NULL, NULL, '流程实例查询'
WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE menu_id = 11621);

INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query_param, is_frame, is_cache, menu_type, visible, status, perms, icon, create_dept, create_by, create_time, update_by, update_time, remark)
SELECT 11622, '请假申请', 11616, 4, 'leave', 'workflow/leave/index', NULL, '1', '0', 'C', '0', '0', 'workflow:leave:list', 'edit', 103, 1, now(), NULL, NULL, '请假单（工作流示例业务）'
WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE menu_id = 11622);

INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query_param, is_frame, is_cache, menu_type, visible, status, perms, icon, create_dept, create_by, create_time, update_by, update_time, remark)
SELECT 11618, '我的任务', 11616, 5, 'task', '', NULL, '1', '0', 'M', '0', '0', '', 'checkbox', 103, 1, now(), NULL, NULL, '待办与已办'
WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE menu_id = 11618);

INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query_param, is_frame, is_cache, menu_type, visible, status, perms, icon, create_dept, create_by, create_time, update_by, update_time, remark)
SELECT 11619, '我的待办', 11618, 1, 'taskWaiting', 'workflow/task/taskWaiting', NULL, '1', '0', 'C', '0', '0', 'workflow:task:todoList', 'time', 103, 1, now(), NULL, NULL, '我的待办'
WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE menu_id = 11619);

INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query_param, is_frame, is_cache, menu_type, visible, status, perms, icon, create_dept, create_by, create_time, update_by, update_time, remark)
SELECT 11632, '我的已办', 11618, 2, 'taskFinish', 'workflow/task/taskFinish', NULL, '1', '0', 'C', '0', '0', 'workflow:task:doneList', 'time-range', 103, 1, now(), NULL, NULL, '我的已办'
WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE menu_id = 11632);

-- 设计器：隐藏菜单，由「流程定义」页面跳转；组件路径必须落在 src/views 下
INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query_param, is_frame, is_cache, menu_type, visible, status, perms, icon, create_dept, create_by, create_time, update_by, update_time, remark)
SELECT 11623, '流程设计器', 11616, 9, 'design/:id?', 'workflow/design/index', NULL, '1', '0', 'C', '1', '0', 'workflow:definition:edit', 'build', 103, 1, now(), NULL, NULL, '流程设计器（隐藏）'
WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE menu_id = 11623);

-- 通配权限：/workflow/** 全部接口一次授权。
-- casbin 的 keyMatch2 中 `*` 可跨 `/` 匹配，因此 `workflow:*` -> `/dev-api/workflow/*`
-- 能覆盖发布、办理、终止等未单独建按钮菜单的接口。
INSERT INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query_param, is_frame, is_cache, menu_type, visible, status, perms, icon, create_dept, create_by, create_time, update_by, update_time, remark)
SELECT 11624, '工作流接口权限', 11616, 99, '', '', NULL, '1', '0', 'F', '1', '0', 'workflow:*', '#', 103, 1, now(), NULL, NULL, '工作流全部接口的通配权限'
WHERE NOT EXISTS (SELECT 1 FROM sys_menu WHERE menu_id = 11624);

-- 角色授权：超管(1) / 普通角色(3) / 演示角色(4) 均可使用工作流
INSERT INTO sys_role_menu (role_id, menu_id)
SELECT r.role_id, m.menu_id
FROM sys_role r
         CROSS JOIN (VALUES (11616), (11617), (11620), (11621), (11622),
                            (11618), (11619), (11632), (11623), (11624)) AS m(menu_id)
WHERE r.role_id IN (1, 3, 4)
  AND NOT EXISTS (SELECT 1
                  FROM sys_role_menu rm
                  WHERE rm.role_id = r.role_id
                    AND rm.menu_id = m.menu_id);

-- 业务状态字典（若已存在则跳过）
INSERT INTO sys_dict_type (dict_id, dict_name, dict_type, create_dept, create_by, create_time, remark)
SELECT 11616, '流程业务状态', 'wf_business_status', 103, 1, now(), '工作流业务状态'
WHERE NOT EXISTS (SELECT 1 FROM sys_dict_type WHERE dict_type = 'wf_business_status');

INSERT INTO sys_dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, remark)
SELECT v.code, v.sort, v.label, v.value, 'wf_business_status', '', v.cls, 'N', 103, 1, now(), NULL
FROM (VALUES (11616, 1, '草稿', 'draft', 'info'),
             (11617, 2, '待审核', 'waiting', 'warning'),
             (11618, 3, '已完成', 'finish', 'success'),
             (11619, 4, '已撤销', 'cancel', 'info'),
             (11620, 5, '已退回', 'back', 'danger'),
             (11621, 6, '已终止', 'termination', 'danger'),
             (11622, 7, '已作废', 'invalid', 'info')) AS v(code, sort, label, value, cls)
WHERE NOT EXISTS (SELECT 1
                  FROM sys_dict_data
                  WHERE dict_type = 'wf_business_status'
                    AND dict_value = v.value);

-- 流程状态字典 flow_status（与 warm-flow 的 FlowStatus 常量一一对应，
-- 前端待办/已办/实例列表用它渲染状态标签）
INSERT INTO sys_dict_type (dict_id, dict_name, dict_type, create_dept, create_by, create_time, remark)
SELECT 11617, '流程状态', 'flow_status', 103, 1, now(), 'warm-flow 流程实例/任务状态'
WHERE NOT EXISTS (SELECT 1 FROM sys_dict_type WHERE dict_type = 'flow_status');

INSERT INTO sys_dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, remark)
SELECT v.code, v.sort, v.label, v.value, 'flow_status', '', v.cls, 'N', 103, 1, now(), NULL
FROM (VALUES (11630, 1, '待提交', '0', 'info'),
             (11631, 2, '审批中', '1', 'warning'),
             (11632, 3, '审批通过', '2', 'success'),
             (11633, 4, '自动通过', '3', 'success'),
             (11634, 5, '作废', '4', 'danger'),
             (11635, 6, '已撤销', '5', 'info'),
             (11636, 7, '已退回', '6', 'danger'),
             (11637, 8, '撤销中', '7', 'warning'),
             (11638, 9, '已办结', '8', 'success'),
             (11639, 10, '退回中', '9', 'warning'),
             (11640, 11, '无效', '10', 'info'),
             (11641, 12, '任务退回', '11', 'danger'),
             (11642, 13, '已重启', '12', 'primary'),
             (11643, 14, '待审批', '13', 'info')) AS v(code, sort, label, value, cls)
WHERE NOT EXISTS (SELECT 1
                  FROM sys_dict_data
                  WHERE dict_type = 'flow_status'
                    AND dict_value = v.value);

-- 请假类型字典（请假示例业务）
INSERT INTO sys_dict_type (dict_id, dict_name, dict_type, create_dept, create_by, create_time, remark)
SELECT 11618, '请假类型', 'leave_type', 103, 1, now(), '工作流示例：请假类型'
WHERE NOT EXISTS (SELECT 1 FROM sys_dict_type WHERE dict_type = 'leave_type');

INSERT INTO sys_dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, remark)
SELECT v.code, v.sort, v.label, v.value, 'leave_type', '', v.cls, 'N', 103, 1, now(), NULL
FROM (VALUES (11650, 1, '事假', '1', 'primary'),
             (11651, 2, '病假', '2', 'warning'),
             (11652, 3, '年假', '3', 'success'),
             (11653, 4, '调休', '4', 'primary'),
             (11654, 5, '婚假', '5', 'danger'),
             (11655, 6, '产假', '6', 'info')) AS v(code, sort, label, value, cls)
WHERE NOT EXISTS (SELECT 1
                  FROM sys_dict_data
                  WHERE dict_type = 'leave_type'
                    AND dict_value = v.value);

-- 权限策略重播种：casbin 只在策略表为空时按菜单生成，
-- 新增菜单后清空一次，下次启动自动重建（含刚加的 workflow:* 策略）。
DELETE FROM casbin_rule;
