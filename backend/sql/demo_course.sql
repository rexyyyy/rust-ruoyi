-- ============================================================
-- demo_course：代码生成器验证用演示表
--
-- 覆盖类型：bigint 主键 / varchar / double precision 小数 / char 状态位 / date 日期
-- / 审计四列（create_by, create_time, update_by, update_time）
--
-- 注意：小数列用 double precision 而非 numeric——backend 的 sea-orm 未启用
-- with-rust_decimal，NUMERIC 列无法解码到 f64（运行时报 mismatched types）。
-- ============================================================

DROP TABLE IF EXISTS demo_course;

CREATE TABLE demo_course (
    course_id   bigint       NOT NULL,
    course_code varchar(64)  NULL,
    course_name varchar(200) NULL,
    teacher     varchar(50)  NULL,
    credit      double precision NULL,
    open_status char(1)      NULL DEFAULT '0',
    start_date  date         NULL,
    remark      varchar(500) NULL,
    create_by   bigint       NULL,
    create_time timestamp    NULL,
    update_by   bigint       NULL,
    update_time timestamp    NULL,
    CONSTRAINT pk_demo_course PRIMARY KEY (course_id)
);

COMMENT ON TABLE demo_course IS '演示课程表';
COMMENT ON COLUMN demo_course.course_id IS '课程编号';
COMMENT ON COLUMN demo_course.course_code IS '课程编码';
COMMENT ON COLUMN demo_course.course_name IS '课程名称';
COMMENT ON COLUMN demo_course.teacher IS '授课教师';
COMMENT ON COLUMN demo_course.credit IS '学分';
COMMENT ON COLUMN demo_course.open_status IS '开课状态';
COMMENT ON COLUMN demo_course.start_date IS '开课日期';
COMMENT ON COLUMN demo_course.remark IS '备注';
COMMENT ON COLUMN demo_course.create_by IS '创建者';
COMMENT ON COLUMN demo_course.create_time IS '创建时间';
COMMENT ON COLUMN demo_course.update_by IS '更新者';
COMMENT ON COLUMN demo_course.update_time IS '更新时间';

-- 两条初始数据，便于生成完直接验证列表接口
INSERT INTO demo_course (course_id, course_code, course_name, teacher, credit, open_status, start_date, remark, create_time)
VALUES (1001, 'GIS-001', '地理信息系统原理', '老杨', 3.0, '0', DATE '2026-03-01', '演示数据一', now()),
       (1002, 'RS-002',  '遥感数字图像处理',  '小云', 2.5, '0', DATE '2026-04-15', '演示数据二', now());
