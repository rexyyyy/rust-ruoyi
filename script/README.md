# script —— 数据库初始化脚本

本目录存放后端建库/初始化用的 SQL。**后端同时支持 PostgreSQL 与 MySQL**，靠连接串前缀（`postgres://` / `mysql://`）自动选择方言，无需改代码。

## 文件说明

| 文件 | 方言 | 表数 | 用途 |
|---|---|---|---|
| `ry_vue_5.X.sql` | **MySQL** | 25 | **建 MySQL 库用这个**（上游 23 + `demo_course` + `sys_trash`） |
| `postgres_ry_vue_5.X.sql` | PostgreSQL | 23 | 建 PG 库用（仅上游 23 表，无项目扩展表） |
| `postgresql-5.x-public.sql` | PostgreSQL | 28 | 本项目真库 `ruoyirust`（PG16:5434）的 Navicat dump，**仅作结构参考，不用于建库** |

### 表的来源
- **上游 23 表**：`sys_user` / `sys_role` / `sys_menu` / `sys_dept` / `sys_post` / `sys_dict_*` / `sys_config` / `sys_notice` / `sys_oss*` / `sys_client` / `gen_table*` / `test_demo` / `test_tree` 等，源自 RuoYi-Vue-Plus 5.x。
- **项目扩展 5 表**：`casbin_rule`（RBAC 策略）、`sys_job` / `sys_job_log`（定时任务）、`demo_course`（codegen 演示）、`sys_trash`（回收站）。

---

## 建库：MySQL

```bash
mysql> CREATE DATABASE ruoyirust DEFAULT CHARSET utf8mb4 COLLATE utf8mb4_general_ci;
mysql> USE ruoyirust;
mysql> SOURCE D:/rust-proj/rust-ruoyi/script/ry_vue_5.X.sql;
```

导入后共 **25 张表**；再启动后端，会**自动补建 3 张**（见下），最终 **28 张**，与 PG 真库表数一致。

## 建库：PostgreSQL

```bash
createdb -E UTF8 ruoyirust
psql -d ruoyirust -f D:/rust-proj/rust-ruoyi/script/postgres_ry_vue_5.X.sql
```

PG 版 `postgres_ry_vue_5.X.sql` 只含上游 23 表；扩展 5 表同样由后端自动补建（`casbin_rule` / `sys_job` / `sys_job_log` 自动，`demo_course` / `sys_trash` 需从 `ry_vue_5.X.sql` 或 `postgresql-5.x-public.sql` 取）。

---

## 哪些表后端会自动建

启动时后端（`main.rs`）会自动创建以下表，**无需手动导入**：

| 表 | 由谁创建 |
|---|---|
| `casbin_rule` | `SeaOrmAdapter::new`（casbin.rs） |
| `sys_job` / `sys_job_log` | `job::service::ensure_tables`（MySQL 分支用 `AUTO_INCREMENT`） |

其余 3 张扩展表 `demo_course` / `sys_trash` + 上游 23 表**必须从 SQL 导入**（已包含在 `ry_vue_5.X.sql` 中）。

---

## 连接串配置

改这两个文件之一（把 `postgres://` 换成 `mysql://`）：

- `backend/application-dev.yml` → `database_url`（本地开发，默认 profile=dev）
- `backend/application.yml` → `database_url`（默认/生产）

MySQL 示例：

```
database_url: "mysql://用户名:密码@localhost:3306/ruoyirust"
```

profile 优先级：命令行 `profile=dev` > 环境变量 `profile` > 默认 dev。

---

## 坑位提醒

1. **文件不带 `CREATE DATABASE` / `USE`**：必须先建库并 `USE`，再 `SOURCE`。
2. **字符集**：SQL 内原有 23 表未显式指定 charset，依赖建库时的 `utf8mb4`；新建的 `demo_course` / `sys_trash` 已显式加 `default charset=utf8mb4` 兜底。建库时务必指定 utf8mb4，否则中文乱码。
3. **`sys_trash.data` 是 `TEXT`**：MySQL 不允许 TEXT 列带 `DEFAULT`，建表时只能可空、不写 default。
4. **`demo_course.course_id` / `sys_trash.id` 非自增**：主键由应用层赋雪花 ID / uuid，建表勿加 `auto_increment`。
5. **`char(1)` 列不能写成 `int(1)`**：sea-orm 实体里 `sys_menu.is_frame` / `is_cache`、`sys_dept.status`、`sys_user.sex` 等都是 `Option<String>`，MySQL 若建为 `int(1)`，查询会报 `decoding column "is_frame": Rust Option<Vec<u8>>(BLOB) not compatible with SQL INT`（int 被当二进制）。`status` / `sex` / `is_frame` / `is_cache` / `visible` / `menu_type` 一律用 `char(1)`。
6. **`sys_post.dept_id` 必须可空**：实体 `SysPost` 没有 `dept_id` 字段，若建为 `not null` 无默认值，新增岗位会报 `1364 Field 'dept_id' doesn't have a default value`（PG 脚本里该列本身就是可空 int8）。
7. **MySQL 严格模式会暴露「局部更新写 NULL」**：`PUT` 只带部分字段（如状态开关只传 `roleId`+`status`）时，若服务层直接整行覆盖，非空列会被写成 NULL 报 `1048`。角色更新已按「仅覆盖 DTO 显式提供的字段」实现（`sys_role_service::update`）。

---

## 无 MySQL 环境时的回归方式

```bash
# 默认 MySQL 版二进制（features 默认已含 mysql）
cd backend && cargo build --bin ruoyi_rust_dev

# PostgreSQL 版（先改 application-*.yml 的 database_url 为 postgres://）
cargo check --bin ruoyi_rust_dev --no-default-features --features postgresql
```

数据库侧无法用单测覆盖时，可用仓库根目录的 `smoke_mysql.ps1` 做端到端接口冒烟（登录 + 71 项读写接口，需后端 60800 与 Redis 就绪）。
