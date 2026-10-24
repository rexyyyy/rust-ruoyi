# RuoYi Rust 后端 ↔ RuoYi-Vue-Plus-5.X 包名与接口对齐报告

> 项目：`D:\rust-proj\rust-ruoyi\backend`（Rust 后端，bin = `ruoyi_rust_dev`）
> 参考：`D:\rust-proj\rust-ruoyi\ref\RuoYi-Vue-Plus-5.X\RuoYi-Vue-Plus-Single-5.X`（Spring Boot）
> 验收参照：`D:\rust-proj\rust-ruoyi\plus-ui-5.X\src\api`（官方 Vue3 前端接口契约）
> 验证：`cargo check --bin ruoyi_rust_dev` 与 `cargo build --bin ruoyi_rust_dev` 全部通过（0 error）

---

## 一、模块（包）结构对照

Java 包 `org.dromara.system.controller.system.SysXxxController` 已镜像为 Rust 模块路径，**目录层级一一对应**：

| Spring（Java 包路径） | Rust（模块路径） | 状态 |
|---|---|---|
| `org.dromara.system.controller.system.SysUserController` | `src/modules/system/controller/system/sys_user_controller.rs` | ✅ 已镜像 |
| `org.dromara.system.controller.system.SysRoleController` | `src/modules/system/controller/system/sys_role_controller.rs` | ✅ 已镜像 |
| `org.dromara.system.controller.system.SysMenuController` | `src/modules/system/controller/system/sys_menu_controller.rs` | ✅ 已镜像 |
| `org.dromara.system.controller.system.SysDeptController` | `src/modules/system/controller/system/sys_dept_controller.rs` | ✅ 已镜像 |
| `org.dromara.system.controller.system.SysPostController` | `src/modules/system/controller/system/sys_post_controller.rs` | ✅ 已镜像 |
| `org.dromara.system.controller.system.SysDictTypeController` | `src/modules/system/controller/system/sys_dict_type_controller.rs` | ✅ 已镜像 |
| `org.dromara.system.controller.system.SysDictDataController` | `src/modules/system/controller/system/sys_dict_data_controller.rs` | ✅ 已镜像 |
| `org.dromara.system.controller.system.SysConfigController` | `src/modules/system/controller/system/sys_config_controller.rs` | ✅ 已镜像 |
| `org.dromara.system.controller.system.SysNoticeController` | `src/modules/system/controller/system/sys_notice_controller.rs` | ✅ 已镜像 |
| `org.dromara.system.controller.system.SysProfileController` | `src/modules/system/controller/system/sys_profile_controller.rs` | ✅ 已镜像 |
| `org.dromara.system.controller.monitor.SysLogininforController` | `src/modules/system/controller/monitor/sys_logininfor_controller.rs` | ✅ 已镜像 |
| `org.dromara.system.controller.monitor.SysOperlogController` | `src/modules/system/controller/monitor/sys_operlog_controller.rs` | ✅ 已镜像（新增） |
| `org.dromara.system.controller.monitor.SysUserOnlineController` | `src/modules/system/controller/monitor/sys_user_online_controller.rs` | ✅ 已镜像 |
| `org.dromara.system.controller.monitor.CacheController` | `src/modules/system/controller/monitor/cache_controller.rs` | ✅ 已镜像（新增） |
| `org.dromara.web.controller.AuthController` | `src/modules/system/controller/sys_auth_controller.rs` | ✅ 已对齐（/auth/*） |
| `org.dromara.common.sse.controller.SseController` | `src/modules/system/controller/system_sse.rs` | ✅ 已对齐（/resource/sse） |
| `org.dromara.system.domain.SysUser`（实体） | `src/modules/system/domain/tables.rs` | ✅ 已镜像 |
| `org.dromara.system.domain.bo.*`（DTO） | `src/modules/system/domain/bo/*.rs`（原 dto 目录） | ✅ 已改名 bo |
| `org.dromara.system.domain.vo.*` | `src/modules/system/domain/vo/*.rs` | ✅ 已镜像 |
| `org.dromara.system.mapper.*` | `src/modules/system/mapper/*.rs` | ✅ 已镜像（原 domain/mapper → system/mapper） |

## 二、路由（接口名）对照表

统一前缀：Spring `/dev-api`、Rust `/dev-api|/prod-api`。表中路由均为前缀之后的路径。

### 2.1 认证 Auth

| Spring | Rust | 状态 |
|---|---|---|
| `POST /auth/login` | `POST /auth/login` | ✅ 已对齐 |
| `POST /auth/logout` | `POST /auth/logout` | ✅ 已对齐 |
| `GET /auth/code`（验证码） | `GET /auth/code`（原 `/captchaImage` 已删除） | ✅ 已对齐 |
| `POST /auth/register` | `POST /auth/register` | ✅ 已实现（建用户+默认角色） |
| `GET /auth/tenant/list` | `GET /auth/tenant/list` | ✅ 已实现（单租户返回空） |
| `POST /auth/social/callback` | `POST /auth/social/callback` | ⚠️ 返回“未配置社交登录”（无 sys_social 表） |
| `GET /system/user/getInfo` | `GET /system/user/getInfo` | ✅ 已对齐（原 `/getInfo` 迁移） |

### 2.2 用户 SysUser / SysProfile

| Spring | Rust | 状态 |
|---|---|---|
| `GET /system/user/list` | `GET /system/user/list` | ✅ |
| `GET /system/user/{userId}` | `GET /system/user/{user_id}` | ✅ |
| `POST /system/user` | `POST /system/user` | ✅ |
| `PUT /system/user` | `PUT /system/user` | ✅ |
| `DELETE /system/user/{userIds}` | `DELETE /system/user/{user_ids}`（逗号批量） | ✅ |
| `PUT /system/user/resetPwd` | `PUT /system/user/resetPwd`（真正重置密码） | ✅ |
| `PUT /system/user/changeStatus` | `PUT /system/user/changeStatus` | ✅ |
| `GET /system/user/optionselect?userIds=` | `GET /system/user/optionselect` | ✅ 已实现（新增） |
| `GET /system/user/list/dept/{deptId}` | `GET /system/user/list/dept/{dept_id}` | ✅ 已实现（新增） |
| `GET /system/user/deptTree` | `GET /system/user/deptTree` | ✅ |
| `PUT /system/user/authRole` | `PUT /system/user/authRole` | ✅ |
| `GET /system/user/authRole/{userId}` | `GET /system/user/authRole/{user_id}` | ✅ |
| `GET /system/user/profile` | `GET /system/user/profile` | ✅ |
| `PUT /system/user/profile` | `PUT /system/user/profile` | ✅ |
| `PUT /system/user/profile/updatePwd` | `PUT /system/user/profile/updatePwd` | ✅ |
| `POST /system/user/profile/avatar` | `POST /system/user/profile/avatar`（multipart 字段 avatarfile） | ✅ 已实现（新增） |
| `POST /system/user/export` | `POST /system/user/export` | ✅ 已实现（新增，xlsx） |
| `POST /system/user/importTemplate` | `POST /system/user/importTemplate` | ✅ 已实现（新增） |
| `POST /system/user/importData` | `POST /system/user/importData`（multipart 字段 file） | ✅ 已实现（新增） |
| `GET /profile/avatar/{file}`（静态） | `GET /profile/avatar/{file}` | ✅ 已实现（新增） |

### 2.3 角色 SysRole

| Spring | Rust | 状态 |
|---|---|---|
| `GET /system/role/list` | `GET /system/role/list` | ✅ |
| `GET /system/role/{roleId}` | `GET /system/role/{role_id}` | ✅ |
| `POST /system/role` | `POST /system/role` | ✅ |
| `PUT /system/role` | `PUT /system/role` | ✅ |
| `DELETE /system/role/{roleIds}` | `DELETE /system/role/{role_ids}` | ✅ |
| `PUT /system/role/dataScope` | `PUT /system/role/dataScope`（写 data_scope + 重建 sys_role_dept） | ✅ 已实现（新增） |
| `PUT /system/role/changeStatus` | `PUT /system/role/changeStatus` | ✅ |
| `GET /system/role/optionselect?roleIds=` | `GET /system/role/optionselect` | ✅ 已实现（新增） |
| `GET /system/role/deptTree/{roleId}` | `GET /system/role/deptTree/{role_id}` | ✅ 已实现（新增） |
| `GET /system/role/authUser/allocatedList` | `GET /system/role/authUser/allocatedList` | ✅ |
| `GET /system/role/authUser/unallocatedList` | `GET /system/role/authUser/unallocatedList` | ✅ |
| `PUT /system/role/authUser/cancel` | `PUT /system/role/authUser/cancel` | ✅ |
| `PUT /system/role/authUser/cancelAll` | `PUT /system/role/authUser/cancelAll` | ✅ |
| `PUT /system/role/authUser/selectAll` | `PUT /system/role/authUser/selectAll` | ✅ |
| `POST /system/role/export` | `POST /system/role/export` | ✅ 已实现（新增） |

### 2.4 菜单 SysMenu

| Spring | Rust | 状态 |
|---|---|---|
| `GET /system/menu/list` | `GET /system/menu/list` | ✅ |
| `GET /system/menu/{menuId}` | `GET /system/menu/{menu_id}` | ✅ |
| `POST /system/menu` | `POST /system/menu` | ✅ |
| `PUT /system/menu` | `PUT /system/menu` | ✅ |
| `DELETE /system/menu/{menuId}` | `DELETE /system/menu/{menu_id}` | ✅ |
| `DELETE /system/menu/cascade/{menuIds}` | `DELETE /system/menu/cascade/{menu_ids}`（递归删子孙+角色菜单关系） | ✅ 已实现（新增） |
| `GET /system/menu/treeselect` | `GET /system/menu/treeselect` | ✅ |
| `GET /system/menu/roleMenuTreeselect/{roleId}` | `GET /system/menu/roleMenuTreeselect/{role_id}` | ✅ |
| `GET /system/menu/getRouters` | `GET /system/menu/getRouters` | ✅ 已对齐（原 `/getRouters` 迁移） |
| `GET /system/menu/tenantPackageMenuTreeselect/{packageId}` | — | ⚠️ 租户套餐，无 sys_tenant 表，未实现 |

### 2.5 部门 / 岗位 / 字典 / 参数 / 公告

| Spring | Rust | 状态 |
|---|---|---|
| `GET /system/dept/list`、`/system/dept/{deptId}`、POST/PUT/DELETE | 同名 | ✅ |
| `GET /system/dept/optionselect?deptIds=` | `GET /system/dept/optionselect` | ✅ 已实现（新增） |
| `GET /system/dept/list/exclude/{deptId}` | `GET /system/dept/list/exclude/{dept_id}` | ✅ 已实现（新增） |
| `POST /system/dept/export` | `POST /system/dept/export` | ✅ 已实现（新增） |
| `GET /system/post/list`、`/system/post/{postId}`、POST/PUT/DELETE | 同名（detail 参数已改为 `{post_id}`） | ✅ |
| `GET /system/post/optionselect` | `GET /system/post/optionselect` | ✅ 已实现（新增） |
| `GET /system/post/deptTree` | `GET /system/post/deptTree` | ✅ 已实现（新增） |
| `POST /system/post/export` | `POST /system/post/export` | ✅ 已实现（新增） |
| `GET /system/dict/type/list`、`/{dictId}`、POST/PUT/DELETE | 同名 | ✅ |
| `DELETE /system/dict/type/refreshCache` | `DELETE /system/dict/type/refreshCache` | ✅ 已实现（新增） |
| `GET /system/dict/type/optionselect` | `GET /system/dict/type/optionselect` | ✅ |
| `POST /system/dict/type/export` | `POST /system/dict/type/export` | ✅ 已实现（新增） |
| `GET /system/dict/data/list`、`/{dictCode}`、POST/PUT/DELETE | 同名 | ✅ |
| `GET /system/dict/data/type/{dictType}` | `GET /system/dict/data/type/{dict_type}` | ✅ |
| `POST /system/dict/data/export` | `POST /system/dict/data/export` | ✅ 已实现（新增） |
| `GET /system/config/list`、`/{configId}`、POST/PUT/DELETE | 同名 | ✅ |
| `GET /system/config/configKey/{configKey}` | `GET /system/config/configKey/{config_key}` | ✅ 已实现（新增，返回 {configKey,configValue}） |
| `PUT /system/config/updateByKey` | `PUT /system/config/updateByKey` | ✅ 已实现（新增） |
| `DELETE /system/config/refreshCache` | `DELETE /system/config/refreshCache` | ✅ 已实现（新增） |
| `POST /system/config/export` | `POST /system/config/export` | ✅ 已实现（新增） |
| `GET /system/notice/list`、`/{noticeId}`、POST/PUT/DELETE | 同名 | ✅ |

### 2.6 监控 Monitor

| Spring | Rust | 状态 |
|---|---|---|
| `GET /monitor/logininfor/list`、`/{infoId}` | 同名 | ✅ |
| `GET /monitor/logininfor/unlock/{userName}` | `GET /monitor/logininfor/unlock/{user_name}`（删缓存键 pwd_err_cnt） | ✅ 已实现（新增） |
| `DELETE /monitor/logininfor/clean` | `DELETE /monitor/logininfor/clean` | ✅ 已实现（新增） |
| `POST /monitor/logininfor/export` | `POST /monitor/logininfor/export` | ✅ 已实现（新增） |
| `GET /monitor/online/list` | `GET /monitor/online/list` | ✅ |
| `DELETE /monitor/online/{tokenId}` | `DELETE /monitor/online/{token_id}` | ✅ |
| `DELETE /monitor/online/myself/{tokenId}` | `DELETE /monitor/online/myself/{token_id}` | ✅ 已实现（新增） |
| `GET /monitor/operlog/list` | `GET /monitor/operlog/list` | ✅ 已实现（新增模块） |
| `DELETE /monitor/operlog/{operIds}` | `DELETE /monitor/operlog/{oper_ids}` | ✅ 已实现（新增） |
| `DELETE /monitor/operlog/clean` | `DELETE /monitor/operlog/clean` | ✅ 已实现（新增） |
| `POST /monitor/operlog/export` | `POST /monitor/operlog/export` | ✅ 已实现（新增） |
| `GET /monitor/cache` | `GET /monitor/cache` | ✅ 已实现（新增） |
| `GET /monitor/cache/getNames` | `GET /monitor/cache/getNames` | ✅ 已实现（新增） |
| `GET /monitor/cache/getKeys/{cacheName}` | `GET /monitor/cache/getKeys/{cache_name}` | ✅ 已实现（新增） |
| `GET /monitor/cache/getValue/{cacheName}/{cacheKey}` | `GET /monitor/cache/getValue/{cache_name}/{cache_key}` | ✅ 已实现（新增） |
| `DELETE /monitor/cache/clearCacheName/{cacheName}` | 同名 | ✅ 已实现（新增） |
| `DELETE /monitor/cache/clearCacheKey/{cacheKey}` | 同名 | ✅ 已实现（新增） |
| `DELETE /monitor/cache/clearCacheAll` | `DELETE /monitor/cache/clearCacheAll` | ✅ 已实现（新增） |

### 2.7 实时通知 SSE 与 Rust 扩展（保留）

| 路由 | 说明 | 状态 |
|---|---|---|
| `GET /resource/sse` | SSE 推送（text/event-stream 无限流，15s 心跳） | ✅ 已对齐（对应 SseController `${sse.path}`） |
| `GET /resource/sse/close` | 关闭 SSE 连接 | ✅ 已对齐 |
| `GET /wx/wecallback`、`GET /ws/{user_id}`、`GET /monitor/server`、`POST /system/uploads`、`POST /system/uploadfrm` | Rust 上游扩展接口 | ➖ 按用户要求保留 |

### 2.8 明确不在本范围（无对应数据表）

`/system/tenant/*`、`/system/client/*`、`/system/social/list`、`/auth/binding/{source}`、`/auth/unlock/{authId}`、`/resource/oss/*`、`/tool/gen/*`、`/workflow/*`、`/demo/*` —— 数据库无 `sys_tenant / sys_social / sys_oss / sys_client` 等表，前端对应页面本就不在可运行范围；`sys_oper_log / sys_role_dept / sys_user_post` 表存在，故相关模块已实现。

## 三、本次新增实现内容

1. **操作日志模块（monitor/operlog）**：`SysOperLog` 实体（tables.rs）、`OperLogPageDTO`、`SysOperLogVO`（camelCase）、`mapper/sys_oper_log.rs`、`service/sys_operlog_service.rs`（list/remove/clean/record）、`controller/monitor/sys_operlog_controller.rs`（list/export/remove/clean）。登录、登出、用户增/改/删已自动记录操作日志（异步写入）。
2. **Excel 导出（7 个模块）**：`util/excel.rs`（rust_xlsxwriter 生成 + calamine 解析 + 下载响应）。user/role/config/dictType/dictData/post/logininfor/operlog 均支持 `POST */export`。
3. **用户导入**：`POST /system/user/importTemplate`（模板下载）、`POST /system/user/importData`（multipart 解析 xlsx，跳过表头、账号查重、默认密码取配置 `sys.user.initPassword`（缺省 123456），返回 successNum/failureNum/failureList）。
4. **头像上传**：`POST /system/user/profile/avatar`（actix-multipart 0.7 `#[derive(MultipartForm)]` 写法，保存到 `./uploads/avatar`）+ `GET /profile/avatar/{file}` 静态访问。
5. **缓存监控**：`monitor/cache_controller.rs` 7 个接口（login_tokens / sys_dict / sys_config / sys_menu 缓存前缀，mem/redis 双实现均可用）。
6. **角色数据范围**：`PUT /system/role/dataScope` 写 `sys_role.data_scope` 并重建 `sys_role_dept`；`GET /system/role/deptTree/{roleId}`。
7. **下拉列表**：user/role/dept/post 的 optionselect；`GET /system/user/list/dept/{deptId}`。
8. **菜单级联删除**：`DELETE /system/menu/cascade/{menuIds}` 递归删除子孙菜单并清理 `sys_role_menu`。
9. **杂项**：config configKey/updateByKey/refreshCache；dict refreshCache；logininfor clean/unlock；online remove_self；`resetPwd` 改为真实改密（PasswordEncoder::encode）。

