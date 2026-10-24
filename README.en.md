# rust-ruoyi — Rust-powered RuoYi enterprise rapid development framework

**[简体中文](README.md) | English**

**QQ Group: 694734955**

---

## 1. Introduction

**Repository:**

- GitHub: <https://github.com/rexyyyy/rust-ruoyi>
- Gitee: <https://gitee.com/witgg2001/rust-ruoyi>

rust-ruoyi is a **rewrite of the RuoYi (若依) system in Rust**, paired with the official frontend from the RuoYi ecosystem.

It stays highly compatible with the original framework, supports code generation, starts instantly, and delivers high performance with a small resource footprint.

**Relationship with RuoYi:**

- **Backend**: fully rewritten in Rust following RuoYi's API conventions and feature set (authentication, RBAC permissions, system management, monitoring, scheduled tasks, etc.). The database schema is identical to RuoYi's, and both API paths and request/response payloads are aligned.
- **Frontend**: reuses the official RuoYi-Vue-Plus frontend **plus-ui 5.x** as-is — no Rust-specific frontend changes are required.

**Tech stack:**

| Layer | Technology |
|---|---|
| Backend language | Rust (actix-web 4 + tokio async runtime) |
| ORM | SeaORM 1.1 (multi-database) |
| Authorization | casbin (RBAC) + jsonwebtoken (JWT) |
| Cache | Redis |
| Scheduled tasks | apalis (cron expressions + Postgres storage) |
| Frontend | plus-ui 5.x (Vue 3 + TypeScript + Element Plus + Vite + Pinia + UnoCSS + vxe-table) |
| Database | PostgreSQL by default (MySQL switchable) |
| Code generation | `codegen` sub-project (reads table schema, generates backend + frontend code) |

> The legacy Vue 3 frontend that shipped with the repository early on has been archived to the `0.1` branch; the mainline frontend is plus-ui 5.x.

## 2. Advantages (Rust vs. Java Spring Boot)

- **Low memory footprint**: no JVM, no GC heap overhead. Resident memory is roughly 1/5 of a Spring Boot service of the same scale, so it runs comfortably on small cloud instances.
- **High throughput, low latency**: actix-web consistently ranks near the top of TechEmpower. Benchmarks for this project (same load-test environment):

| Framework | Sustained RPS | Peak memory | p99 latency |
|:---|---:|---:|:---:|
| **Rust (Actix-Web)** | **110,000** | **250 MB** | **7 ms** |
| Go (Gin) | 105,000 | 190 MB | 10 ms |
| Java (Spring Boot) | 40,000 | 1.4 GB | 50 ms |

- **Instant startup**: a native binary starts in seconds — no JVM warm-up, no JIT jitter. Well suited to serverless and elastic scaling scenarios.
- **Simple deployment**: the build artifact is a single executable (release with LTO + `opt-level=z`). No JDK installation, no heap tuning; container images can be compressed to tens of megabytes.
- **Memory safety**: the ownership model eliminates data races and dangling pointers at compile time, with no GC pauses under high concurrency.
- **Approachable syntax**: safer than C++, faster than Java — a good fit for teams pursuing extreme resource efficiency.

## 3. Implemented features

**System management**: users, roles, menus, departments, positions, dictionary types / dictionary data, parameter configuration, notices, personal center

**Authentication & authorization**: username/password login with captcha, stateless JWT authentication, casbin endpoint-level RBAC

**System monitoring**: server monitoring (CPU/memory/disk), online users, cache monitoring, operation logs, login logs

**Common capabilities**: file upload, Excel import/export (rust_xlsxwriter / calamine), WebSocket example, SSE server push

