-- ============================================================================
-- 工作流模块初始化（MySQL 8）
--
-- 内容：
--   1. warm-flow 引擎表（flow_definition / flow_node / flow_skip / flow_instance /
--      flow_task / flow_his_task / flow_user / flow_form）
--   2. 业务表 flow_category（流程分类）、test_leave（请假单）
--   3. 工作流菜单 + 角色授权 + 字典
--
-- 说明：
--   - 与 workflow_pg.sql 是同一份结构的双方言版本，表名/列名/长度逐列对齐，
--     改一边务必同步另一边；
--   - 脚本可重复执行（CREATE TABLE IF NOT EXISTS / INSERT IGNORE）；
--   - 列类型按 SeaORM 实体来，不用官方 DDL 的 tinyint(1)：实体是 i32，
--     见下面第 0 节的兼容修正；
--   - 示例流程 leave1 不在这里插数据：后端启动时由
--     `modules::workflow::service::demo` 自动导入并发布；
--   - 末尾会清空 casbin_rule，使下次启动按最新菜单重新播种权限策略。
--
-- 用法：mysql --default-character-set=utf8mb4 -uroot -p ruoyirust \
--           -e "source d:/rust-proj/rust-ruoyi/backend/sql/workflow_mysql.sql"
-- ============================================================================

-- ---------------------------------------------------------------------------
-- 0. 兼容修正：官方 DDL（warm-flow/sql/mysql/warm-flow-all.sql）把类型列写成了
--    tinyint(1)，而 SeaORM 实体是 i32，这里统一改宽为 int。
--    MySQL 不支持 ALTER TABLE IF EXISTS，所以按 information_schema 逐列判断后动态执行：
--    表不存在或已经是 int 时不做任何事，可重复执行。
-- ---------------------------------------------------------------------------
DROP PROCEDURE IF EXISTS wf_widen_int_columns;
DELIMITER $$
CREATE PROCEDURE wf_widen_int_columns()
BEGIN
    DECLARE v_tbl  VARCHAR(64);
    DECLARE v_col  VARCHAR(64);
    DECLARE v_null VARCHAR(3);
    DECLARE v_def  LONGTEXT;
    DECLARE v_done INT DEFAULT 0;
    DECLARE c CURSOR FOR
        SELECT table_name, column_name, is_nullable, IFNULL(column_default, '')
        FROM information_schema.columns
        WHERE table_schema = DATABASE()
          AND table_name IN ('flow_definition', 'flow_node', 'flow_skip', 'flow_instance',
                             'flow_task', 'flow_his_task', 'flow_user', 'flow_form')
          AND column_name IN ('is_publish', 'activity_status', 'node_type', 'now_node_type',
                              'next_node_type', 'cooperate_type', 'form_type')
          AND data_type <> 'int';
    DECLARE CONTINUE HANDLER FOR NOT FOUND SET v_done = 1;

    OPEN c;
    widen: LOOP
        FETCH c INTO v_tbl, v_col, v_null, v_def;
        IF v_done = 1 THEN
            LEAVE widen;
        END IF;
        SET @ddl = CONCAT('ALTER TABLE `', v_tbl, '` MODIFY COLUMN `', v_col, '` int ',
                          IF(v_null = 'YES', 'NULL', 'NOT NULL'),
                          IF(v_def = '', '', CONCAT(' DEFAULT ', QUOTE(v_def))));
        PREPARE stmt FROM @ddl;
        EXECUTE stmt;
        DEALLOCATE PREPARE stmt;
    END LOOP;
    CLOSE c;
END$$
DELIMITER ;

CALL wf_widen_int_columns();
DROP PROCEDURE wf_widen_int_columns;

