//! 模板自检：模板是编译进二进制的字符串，语法错只有在运行时才会暴露，
//! 这个用例保证改模板后能在 `cargo test` 阶段立刻发现。

// 直接拿模板原文做一次解析，出错时把 Tera 的详细报错打出来。
const FRONTEND_INDEX_VUE: &str = include_str!("../templates/frontend/index.vue.tera");
const BACKEND_ENTITY: &str = include_str!("../templates/backend/entity.rs.tera");

#[test]
fn templates_are_parseable() {
    let mut tera = tera::Tera::default();
    tera.add_raw_templates(vec![
        ("frontend/index.vue", FRONTEND_INDEX_VUE),
        ("backend/entity.rs", BACKEND_ENTITY),
    ])
        .expect("模板语法错误");
}
