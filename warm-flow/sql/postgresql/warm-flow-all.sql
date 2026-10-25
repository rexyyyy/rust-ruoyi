-- Warm-Flow (rust) PostgreSQL 初始化脚本
-- 与 Java 版 sql/postgresql/postgresql-warm-flow-all.sql 表结构一致，额外补充 flow_form 与索引。
--   CREATE DATABASE warm_flow WITH ENCODING 'UTF8';

CREATE TABLE flow_definition
(
    id              int8         NOT NULL,
    flow_code       varchar(40)  NOT NULL,
    flow_name       varchar(100) NOT NULL,
    model_value     varchar(40)  NOT NULL DEFAULT 'CLASSICS',
    category        varchar(100) NULL,
    "version"       varchar(20)  NOT NULL,
    is_publish      int2         NOT NULL DEFAULT 0,
    form_custom     bpchar(1)    NULL     DEFAULT 'N',
    form_path       varchar(100) NULL,
    activity_status int2         NOT NULL DEFAULT 1,
    listener_type   varchar(100) NULL,
    listener_path   varchar(400) NULL,
    ext             varchar(500) NULL,
    create_time     timestamp    NULL,
    create_by       varchar(64)  NULL     DEFAULT '',
    update_time     timestamp    NULL,
    update_by       varchar(64)  NULL     DEFAULT '',
    del_flag        bpchar(1)    NULL     DEFAULT '0',
    tenant_id       varchar(40)  NULL,
    CONSTRAINT flow_definition_pkey PRIMARY KEY (id)
);
CREATE INDEX idx_definition_code ON flow_definition (flow_code);
COMMENT ON TABLE flow_definition IS '流程定义表';

CREATE TABLE flow_node
(
    id              int8          NOT NULL,
    node_type       int2          NOT NULL,
    definition_id   int8          NOT NULL,
    node_code       varchar(100)  NOT NULL,
    node_name       varchar(100)  NULL,
    permission_flag varchar(200)  NULL,
    node_ratio      varchar(200)  NULL,
    coordinate      varchar(100)  NULL,
    any_node_skip   varchar(100)  NULL,
    listener_type   varchar(100)  NULL,
    listener_path   varchar(400)  NULL,
    form_custom     bpchar(1)     NULL DEFAULT 'N',
    form_path       varchar(100)  NULL,
    "version"       varchar(20)   NOT NULL,
    create_time     timestamp     NULL,
    create_by       varchar(64)   NULL DEFAULT '',
    update_time     timestamp     NULL,
    update_by       varchar(64)   NULL DEFAULT '',
    ext             text          NULL,
    del_flag        bpchar(1)     NULL DEFAULT '0',
    tenant_id       varchar(40)   NULL,
    CONSTRAINT flow_node_pkey PRIMARY KEY (id)
);
CREATE INDEX idx_node_definition ON flow_node (definition_id);
COMMENT ON TABLE flow_node IS '流程节点表';

CREATE TABLE flow_skip
(
    id             int8         NOT NULL,
    definition_id  int8         NOT NULL,
    now_node_code  varchar(100) NOT NULL,
    now_node_type  int2         NULL,
    next_node_code varchar(100) NOT NULL,
    next_node_type int2         NULL,
    skip_name      varchar(100) NULL,
    skip_type      varchar(40)  NULL,
    skip_condition varchar(200) NULL,
    coordinate     varchar(100) NULL,
    create_time    timestamp    NULL,
    create_by      varchar(64)  NULL DEFAULT '',
    update_time    timestamp    NULL,
    update_by      varchar(64)  NULL DEFAULT '',
    del_flag       bpchar(1)    NULL DEFAULT '0',
    tenant_id      varchar(40)  NULL,
    CONSTRAINT flow_skip_pkey PRIMARY KEY (id)
);
CREATE INDEX idx_skip_definition ON flow_skip (definition_id);
COMMENT ON TABLE flow_skip IS '节点跳转关联表';

CREATE TABLE flow_instance
(
    id              int8         NOT NULL,
    definition_id   int8         NOT NULL,
    business_id     varchar(40)  NOT NULL,
    node_type       int2         NOT NULL,
    node_code       varchar(40)  NOT NULL,
    node_name       varchar(100) NULL,
    variable        text         NULL,
    flow_status     varchar(20)  NOT NULL,
    activity_status int2         NOT NULL DEFAULT 1,
    def_json        text         NULL,
    create_time     timestamp    NULL,
    create_by       varchar(64)  NULL DEFAULT '',
    update_time     timestamp    NULL,
    update_by       varchar(64)  NULL DEFAULT '',
    ext             varchar(500) NULL,
    del_flag        bpchar(1)    NULL DEFAULT '0',
    tenant_id       varchar(40)  NULL,
    CONSTRAINT flow_instance_pkey PRIMARY KEY (id)
);
CREATE INDEX idx_instance_business ON flow_instance (business_id);
CREATE INDEX idx_instance_definition ON flow_instance (definition_id);
COMMENT ON TABLE flow_instance IS '流程实例表';

