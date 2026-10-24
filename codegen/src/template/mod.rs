//! 模板注册中心。
//!
//! 模板用 `include_str!` **编译进二进制**，而不是像旧版那样用
//! `Tera::new("templates/**/*.tera")` 到运行目录去找。
//! 旧写法有两个坑：一是工作目录一换就报模板找不到，二是 codegen 被 backend
//! 内嵌后，`backend/` 下根本不存在 `templates/`。编译进包，任何场景都能跑。
//!
//! 想改模板：编辑 `templates/**` 下对应文件后重新编译即可。

use once_cell::sync::Lazy;
use tera::{Context, Tera};

use crate::error::{Error, Result};

// ------------------------------- 后端模板 -------------------------------
const BE_ENTITY: &str = include_str!("../../templates/backend/entity.rs.tera");
const BE_BO: &str = include_str!("../../templates/backend/bo.rs.tera");
const BE_VO: &str = include_str!("../../templates/backend/vo.rs.tera");
const BE_DOMAIN_MOD: &str = include_str!("../../templates/backend/domain_mod.rs.tera");
const BE_SERVICE: &str = include_str!("../../templates/backend/service.rs.tera");
const BE_SERVICE_MOD: &str = include_str!("../../templates/backend/service_mod.rs.tera");
const BE_CONTROLLER: &str = include_str!("../../templates/backend/controller.rs.tera");
const BE_CONTROLLER_MOD: &str = include_str!("../../templates/backend/controller_mod.rs.tera");
const BE_MODULE_MOD: &str = include_str!("../../templates/backend/module_mod.rs.tera");

// ------------------------------- 前端模板 -------------------------------
const FE_API: &str = include_str!("../../templates/frontend/api.ts.tera");
const FE_TYPES: &str = include_str!("../../templates/frontend/types.ts.tera");
const FE_INDEX_VUE: &str = include_str!("../../templates/frontend/index.vue.tera");

// ------------------------------- SQL 模板 -------------------------------
const SQL_MENU: &str = include_str!("../../templates/sql/menu.sql.tera");

/// 模板名。`render()` 用这些常量，重命名模板文件不会造成运行时 404。
pub mod name {
    pub const BE_ENTITY: &str = "backend/entity.rs";
    pub const BE_BO: &str = "backend/bo.rs";
    pub const BE_VO: &str = "backend/vo.rs";
    pub const BE_DOMAIN_MOD: &str = "backend/domain_mod.rs";
    pub const BE_SERVICE: &str = "backend/service.rs";
    pub const BE_SERVICE_MOD: &str = "backend/service_mod.rs";
    pub const BE_CONTROLLER: &str = "backend/controller.rs";
    pub const BE_CONTROLLER_MOD: &str = "backend/controller_mod.rs";
    pub const BE_MODULE_MOD: &str = "backend/module_mod.rs";

    pub const FE_API: &str = "frontend/api.ts";
    pub const FE_TYPES: &str = "frontend/types.ts";
    pub const FE_INDEX_VUE: &str = "frontend/index.vue";

    pub const SQL_MENU: &str = "sql/menu.sql";
}

static TERA: Lazy<std::result::Result<Tera, String>> = Lazy::new(|| {
    let pairs: Vec<(&str, &str)> = vec![
        (name::BE_ENTITY, BE_ENTITY),
        (name::BE_BO, BE_BO),
        (name::BE_VO, BE_VO),
        (name::BE_DOMAIN_MOD, BE_DOMAIN_MOD),
        (name::BE_SERVICE, BE_SERVICE),
        (name::BE_SERVICE_MOD, BE_SERVICE_MOD),
        (name::BE_CONTROLLER, BE_CONTROLLER),
        (name::BE_CONTROLLER_MOD, BE_CONTROLLER_MOD),
        (name::BE_MODULE_MOD, BE_MODULE_MOD),
        (name::FE_API, FE_API),
        (name::FE_TYPES, FE_TYPES),
        (name::FE_INDEX_VUE, FE_INDEX_VUE),
        (name::SQL_MENU, SQL_MENU),
    ];
    let mut tera = Tera::default();
    tera.add_raw_templates(pairs).map_err(|e| e.to_string())?;
    Ok(tera)
});

/// 渲染模板
pub fn render(template: &str, ctx: &Context) -> Result<String> {
    let tera = TERA
        .as_ref()
        .map_err(|e| Error::E(format!("模板初始化失败: {}", e)))?;
    Ok(tera.render(template, ctx)?)
}