## 四、关键文件与配置改动

- `src/http_server.rs`：路由总表（镜像 /system、/monitor、/auth、/resource 四个 scope；保留 /wx、/ws、uploads 扩展）。
- `src/modules/system/controller/{system,monitor}/…`：10 个系统控制器移入 `controller/system/`，新增 monitor 控制器。
- `src/modules/system/domain/`：原 `dto/` → `bo/`，原 `domain/mapper/` → `system/mapper/`，原 `domain/table/` 并入 `tables.rs`。
- `src/modules/system/service/`：新增 sys_operlog_service，CONTEXT 挂载。
- `application-dev.yml` / `application-prod.yml`：白名单改为 `/auth/login|logout|code|register|tenant/list`、`/resource/sse|sse/close`、`/system/user/getInfo`（已移除旧 `/login`、`/logout`、`/captchaImage`）。
- `Cargo.toml`：新增 `rust_xlsxwriter = "0.99.1"`、`calamine = "0.36.1"`。

## 五、构建验证

```
cd D:\rust-proj\rust-ruoyi\backend
cargo check --bin ruoyi_rust_dev   # 0 error（34 warnings 为既有代码风格提示）
cargo build --bin ruoyi_rust_dev   # Finished dev profile，产物 target\debug\ruoyi_rust_dev.exe
```