CREATE TABLE flow_task
(
    id            int8         NOT NULL,
    definition_id int8         NOT NULL,
    instance_id   int8         NOT NULL,
    node_code     varchar(100) NOT NULL,
    node_name     varchar(100) NULL,
    node_type     int2         NOT NULL,
    flow_status   varchar(20)  NOT NULL,
    form_custom   bpchar(1)    NULL DEFAULT 'N',
    form_path     varchar(100) NULL,
    create_time   timestamp    NULL,
    create_by     varchar(64)  NULL DEFAULT '',
    update_time   timestamp    NULL,
    update_by     varchar(64)  NULL DEFAULT '',
    del_flag      bpchar(1)    NULL DEFAULT '0',
    tenant_id     varchar(40)  NULL,
    CONSTRAINT flow_task_pkey PRIMARY KEY (id)
);
CREATE INDEX idx_task_instance ON flow_task (instance_id);
COMMENT ON TABLE flow_task IS '待办任务表';

CREATE TABLE flow_his_task
(
    id               int8         NOT NULL,
    definition_id    int8         NOT NULL,
    instance_id      int8         NOT NULL,
    task_id          int8         NOT NULL,
    node_code        varchar(100) NULL,
    node_name        varchar(100) NULL,
    node_type        int2         NULL,
    target_node_code varchar(200) NULL,
    target_node_name varchar(200) NULL,
    approver         varchar(40)  NULL,
    cooperate_type   int2         NOT NULL DEFAULT 0,
    collaborator     varchar(500) NULL,
    skip_type        varchar(10)  NULL,
    flow_status      varchar(20)  NOT NULL,
    form_custom      bpchar(1)    NULL     DEFAULT 'N',
    form_path        varchar(100) NULL,
    ext              text         NULL,
    message          varchar(500) NULL,
    variable         text         NULL,
    create_time      timestamp    NULL,
    update_time      timestamp    NULL,
    del_flag         bpchar(1)    NULL     DEFAULT '0',
    tenant_id        varchar(40)  NULL,
    CONSTRAINT flow_his_task_pkey PRIMARY KEY (id)
);
CREATE INDEX idx_his_instance ON flow_his_task (instance_id);
CREATE INDEX idx_his_task ON flow_his_task (task_id);
CREATE INDEX idx_his_approver ON flow_his_task (approver);
COMMENT ON TABLE flow_his_task IS '历史任务记录表';

CREATE TABLE flow_user
(
    id           int8        NOT NULL,
    "type"       bpchar(1)   NOT NULL,
    processed_by varchar(80) NULL,
    associated   int8        NOT NULL,
    create_time  timestamp   NULL,
    create_by    varchar(64) NULL DEFAULT '',
    update_time  timestamp   NULL,
    update_by    varchar(64) NULL DEFAULT '',
    del_flag     bpchar(1)   NULL DEFAULT '0',
    tenant_id    varchar(40) NULL,
    CONSTRAINT flow_user_pk PRIMARY KEY (id)
);
CREATE INDEX user_processed_type ON flow_user (processed_by, type);
CREATE INDEX user_associated_idx ON flow_user (associated);
COMMENT ON TABLE flow_user IS '流程用户表';

CREATE TABLE flow_form
(
    id           int8         NOT NULL,
    form_code    varchar(40)  NOT NULL,
    form_name    varchar(100) NOT NULL,
    "version"    varchar(20)  NOT NULL,
    is_publish   int2         NOT NULL DEFAULT 0,
    form_type    int2         NULL     DEFAULT 0,
    form_path    varchar(100) NULL,
    form_content text         NULL,
    ext          varchar(400) NULL,
    create_time  timestamp    NULL,
    update_time  timestamp    NULL,
    del_flag     bpchar(1)    NULL     DEFAULT '0',
    tenant_id    varchar(40)  NULL,
    CONSTRAINT flow_form_pkey PRIMARY KEY (id)
);
COMMENT ON TABLE flow_form IS '流程表单表';
