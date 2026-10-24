# rust-ruoyi — 基于 Rust 的 RuoYi 企业级快速开发框架

**简体中文 | [English](README.en.md)**

**QQ交流群：694734955**

---

## 1. 项目简介

**仓库地址：**

- GitHub：<https://github.com/rexyyyy/rust-ruoyi>
- Gitee：<https://gitee.com/witgg2001/rust-ruoyi>

rust-ruoyi 是使用 **Rust** 对若依（RuoYi）系统的**重写**，并搭配若依生态官方前端的项目。
对原框架兼容性高,支持代码生成，启动快捷，性能高资源少
**与 RuoYi 的关系：**

- **后端**：按 RuoYi 的接口规范与功能，用 Rust 全部重写（登录认证、RBAC 权限、系统管理、监控、定时任务等），数据库表结构与 RuoYi 保持一致，接口路径/出入参对齐。
- **前端**：直接复用 [RuoYi-Vue-Plus](https://gitee.com/dromara/RuoYi-Vue-Plus) 官方前端 **plus-ui 5.x**，无需为 Rust 后端单独改造前端。

**技术栈：**

| 层 | 技术 |
|---|---|
| 后端语言 | Rust（actix-web 4 + tokio 异步运行时） |
| ORM | SeaORM 1.1（多数据库） |
| 权限 | casbin（RBAC）+ jsonwebtoken（JWT） |
| 缓存 | 默认 Moka（进程内 TinyLFU + TTL），可切换 Redis |
| 定时任务 | 自研轮询调度器（cron 表达式，方言无关、不绑定特定数据库） |
| 前端 | plus-ui 5.x（Vue 3 + TypeScript + Element Plus + Vite + Pinia + UnoCSS + vxe-table） |
| 数据库 | 默认 PostgreSQL（可切换 MySQL） |
| 代码生成 | codegen 子项目（读表结构生成前后端代码） |

> 仓库早期自带的旧版 Vue3 前端已归档至 `0.1` 分支，主线前端统一为 plus-ui 5.x。

## 2. 项目优势（Rust vs Java Spring Boot）

- **内存占用低**：无 JVM、无 GC 堆开销，常驻内存约为 Spring Boot 同规模服务的 1/5 量级，小内存云服务器即可承载。
- **吞吐高、延迟低**：actix-web 长期位居 TechEmpower 前列；下为本项目实测对比（同一压测环境）：

| 框架 | 持续稳定 RPS | 峰值内存占用 | 99% 分位延迟 |
|:---|---:|---:|:---:|
| **Rust (Actix-Web)** | **110,000** | **250 MB** | **7 ms** |
| Go (Gin) | 105,000 | 190 MB | 10 ms |
| Java (Spring Boot) | 40,000 | 1.4 GB | 50 ms |

- **启动即巅峰**：原生二进制秒级启动，没有 JVM 预热与 JIT 抖动，适合 Serverless / 弹性伸缩场景。
- **部署简单**：编译产物是单个可执行文件（release 开启 LTO + opt-level=z），无需安装 JDK/配置堆参数，容器镜像可压缩到几十 MB。
- **内存安全**：所有权模型在编译期消除数据竞争与悬垂指针，高并发下无 GC 停顿。
- **语法友好**：相比 C++ 现代安全，相比 Java 性能更好，适合追求极致资源效率的团队学习演进。

## 3. 已实现功能

**系统管理**：用户、角色、菜单、部门、岗位、字典类型/字典数据、参数配置、通知公告、个人中心
**认证与权限**：账号密码登录 + 验证码、JWT 无状态认证、casbin 接口级 RBAC 鉴权
**系统监控**：服务监控（CPU/内存/磁盘）、在线用户、缓存监控、操作日志、登录日志
**通用能力**：文件上传、Excel 导入导出（rust_xlsxwriter / calamine）、WebSocket 通讯示例、SSE 服务端推送
**定时任务**：自研「方言无关轮询调度器」（替代 RuoYi ruoyi-job 与原 apalis 引擎），支持 cron 表达式与并发限制，任务数据存在业务库（PostgreSQL / MySQL 均可）
**多数据库**：基于 SeaORM，同一套代码支持 PostgreSQL / MySQL（当前默认 PostgreSQL，通过 Cargo features 切换，切换后建议回归测试）
**缓存**：默认 Moka 进程内缓存（登录态、字典/菜单/参数等业务缓存）；`cache_type` 可切换为 Redis，用于多实例共享会话
**代码生成**：codegen 子项目读取数据库表结构，按模板生成后端（controller/service/domain）+ 前端页面 + SQL 菜单脚本
**扩展示例**：微信公众号消息接入（wx 模块）、demo 业务模块（course 课程管理，演示从建表到生成代码的全流程）

## 4. 交流

- **QQ 交流群：694734955**，欢迎 Rust 后端、若依生态、企业级管理后台方向的同学加入交流。

## 5. 相关链接

**RuoYi 官方生态：**

- RuoYi（单体）：https://gitee.com/y_project/RuoYi
- RuoYi-Vue（前后端分离）：https://gitee.com/y_project/RuoYi-Vue
- RuoYi-Vue-Plus（增强版）：https://gitee.com/dromara/RuoYi-Vue-Plus
- plus-ui（本项目使用的前端）：https://gitee.com/JavaLionLi/plus-ui

**Rust 技术组件：**

- actix-web：https://github.com/actix/actix-web
- SeaORM：https://github.com/SeaQL/sea-orm
- moka（进程内缓存，默认）：https://github.com/moka-rs/moka
- casbin-rs：https://github.com/casbin/casbin-rs
- Redis crate（可选缓存后端）：https://github.com/redis-rs/redis-rs

## 6. 快速开始

### 6.1 快速运行

**前置依赖**

| 依赖 | 要求 |
|---|---|
| Rust 工具链 | 含 `cargo` |
| PostgreSQL | 13~16，建议设置 `lc_messages = 'en_US.UTF-8'` |
| Redis | **可选**。默认走 Moka 进程内缓存，单机启动无需 Redis；多实例共享会话时才用到（届时把 `cache_type` 改为 `redis`） |
| 库表初始化 | 导入 `script/postgresql-5.x-public.sql`（默认库名 `ruoyirust`） |
| 前端 | Node >= 20 + pnpm（目录 `plus-ui-5.X`） |

**后端**

`profile` 解析优先级：命令行参数 `profile=dev` → 环境变量 `profile` → 默认 `dev`（默认即读取 `application-dev.yml`，故本地直接 `cargo run` 即可）。

```bash
# 方式一：cargo 直接运行（默认 profile=dev，读 application-dev.yml）
cd backend
cargo run --bin ruoyi_rust_dev

# 方式二：显式指定 profile（如改用 application-prod.yml）
cargo run --bin ruoyi_rust_prod -- profile=prod

# Windows 一键构建（脚本自动注入 VS2022 工具链后编译）
powershell -File build_backend.ps1
```

默认连接串见 `application-dev.yml`：`postgres://postgres:123456@localhost:5434/ruoyirust`。按需修改为本机环境。

**缓存后端选择**（`application.yml` 的 `cache_type`）

| `cache_type` | 说明 | 适用场景 |
|---|---|---|
| `moka`（默认） | 进程内缓存：TinyLFU 淘汰 + per-entry TTL + 加权容量预算，**零外部依赖** | 单机部署 / 本地开发（推荐） |
| `redis` | 多实例共享会话与缓存，需配 `web_redis_url` | **多实例部署必须用它**，否则登录态无法跨实例共享 |
| `mem` | 旧 HashMap 实现：无容量上限、每次读写全表扫描 | 仅作对照，不建议生产 |

配套配置项（仅 `cache_type: moka` 生效）：

| 配置项 | 默认值 | 说明 |
|---|---|---|
| `cache_max_capacity_mb` | `64` | 容量预算（按 key+value 字节计），超限按 TinyLFU 淘汰冷数据 |
| `cache_ttl_secs` | `0` | 兜底 TTL（秒）。`0`=不设全局上限，每条只按写入时的过期时间；`>0` 作为最长存活上限 |
| `cache_time_to_idle_secs` | `0` | 空闲淘汰时长（秒），`0`=关闭（只按 TTL 淘汰） |
| `cache_maintain_secs` | `60` | 周期维护间隔（秒）：定期驱逐过期条目并刷新计数，`0`=关闭 |

> `cache_type` 留空或写成未知值时，会回落到 `moka` 并打印 warn，**不再 panic**——避免配置笔误导致服务起不来。

**前端（plus-ui-5.X）**

```bash
cd plus-ui-5.X
pnpm install
pnpm dev
```

启动后访问 plus-ui 默认端口，使用系统默认账号登录即可。

### 6.2 快速生成代码模块

rust-ruoyi 提供**两种**代码生成方式，底层都由 `codegen` crate（读表结构 → 渲染模板 → 产出 backend / frontend / sql 三套）驱动：

- **方式一：系统内可视化生成**（推荐日常用）—— 登录 plus-ui 后台，在「系统工具 → 代码生成」界面点选操作，无需离开浏览器。
- **方式二：命令行生成**（`codegen-cli`）—— 适合离线、CI、批量脚本场景，不依赖前端。

#### 6.2.1 系统内可视化代码生成（plus-ui 界面）

后端已实现对标 RuoYi `ruoyi-generator` 的 `modules/tool` 模块，提供 `/tool/gen/*` 接口（导入表、预览、同步结构、批量下载 zip 等），与 plus-ui-5.X 前端 `api/tool/gen` 契约对齐。

**操作步骤**（前端 `plus-ui-5.X` 启动并登录后）：

1. 进入 **系统工具 → 代码生成**；
2. 点 **导入**，勾选目标物理表；
3. 在列表里 **编辑** 该表：填作者、模块名、包路径、前端类型、上级菜单等生成选项；


> 数据库表结构变更后，可在列表里点 **同步** 把 `gen_table_column` 与物理表对齐（对应 `/tool/gen/synchDb/{id}`）。

#### 6.2.2 命令行代码生成（codegen-cli）

适合无前端环境或自动化流水线。必填 `--database-url` 与 `--table`，其余可选。**建议先用 `--preview` 预览、确认无误再落盘。**

```bash
cd codegen

# 1) 预览（不落盘，打印将生成的文件清单）
cargo run --bin codegen-cli -- \
  --database-url postgres://postgres:123456@localhost:5434/ruoyirust \
  --table demo_course \
  --module demo --business course --function 课程 \
  --preview

# 2) 生成到指定目录
cargo run --bin codegen-cli -- \
  --database-url postgres://postgres:123456@localhost:5434/ruoyirust \
  --table demo_course \
  --module demo --business course --function 课程 \
  --out D:/tmp/gen-out
```

**参数说明**

| 参数 | 必填 | 说明 | 默认 |
|---|---|---|---|
| `--database-url` | 是 | 目标库连接串 | — |
| `--table` | 是 | 业务表名 | — |
| `--schema` | 否 | schema 名 | `public` |
| `--module` | 否 | 模块名 | 取表名首段（`demo_course` → `demo`） |
| `--business` | 否 | 业务名 | 表名去首段转 camelCase |
| `--function` | 否 | 功能中文名 | 表注释 |
| `--author` | 否 | 作者 | `ruoyi-rust` |
| `--parent-menu-id` | 否 | 父菜单 ID | `0`（一级目录） |
| `--out` | 否 | 输出根目录 | `./codegen-out` |
| `--preview` | 否 | 仅预览不落盘 | 关闭 |

生成产物含 `backend` / `frontend` / `sql` 三套目录：后端代码并入 `backend/src`、前端页面并入 `plus-ui-5.X`，并执行 `--out` 中的 SQL 注册菜单与权限。