修复过程中处理的编译问题（全部清零）：E0252 重复导入、impl 块外方法（self 参数）、rbatis 无 `select_in_column`（改原生 SQL IN）、actix-multipart 0.7 需 `#[derive(MultipartForm)]`、`page`/`config_key` 命名与 actix 生成 unit struct 冲突（改名）、Excel 表头引号、`&str` 参数类型等。

## 六、遗留差异（如实说明）

1. **操作日志覆盖面**：当前记录「登录 / 退出登录 / 用户新增 / 用户修改 / 用户删除」；角色、菜单、部门、岗位、字典、参数、公告的增删改尚未埋点（`SysOperlogService::record` 已提供，按需在各 controller 追加一行即可）。
2. **登录日志**：`sys_logininfor` 沿用既有 `add_async` 写入机制。
3. **租户 / 社交 / OSS / 客户端 / 工作流 / 代码生成**：无对应数据表，接口未实现（前端对应页面不可用），与 Spring 全功能版存在差异。
4. **分页查询条件**：operlog 页面的起止时间筛选（`params[beginTime/endTime]`）未实现，按 title/operName/businessType/status 过滤可用。

---

## 七、响应层级（envelope）对齐 —— 全量排查与修复

### 7.1 判定基准

前端 `plus-ui-5.X/src/utils/request.ts` 的响应拦截器返回的是 **`res.data`，即整个后端 body**。因此后端返回结构的层级必须与前端取值方式逐字段一致。后端三套封装的语义：

