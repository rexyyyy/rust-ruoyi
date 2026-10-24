
vscode windows 运行
首先在 PowerShell 下设置环境变量
$env:RUST_BACKTRACE=1 ;
$env:profile= "dev" ;

cargo run --bin ruoyi_rust_dev

$env:RUST_BACKTRACE=1 ;
$env:profile= "prod" ;

cargo run --bin ruoyi_rust_prod


---

# 代码生成器（codegen）新增业务模块流程

以本次新增 `oms/regorder`（注册码登记）模块为例，完整链路如下。

## 1. 建表 → 导入表 → 生成代码

1. 在数据库建好业务表（例：`oms_regorder`，表名用 `模块_业务` 前缀，如 `oms_regorder`）。
2. 前端「系统工具 → 代码生成」中「导入」该表，编辑生成信息：
   - **模块名（module_name）**、**业务名（business_name）**、**功能名/菜单名**、**权限前缀（perm）**。
3. 点击「生成」，产出三部分：
   - 后端：`src/modules/<module>/`（controller / domain(bo,entity,vo) / service）
   - 前端：`src/views/<module>/<business>/index.vue` + `src/api/<module>/<business>/{index.ts,types.ts}`
   - SQL：菜单初始化脚本（`menu.sql`，创建一级目录 `<module>` + 二级菜单 + 按钮权限）

> ⚠️ **命名不能与已有模块/菜单重复**（本次踩坑重点）：
> - 模块名重复 → 后端 `src/modules/` 目录、`pub mod` 冲突；
> - 菜单 `path` 片段重复 → 前端路由 `name` 可能重复（详见「问题 1」）；
> - 权限标识 `perm` 重复 → `menu.sql` 的 `WHERE NOT EXISTS (... perms=...)` 判重会跳过插入，菜单生成不出来。
> 建议：模块名、菜单 `path`、`perm` 前缀三者都取全局唯一值。

## 2. 将生成的代码接入实际系统

### 后端（两步，缺一不可）
1. `src/modules/mod.rs` 增加：`pub mod oms;`
2. **`src/http_server.rs` 增加 scope 挂载**（codegen 不会自动写入，必须手工加）：
   ```rust
   ).service(
       web::scope("/oms") //业务模块（codegen 生成）
           .configure(crate::modules::oms::controller::init)
   ```
   放在 `web::scope(&base_api)` 内、与其它 scope 同级即可。
3. `cargo build`（或 `cargo run --bin ruoyi_rust_dev`）重启后端。

### 前端
1. 把生成的 `src/views/<module>/<business>/` 与 `src/api/<module>/<business>/` 拷贝到 `plus-ui-5.X/src` 对应目录。
2. 执行第 1 步生成的 `menu.sql`，把菜单写入 `sys_menu`。
3. **刷新菜单缓存**（`getRouters` 读的是 Redis 里的 `sys_menu:all_vo`）：
   - 重启后端，或运行 `cargo run --example refresh_menu_cache`；
   - 前端**硬刷新一次页面**（Ctrl+F5），让 `import.meta.glob` 重新收集到新加的 `views/**/*.vue`。

## 3. 本次发现的问题与注意事项（重要）

### 问题 1：前端页面 404（动态路由匹配不到）
- **根因**：Rust 移植版生成路由 `name` 时漏了 Java 原版的 `+ menuId` 后缀（Java 为 `getRouteName() + getMenuId()`）。当两个菜单 `path` 片段相同（本次是两个 `/demo` 都叫 `Demo`）时，`name` 重复；vue-router `addRoute` 遇同名会**顶掉先注册的旧路由**，导致新模块落到兜底 `/:pathMatch(.*)*` → 404。
- **修复**：`src/modules/system/service/sys_menu_service.rs` 的 `get_route_name` 及 menu-frame / inner-link 子路由命名分支，均追加 `menu_id`，保证 `name` 全局唯一。
- **注意**：后端已保证 `name` 唯一，但**菜单 `path`、`perm` 仍需人工保证不与已有数据重复**，否则仍会出现路由/菜单异常。

### 问题 2：后端接口 404（`/dev-api/<module>/**`）
- **根因**：codegen 生成的模块，只加了 `pub mod <module>;`，但**没有在 `http_server.rs` 挂载 `web::scope("/<module>")`**，所以该模块所有接口 404。
- **修复/规范**：每新增一个 codegen 模块，务必在 `http_server.rs` 补一行 `web::scope("/<module>").configure(<module>::controller::init)`。

### 通用检查清单（新模块上不了线时按序排查）
1. 接口 404 → `http_server.rs` 是否挂了该 scope？后端是否重新编译并重启？
2. 页面 404 → `getRouters` 返回里该菜单 `path`/`name` 是否正确、`name` 是否与别的菜单重复？
3. 菜单不显示 → `menu.sql` 是否执行成功（`perm` 是否因判重被跳过）？Redis 菜单缓存是否刷新？
4. 页面空白/组件报错 → 前端是否硬刷新（`import.meta.glob` 是否收集到新 `views` 文件）？`menu.component` 是否与 `views/<module>/<business>/index` 一致？
5. 列表能进但数据接口 404 → 后端 scope 前缀（`/<module>`）与控制器 `#[get("/<business>/list")]` 拼出的完整路径，是否与前端 `api` 请求路径一致？