**Scheduled tasks**: based on apalis 1.0 (replacing RuoYi's ruoyi-job), supporting cron expressions, retries, timeouts and concurrency limits; task data is stored in PostgreSQL

**Multi-database**: powered by SeaORM — a single codebase supports PostgreSQL / MySQL (PostgreSQL is the current default; switch via Cargo features and run a regression test after switching)

**Cache**: Redis (login state, business cache)

**Code generation**: the `codegen` sub-project reads database table schemas and generates backend (controller/service/domain) + frontend pages + SQL menu scripts from templates

**Extension examples**: WeChat Official Account message integration (`wx` module), demo business module (course management, demonstrating the whole flow from table creation to generated code)

## 4. Community

- **QQ Group: 694734955** — welcome to join if you work on Rust backends, the RuoYi ecosystem, or enterprise admin platforms.

## 5. Related links

**RuoYi ecosystem:**

- RuoYi (monolith): https://gitee.com/y_project/RuoYi
- RuoYi-Vue (frontend/backend separated): https://gitee.com/y_project/RuoYi-Vue
- RuoYi-Vue-Plus (enhanced): https://gitee.com/dromara/RuoYi-Vue-Plus
- plus-ui (frontend used by this project): https://gitee.com/JavaLionLi/plus-ui

**Rust components:**

- actix-web: https://github.com/actix/actix-web
- SeaORM: https://github.com/SeaQL/sea-orm
- apalis (scheduled tasks): https://github.com/apalis-dev/apalis
- casbin-rs: https://github.com/casbin/casbin-rs
- Redis crate: https://github.com/redis-rs/redis-rs

## 6. Getting started

### 6.1 Run it

**Prerequisites**

| Dependency | Requirement |
|---|---|
| Rust toolchain | including `cargo` |
| PostgreSQL | 13–16, `lc_messages = 'en_US.UTF-8'` recommended |
| Redis | `redis://localhost:6379` by default |
| Schema initialization | import `script/postgresql-5.x-public.sql` (default database `ruoyirust`) |
| Frontend | Node >= 20 + pnpm (directory `plus-ui-5.X`) |

**Backend**

`profile` resolution order: CLI argument `profile=dev` → environment variable `profile` → default `dev` (which reads `application-dev.yml`, so a local `cargo run` works out of the box).

```bash
# Option 1: run with cargo (profile=dev by default, reads application-dev.yml)
cd backend
cargo run --bin ruoyi_rust_dev

# Option 2: specify the profile explicitly (e.g. application-prod.yml)
cargo run --bin ruoyi_rust_prod -- profile=prod

# One-click build on Windows (the script injects the VS2022 toolchain before compiling)
powershell -File build_backend.ps1
```

The default connection string lives in `application-dev.yml`: `postgres://postgres:123456@localhost:5434/ruoyirust`; Redis is configured via `web_redis_url`. Adjust both to your local environment.

**Frontend (plus-ui-5.X)**

```bash
cd plus-ui-5.X
pnpm install
pnpm dev
```

After startup, open plus-ui's default port and log in with the built-in default account.

### 6.2 Generating a code module

rust-ruoyi offers **two** code generation paths, both backed by the `codegen` crate (read table schema → render templates → emit backend / frontend / sql):

- **Option 1: in-app visual generation** (recommended for daily use) — log in to the plus-ui admin, then use *System Tools → Code Generation*; no need to leave the browser.
- **Option 2: command line** (`codegen-cli`) — suitable for offline work, CI, and batch scripts; no frontend required.

#### 6.2.1 In-app visual generation (plus-ui)

The backend implements a `modules/tool` module mirroring RuoYi's `ruoyi-generator`, exposing `/tool/gen/*` endpoints (import tables, preview, synchronize schema, batch zip download, etc.) that match the plus-ui-5.X `api/tool/gen` contract.

**Steps** (with plus-ui-5.X running and logged in):

1. Go to **System Tools → Code Generation**;
2. Click **Import** and select the target physical tables;
3. **Edit** the table row: fill in author, module name, package path, frontend type, parent menu and other generation options.

> After the database schema changes, click **Sync** in the list to realign `gen_table_column` with the physical table (maps to `/tool/gen/synchDb/{id}`).

#### 6.2.2 Command-line generation (codegen-cli)

Useful without a frontend, or in automated pipelines. `--database-url` and `--table` are required; everything else is optional. **Preview with `--preview` first, then write to disk once verified.**

```bash
cd codegen

# 1) Preview (nothing written; prints the list of files that would be generated)
cargo run --bin codegen-cli -- \
  --database-url postgres://postgres:123456@localhost:5434/ruoyirust \
  --table demo_course \
  --module demo --business course --function 课程 \
  --preview

# 2) Generate into a target directory
cargo run --bin codegen-cli -- \
  --database-url postgres://postgres:123456@localhost:5434/ruoyirust \
  --table demo_course \
  --module demo --business course --function 课程 \
  --out D:/tmp/gen-out
```

**Parameters**

| Parameter | Required | Description | Default |
|---|---|---|---|
| `--database-url` | yes | Target database connection string | — |
| `--table` | yes | Business table name | — |
| `--schema` | no | Schema name | `public` |
| `--module` | no | Module name | first segment of the table name (`demo_course` → `demo`) |
| `--business` | no | Business name | table name without its first segment, camelCased |
| `--function` | no | Human-readable function name | table comment |
| `--author` | no | Author | `ruoyi-rust` |
| `--parent-menu-id` | no | Parent menu ID | `0` (top-level directory) |
| `--out` | no | Output root directory | `./codegen-out` |
| `--preview` | no | Preview only, write nothing | off |

The output contains three directory trees — `backend` / `frontend` / `sql`. Merge the backend code into `backend/src`, the frontend pages into `plus-ui-5.X`, then run the SQL from `--out` to register menus and permissions.