| 后端封装 | 序列化结果 | 前端取值 | 适用 |
|---|---|---|---|
| `RespVO<T>` | `{code, msg, data}` | `res.data` | 单对象 / 增删改 |
| `PageVO<T>` | `{code, msg, rows, total}` | `res.rows` / `res.total` | 分页列表 |
| `RespJson` | 任意平铺键值 | 按 key 顶层取 | 老 RuoYi-Vue2 风格特例 |

关键结论：**默认 `data` 包裹是正确的**，只有少数「复合对象」接口出现问题——这些接口原本按 RuoYi-Vue2 习惯平铺，而 plus-ui（Vue3）已统一改为从 `data` 中取。

### 7.2 修复清单（后端改，前端零改动）

| # | 接口 | 修复前 | 修复后（前端期望） | 影响面 |
|---|---|---|---|---|
| 1 | `GET /system/user/profile` | `{data: user, roleGroup: 顶层}`，无 postGroup | `{data:{user, roleGroup, postGroup}}` | 个人中心页全空 |
| 2 | `GET /system/user/{userId}` | `{data: user, posts/roles/roleIds: 顶层}`，无 postIds | `{data:{user, posts, roles, postIds, roleIds}}` | 用户编辑对话框全空 |
| 3 | `GET /system/user/` | `{posts, roles}` 顶层 | `{data:{posts, roles}}` | 新增用户下拉为空 |
| 4 | `GET /system/user/authRole/{userId}` | `{user, roles}` 顶层 | `{data:{user, roles}}` | 分配角色页空白 |
| 5 | `GET /system/role/deptTree/{roleId}` | `{depts, checkedKeys}` 顶层 | `{data:{depts, checkedKeys}}` | 数据权限树空白/报错 |
| 6 | `GET /system/menu/roleMenuTreeselect/{roleId}` | `{menus, checkedKeys}` 顶层 | `{data:{menus, checkedKeys}}` | 角色菜单树空白/报错 |
| 7 | `GET /system/role/authUser/allocatedList` | 只有 `rows`，缺 `total` | `{rows, total}` | 分页失效 |
| 8 | `GET /system/role/authUser/unallocatedList` | 只有 `rows`，缺 `total` | `{rows, total}` | 分页失效 |
| 9 | `POST /system/user/profile/avatar` | `{imgUrl}` 顶层 | `{data:{imgUrl}}` | 头像上传后不回显 |
| 10 | `GET /system/dept/{deptId}` | 无 `data`（查询失败） | `{data:{..., parentId, parentName}}` | 部门编辑表单空白 |