-- ---------------------------------------------------------------------------
-- 1. warm-flow 引擎表
--    表名/列名与 warm-flow-orm 的 SeaORM 实体一一对应；
--    类型按实体调整：i64 => bigint，i32 => int，String => varchar/char。
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS `flow_definition`
(
    `id`              bigint       NOT NULL COMMENT '主键id',
    `flow_code`       varchar(40)  NOT NULL COMMENT '流程编码',
    `flow_name`       varchar(100) NOT NULL COMMENT '流程名称',
    `model_value`     varchar(40)  NOT NULL DEFAULT 'CLASSICS' COMMENT '设计器模型（CLASSICS经典模型 MIMIC仿钉钉模型）',
    `category`        varchar(100)          DEFAULT NULL COMMENT '流程类别',
    `version`         varchar(20)  NOT NULL COMMENT '流程版本',
    `is_publish`      int          NOT NULL DEFAULT '0' COMMENT '是否发布（0未发布 1已发布 9失效）',
    `form_custom`     char(1)               DEFAULT 'N' COMMENT '审批表单是否自定义（Y是 N否）',
    `form_path`       varchar(100)          DEFAULT NULL COMMENT '审批表单路径',
    `activity_status` int          NOT NULL DEFAULT '1' COMMENT '流程激活状态（0挂起 1激活）',
    `listener_type`   varchar(100)          DEFAULT NULL COMMENT '监听器类型',
    `listener_path`   varchar(400)          DEFAULT NULL COMMENT '监听器路径',
    `ext`             varchar(500)          DEFAULT NULL COMMENT '业务详情 存业务表对象json字符串',
    `create_time`     datetime              DEFAULT NULL COMMENT '创建时间',
    `create_by`       varchar(64)           DEFAULT '' COMMENT '创建人',
    `update_time`     datetime              DEFAULT NULL COMMENT '更新时间',
    `update_by`       varchar(64)           DEFAULT '' COMMENT '更新人',
    `del_flag`        char(1)               DEFAULT '0' COMMENT '删除标志',
    `tenant_id`       varchar(40)           DEFAULT NULL COMMENT '租户id',
    PRIMARY KEY (`id`),
    KEY `idx_definition_code` (`flow_code`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '流程定义表';

CREATE TABLE IF NOT EXISTS `flow_node`
(
    `id`              bigint       NOT NULL COMMENT '主键id',
    `node_type`       int          NOT NULL COMMENT '节点类型（0开始 1中间 2结束 3互斥网关 4并行网关 5包容网关）',
    `definition_id`   bigint       NOT NULL COMMENT '流程定义id',
    `node_code`       varchar(100) NOT NULL COMMENT '流程节点编码',
    `node_name`       varchar(100)          DEFAULT NULL COMMENT '流程节点名称',
    `permission_flag` varchar(200)          DEFAULT NULL COMMENT '权限标识（权限类型:权限标识，可以多个，用@@隔开）',
    `node_ratio`      varchar(200)          DEFAULT NULL COMMENT '流程签署比例值',
    `coordinate`      varchar(100)          DEFAULT NULL COMMENT '坐标',
    `any_node_skip`   varchar(100)          DEFAULT NULL COMMENT '任意结点跳转',
    `listener_type`   varchar(100)          DEFAULT NULL COMMENT '监听器类型',
    `listener_path`   varchar(400)          DEFAULT NULL COMMENT '监听器路径',
    `form_custom`     char(1)               DEFAULT 'N' COMMENT '审批表单是否自定义（Y是 N否）',
    `form_path`       varchar(100)          DEFAULT NULL COMMENT '审批表单路径',
    `version`         varchar(20)  NOT NULL COMMENT '版本',
    `create_time`     datetime              DEFAULT NULL COMMENT '创建时间',
    `create_by`       varchar(64)           DEFAULT '' COMMENT '创建人',
    `update_time`     datetime              DEFAULT NULL COMMENT '更新时间',
    `update_by`       varchar(64)           DEFAULT '' COMMENT '更新人',
    `ext`             text COMMENT '节点扩展属性',
    `del_flag`        char(1)               DEFAULT '0' COMMENT '删除标志',
    `tenant_id`       varchar(40)           DEFAULT NULL COMMENT '租户id',
    PRIMARY KEY (`id`),
    KEY `idx_node_definition` (`definition_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '流程节点表';

CREATE TABLE IF NOT EXISTS `flow_skip`
(
    `id`             bigint       NOT NULL COMMENT '主键id',
    `definition_id`  bigint       NOT NULL COMMENT '流程定义id',
    `now_node_code`  varchar(100) NOT NULL COMMENT '当前流程节点的编码',
    `now_node_type`  int                   DEFAULT NULL COMMENT '当前节点类型',
    `next_node_code` varchar(100) NOT NULL COMMENT '下一个流程节点的编码',
    `next_node_type` int                   DEFAULT NULL COMMENT '下一个节点类型',
    `skip_name`      varchar(100)          DEFAULT NULL COMMENT '跳转名称',
    `skip_type`      varchar(40)           DEFAULT NULL COMMENT '跳转类型（PASS审批通过 REJECT退回）',
    `skip_condition` varchar(200)          DEFAULT NULL COMMENT '跳转条件（操作符@@变量名|值）',
    `coordinate`     varchar(100)          DEFAULT NULL COMMENT '坐标',
    `create_time`    datetime              DEFAULT NULL COMMENT '创建时间',
    `create_by`      varchar(64)           DEFAULT '' COMMENT '创建人',
    `update_time`    datetime              DEFAULT NULL COMMENT '更新时间',
    `update_by`      varchar(64)           DEFAULT '' COMMENT '更新人',
    `del_flag`       char(1)               DEFAULT '0' COMMENT '删除标志',
    `tenant_id`      varchar(40)           DEFAULT NULL COMMENT '租户id',
    PRIMARY KEY (`id`),
    KEY `idx_skip_definition` (`definition_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '节点跳转关联表';

CREATE TABLE IF NOT EXISTS `flow_instance`
(
    `id`              bigint       NOT NULL COMMENT '主键id',
    `definition_id`   bigint       NOT NULL COMMENT '对应flow_definition表的id',
    `business_id`     varchar(40)  NOT NULL COMMENT '业务id',
    `node_type`       int          NOT NULL COMMENT '节点类型',
    `node_code`       varchar(40)  NOT NULL COMMENT '流程节点编码',
    `node_name`       varchar(100)          DEFAULT NULL COMMENT '流程节点名称',
    `variable`        text COMMENT '任务变量',
    `flow_status`     varchar(20)  NOT NULL COMMENT '流程状态（见 flow_status 字典）',
    `activity_status` int          NOT NULL DEFAULT '1' COMMENT '流程激活状态（0挂起 1激活）',
    `def_json`        text COMMENT '流程定义json',
    `create_time`     datetime              DEFAULT NULL COMMENT '创建时间',
    `create_by`       varchar(64)           DEFAULT '' COMMENT '创建人',
    `update_time`     datetime              DEFAULT NULL COMMENT '更新时间',
    `update_by`       varchar(64)           DEFAULT '' COMMENT '更新人',
    `ext`             varchar(500)          DEFAULT NULL COMMENT '扩展字段，预留给业务系统使用',
    `del_flag`        char(1)               DEFAULT '0' COMMENT '删除标志',
    `tenant_id`       varchar(40)           DEFAULT NULL COMMENT '租户id',
    PRIMARY KEY (`id`),
    KEY `idx_instance_business` (`business_id`),
    KEY `idx_instance_definition` (`definition_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '流程实例表';

CREATE TABLE IF NOT EXISTS `flow_task`
(
    `id`            bigint       NOT NULL COMMENT '主键id',
    `definition_id` bigint       NOT NULL COMMENT '对应flow_definition表的id',
    `instance_id`   bigint       NOT NULL COMMENT '对应flow_instance表的id',
    `node_code`     varchar(100) NOT NULL COMMENT '节点编码',
    `node_name`     varchar(100)          DEFAULT NULL COMMENT '节点名称',
    `node_type`     int          NOT NULL COMMENT '节点类型',
    `flow_status`   varchar(20)  NOT NULL COMMENT '流程状态',
    `form_custom`   char(1)               DEFAULT 'N' COMMENT '审批表单是否自定义（Y是 N否）',
    `form_path`     varchar(100)          DEFAULT NULL COMMENT '审批表单路径',
    `create_time`   datetime              DEFAULT NULL COMMENT '创建时间',
    `create_by`     varchar(64)           DEFAULT '' COMMENT '创建人',
    `update_time`   datetime              DEFAULT NULL COMMENT '更新时间',
    `update_by`     varchar(64)           DEFAULT '' COMMENT '更新人',
    `del_flag`      char(1)               DEFAULT '0' COMMENT '删除标志',
    `tenant_id`     varchar(40)           DEFAULT NULL COMMENT '租户id',
    PRIMARY KEY (`id`),
    KEY `idx_task_instance` (`instance_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '待办任务表';

CREATE TABLE IF NOT EXISTS `flow_his_task`
(
    `id`               bigint       NOT NULL COMMENT '主键id',
    `definition_id`    bigint       NOT NULL COMMENT '对应flow_definition表的id',
    `instance_id`      bigint       NOT NULL COMMENT '对应flow_instance表的id',
    `task_id`          bigint       NOT NULL COMMENT '对应flow_task表的id',
    `node_code`        varchar(100)          DEFAULT NULL COMMENT '开始节点编码',
    `node_name`        varchar(100)          DEFAULT NULL COMMENT '开始节点名称',
    `node_type`        int                   DEFAULT NULL COMMENT '开始节点类型',
    `target_node_code` varchar(200)          DEFAULT NULL COMMENT '目标节点编码',
    `target_node_name` varchar(200)          DEFAULT NULL COMMENT '结束节点名称',
    `approver`         varchar(40)           DEFAULT NULL COMMENT '审批人（用户id，列表显示昵称由后端补齐）',
    `cooperate_type`   int          NOT NULL DEFAULT '0' COMMENT '协作方式(1审批 2转办 3委派 4会签 5票签 6加签 7减签)',
    `collaborator`     varchar(500)          DEFAULT NULL COMMENT '协作人',
    `skip_type`        varchar(20)           DEFAULT NULL COMMENT '流转类型（PASS通过 REJECT退回 NONE无动作）',
    `flow_status`      varchar(20)  NOT NULL COMMENT '流程状态',
    `form_custom`      char(1)               DEFAULT 'N' COMMENT '审批表单是否自定义（Y是 N否）',
    `form_path`        varchar(100)          DEFAULT NULL COMMENT '审批表单路径',
    `ext`              text COMMENT '业务详情 存业务表对象json字符串',
    `message`          varchar(500)          DEFAULT NULL COMMENT '审批意见',
    `variable`         text COMMENT '任务变量',
    `create_time`      datetime              DEFAULT NULL COMMENT '任务开始时间',
    `update_time`      datetime              DEFAULT NULL COMMENT '审批完成时间',
    `del_flag`         char(1)               DEFAULT '0' COMMENT '删除标志',
    `tenant_id`        varchar(40)           DEFAULT NULL COMMENT '租户id',
    PRIMARY KEY (`id`),
    KEY `idx_his_instance` (`instance_id`),
    KEY `idx_his_task` (`task_id`),
    KEY `idx_his_approver` (`approver`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '历史任务记录表';

CREATE TABLE IF NOT EXISTS `flow_user`
(
    `id`           bigint      NOT NULL COMMENT '主键id',
    `type`         char(1)     NOT NULL COMMENT '人员类型（1待办任务的审批人权限 2转办人权限 3委托人权限）',
    `processed_by` varchar(80)          DEFAULT NULL COMMENT '权限人',
    `associated`   bigint      NOT NULL COMMENT '任务表id',
    `create_time`  datetime             DEFAULT NULL COMMENT '创建时间',
    `create_by`    varchar(64)          DEFAULT '' COMMENT '创建人',
    `update_time`  datetime             DEFAULT NULL COMMENT '更新时间',
    `update_by`    varchar(64)          DEFAULT '' COMMENT '更新人',
    `del_flag`     char(1)              DEFAULT '0' COMMENT '删除标志',
    `tenant_id`    varchar(40)          DEFAULT NULL COMMENT '租户id',
    PRIMARY KEY (`id`),
    KEY `user_processed_type` (`processed_by`, `type`),
    KEY `user_associated_idx` (`associated`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '流程用户表';

CREATE TABLE IF NOT EXISTS `flow_form`
(
    `id`           bigint       NOT NULL COMMENT '主键id',
    `form_code`    varchar(40)  NOT NULL COMMENT '表单编码',
    `form_name`    varchar(100) NOT NULL COMMENT '表单名称',
    `version`      varchar(20)  NOT NULL COMMENT '表单版本',
    `is_publish`   int          NOT NULL DEFAULT '0' COMMENT '是否发布（0未发布 1已发布 9失效）',
    `form_type`    int                   DEFAULT '0' COMMENT '表单类型（0内置表单 存 form_content 1外挂表单 存 form_path）',
    `form_path`    varchar(100)          DEFAULT NULL COMMENT '表单路径',
    `form_content` text COMMENT '表单内容',
    `ext`          varchar(400)          DEFAULT NULL COMMENT '表单扩展，用户自行使用',
    `create_time`  datetime              DEFAULT NULL COMMENT '创建时间',
    `update_time`  datetime              DEFAULT NULL COMMENT '更新时间',
    `del_flag`     char(1)               DEFAULT '0' COMMENT '删除标志',
    `tenant_id`    varchar(40)           DEFAULT NULL COMMENT '租户id',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '流程表单表';

-- ---------------------------------------------------------------------------
-- 2. 业务表
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS `flow_category`
(
    `category_id`   bigint      NOT NULL COMMENT '流程分类id',
    `tenant_id`     varchar(20)          DEFAULT '000000' COMMENT '租户编号',
    `parent_id`     bigint               DEFAULT '0' COMMENT '父流程分类id',
    `ancestors`     varchar(500)         DEFAULT '' COMMENT '祖先流程分类id列表',
    `category_name` varchar(30) NOT NULL COMMENT '流程分类名称',
    `order_num`     int                  DEFAULT '0' COMMENT '显示顺序',
    `del_flag`      char(1)              DEFAULT '0' COMMENT '删除标志（0存在 2删除）',
    `create_dept`   bigint               DEFAULT NULL COMMENT '创建部门',
    `create_by`     bigint               DEFAULT NULL COMMENT '创建者',
    `create_time`   datetime             DEFAULT NULL COMMENT '创建时间',
    `update_by`     bigint               DEFAULT NULL COMMENT '更新者',
    `update_time`   datetime             DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`category_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '流程分类';

INSERT IGNORE INTO flow_category (category_id, parent_id, ancestors, category_name, order_num, del_flag, create_by, create_time)
VALUES (100, 0, '0', 'OA审批', 0, '0', 1, now()),
       (101, 100, '0,100', '假勤管理', 0, '0', 1, now()),
       (102, 100, '0,100', '人事管理', 1, '0', 1, now()),
       (103, 101, '0,100,101', '请假', 0, '0', 1, now()),
       (104, 101, '0,100,101', '出差', 1, '0', 1, now()),
       (105, 101, '0,100,101', '加班', 2, '0', 1, now()),
       (106, 101, '0,100,101', '换班', 3, '0', 1, now()),
       (107, 101, '0,100,101', '外出', 4, '0', 1, now()),
       (108, 102, '0,100,102', '转正', 1, '0', 1, now()),
       (109, 102, '0,100,102', '离职', 2, '0', 1, now());

CREATE TABLE IF NOT EXISTS `test_leave`
(
    `id`          bigint       NOT NULL COMMENT '请假id',
    `tenant_id`   varchar(20)           DEFAULT '000000' COMMENT '租户编号',
    `apply_code`  varchar(50)           DEFAULT NULL COMMENT '申请编号（流程开启后回填）',
    `leave_type`  varchar(255) NOT NULL COMMENT '请假类型（字典 leave_type）',
    `start_date`  datetime     NOT NULL COMMENT '开始时间',
    `end_date`    datetime     NOT NULL COMMENT '结束时间',
    `leave_days`  int          NOT NULL COMMENT '请假天数',
    `remark`      varchar(255)          DEFAULT NULL COMMENT '请假原因',
    `status`      varchar(255)          DEFAULT NULL COMMENT '业务状态（字典 wf_business_status）',
    `create_dept` bigint                DEFAULT NULL COMMENT '创建部门',
    `create_by`   bigint                DEFAULT NULL COMMENT '创建者',
    `create_time` datetime              DEFAULT NULL COMMENT '创建时间',
    `update_by`   bigint                DEFAULT NULL COMMENT '更新者',
    `update_time` datetime              DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '请假申请表（工作流接入示例）';

-- ---------------------------------------------------------------------------
-- 3. 菜单（menu_id 11616 起，与 Java 版 ruoyi-workflow 保持一致）
--    组件路径对应 plus-ui-5.X/src/views/workflow/**
-- ---------------------------------------------------------------------------
INSERT IGNORE INTO sys_menu (menu_id, menu_name, parent_id, order_num, path, component, query_param, is_frame, is_cache, menu_type, visible, status, perms, icon, create_dept, create_by, create_time, update_by, update_time, remark)
VALUES (11616, '工作流', 0, 6, 'workflow', '', NULL, '1', '0', 'M', '0', '0', '', 'workflow', 103, 1, now(), NULL, NULL, '工作流目录'),
       (11617, '流程分类', 11616, 1, 'category', 'workflow/category/index', NULL, '1', '0', 'C', '0', '0', 'workflow:category:list', 'tree', 103, 1, now(), NULL, NULL, '流程分类管理'),
       (11620, '流程定义', 11616, 2, 'definition', 'workflow/definition/index', NULL, '1', '0', 'C', '0', '0', 'workflow:definition:list', 'component', 103, 1, now(), NULL, NULL, '流程定义与设计器入口'),
       (11621, '流程实例', 11616, 3, 'instance', 'workflow/instance/index', NULL, '1', '0', 'C', '0', '0', 'workflow:instance:list', 'documentation', 103, 1, now(), NULL, NULL, '流程实例查询'),
       (11622, '请假申请', 11616, 4, 'leave', 'workflow/leave/index', NULL, '1', '0', 'C', '0', '0', 'workflow:leave:list', 'edit', 103, 1, now(), NULL, NULL, '请假单（工作流示例业务）'),
       (11618, '我的任务', 11616, 5, 'task', '', NULL, '1', '0', 'M', '0', '0', '', 'checkbox', 103, 1, now(), NULL, NULL, '待办与已办'),
       (11619, '我的待办', 11618, 1, 'taskWaiting', 'workflow/task/taskWaiting', NULL, '1', '0', 'C', '0', '0', 'workflow:task:todoList', 'time', 103, 1, now(), NULL, NULL, '我的待办'),
       (11632, '我的已办', 11618, 2, 'taskFinish', 'workflow/task/taskFinish', NULL, '1', '0', 'C', '0', '0', 'workflow:task:doneList', 'time-range', 103, 1, now(), NULL, NULL, '我的已办'),
       -- 流程设计器：隐藏菜单，由「流程定义」页面跳转；组件路径必须落在 src/views 下
       (11623, '流程设计器', 11616, 9, 'design/:id?', 'workflow/design/index', NULL, '1', '0', 'C', '1', '0', 'workflow:definition:edit', 'build', 103, 1, now(), NULL, NULL, '流程设计器（隐藏）'),
       -- 通配权限：/workflow/** 全部接口一次授权。
       -- casbin 的 keyMatch2 中 `*` 可跨 `/` 匹配，因此 `workflow:*` -> `/dev-api/workflow/*`
       -- 能覆盖发布、办理、终止等未单独建按钮菜单的接口。
       (11624, '工作流接口权限', 11616, 99, '', '', NULL, '1', '0', 'F', '1', '0', 'workflow:*', '#', 103, 1, now(), NULL, NULL, '工作流全部接口的通配权限');

-- 角色授权：超管(1) / 普通角色(3) / 演示角色(4) 均可使用工作流
INSERT IGNORE INTO sys_role_menu (role_id, menu_id)
SELECT r.role_id, m.menu_id
FROM sys_role r
         CROSS JOIN (SELECT 11616 AS menu_id
                     UNION ALL SELECT 11617
                     UNION ALL SELECT 11620
                     UNION ALL SELECT 11621
                     UNION ALL SELECT 11622
                     UNION ALL SELECT 11618
                     UNION ALL SELECT 11619
                     UNION ALL SELECT 11632
                     UNION ALL SELECT 11623
                     UNION ALL SELECT 11624) m
WHERE r.role_id IN (1, 3, 4);

-- ---------------------------------------------------------------------------
-- 4. 字典
-- ---------------------------------------------------------------------------
INSERT IGNORE INTO sys_dict_type (dict_id, dict_name, dict_type, create_dept, create_by, create_time, remark)
VALUES (11616, '流程业务状态', 'wf_business_status', 103, 1, now(), '工作流业务状态'),
       (11617, '流程状态', 'flow_status', 103, 1, now(), 'warm-flow 流程实例/任务状态'),
       (11618, '请假类型', 'leave_type', 103, 1, now(), '工作流示例：请假类型');

-- 业务状态字典
INSERT IGNORE INTO sys_dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, remark)
VALUES (11616, 1, '草稿', 'draft', 'wf_business_status', '', 'info', 'N', 103, 1, now(), NULL),
       (11617, 2, '待审核', 'waiting', 'wf_business_status', '', 'warning', 'N', 103, 1, now(), NULL),
       (11618, 3, '已完成', 'finish', 'wf_business_status', '', 'success', 'N', 103, 1, now(), NULL),
       (11619, 4, '已撤销', 'cancel', 'wf_business_status', '', 'info', 'N', 103, 1, now(), NULL),
       (11620, 5, '已退回', 'back', 'wf_business_status', '', 'danger', 'N', 103, 1, now(), NULL),
       (11621, 6, '已终止', 'termination', 'wf_business_status', '', 'danger', 'N', 103, 1, now(), NULL),
       (11622, 7, '已作废', 'invalid', 'wf_business_status', '', 'info', 'N', 103, 1, now(), NULL);

-- 流程状态字典 flow_status（与 warm-flow 的 FlowStatus 常量一一对应，
-- 前端待办/已办/实例列表用它渲染状态标签）
INSERT IGNORE INTO sys_dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, remark)
VALUES (11630, 1, '待提交', '0', 'flow_status', '', 'info', 'N', 103, 1, now(), NULL),
       (11631, 2, '审批中', '1', 'flow_status', '', 'warning', 'N', 103, 1, now(), NULL),
       (11632, 3, '审批通过', '2', 'flow_status', '', 'success', 'N', 103, 1, now(), NULL),
       (11633, 4, '自动通过', '3', 'flow_status', '', 'success', 'N', 103, 1, now(), NULL),
       (11634, 5, '作废', '4', 'flow_status', '', 'danger', 'N', 103, 1, now(), NULL),
       (11635, 6, '已撤销', '5', 'flow_status', '', 'info', 'N', 103, 1, now(), NULL),
       (11636, 7, '已退回', '6', 'flow_status', '', 'danger', 'N', 103, 1, now(), NULL),
       (11637, 8, '撤销中', '7', 'flow_status', '', 'warning', 'N', 103, 1, now(), NULL),
       (11638, 9, '已办结', '8', 'flow_status', '', 'success', 'N', 103, 1, now(), NULL),
       (11639, 10, '退回中', '9', 'flow_status', '', 'warning', 'N', 103, 1, now(), NULL),
       (11640, 11, '无效', '10', 'flow_status', '', 'info', 'N', 103, 1, now(), NULL),
       (11641, 12, '任务退回', '11', 'flow_status', '', 'danger', 'N', 103, 1, now(), NULL),
       (11642, 13, '已重启', '12', 'flow_status', '', 'primary', 'N', 103, 1, now(), NULL),
       (11643, 14, '待审批', '13', 'flow_status', '', 'info', 'N', 103, 1, now(), NULL);

-- 请假类型字典（请假示例业务）
INSERT IGNORE INTO sys_dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, remark)
VALUES (11650, 1, '事假', '1', 'leave_type', '', 'primary', 'N', 103, 1, now(), NULL),
       (11651, 2, '病假', '2', 'leave_type', '', 'warning', 'N', 103, 1, now(), NULL),
       (11652, 3, '年假', '3', 'leave_type', '', 'success', 'N', 103, 1, now(), NULL),
       (11653, 4, '调休', '4', 'leave_type', '', 'primary', 'N', 103, 1, now(), NULL),
       (11654, 5, '婚假', '5', 'leave_type', '', 'danger', 'N', 103, 1, now(), NULL),
       (11655, 6, '产假', '6', 'leave_type', '', 'info', 'N', 103, 1, now(), NULL);

-- 权限策略重播种：casbin 只在策略表为空时按菜单生成，
-- 新增菜单后清空一次，下次启动自动重建（含刚加的 workflow:* 策略）。
DELETE FROM casbin_rule;