### 7.3 连带发现：bigint 主键按字符串比较导致查询/删除静默失效

PostgreSQL 下 `bigint` 列与 `text` 参数比较不报错但**永远匹配不到行**，故障表现为「接口返回成功但没效果」。

| 位置 | 原写法 | 修复 |
|---|---|---|
| `sys_dept_service::detail` / `remove` | `DeptId.eq(dept_id.to_string())` | `eq(dept_id.parse::<i64>())` |
| `sys_post_service::detail` / `remove` | `PostId.eq(post_id.to_string())` | `eq(post_id.parse::<i64>())` |
| `sys_notice_service::detail` / `remove` | `NoticeId.eq(notice_id.to_string())` | `eq(notice_id.parse::<i64>())` |
| `sys_user_service::remove` | `UserId.eq(id.to_string())` | `eq(id.parse::<i64>())` |
| `sys_operlog_service::remove` | `OperId.eq(oper_id.to_string())` | `eq(oper_id.parse::<i64>())` |
| `sys_logininfor_service::remove` | `InfoId.eq(info_id.to_string())` | `eq(info_id.parse::<i64>())` |

### 7.4 新增能力

- `SysUserService::finds_post_ids_by_user_id` / `finds_post_names_by_user_id`：查 `sys_user_post` 补出 `postIds` 与 `postGroup`。
- `mapper::sys_user::db_auth_user_count` / `db_unallocated_user_count`：为角色授权用户列表补 `total`（where 条件与对应 list 保持一致）。
- `SysDeptVO` 新增 `parent_name` 字段，`detail` 中回填上级部门名。

### 7.5 验收结果

编译 `cargo build` 通过（0 error）；启动后以 admin 登录后逐接口实测，**22 个接口、37 个关键字段全部命中（FAILED=0）**，覆盖登录、用户、角色、部门、岗位、菜单、字典、参数、公告、监控等模块。

### 7.6 与后端层级无关、属「未实现」的差异（不计入本次修复）

- `/resource/oss/*`（OSS 对象存储）：前端 ImageUpload / FileUpload 期望 `{data:{url, fileName, ossId}}`，后端 `/resource` scope 为空，OSS 模块整体未实现。
- `/system/uploads`、`/system/uploadfrm`：Rust 扩展的自定义上传，返回 `file_id` / `filePath`，非 RuoYi 标准 OSS 契约，前端无对应消费方。
