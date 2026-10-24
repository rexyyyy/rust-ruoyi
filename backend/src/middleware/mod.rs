// 统一鉴权中间件实现。
//
// auth_actix：actix-web Transform 实现，引用的路径均为迁移后的
//   crate::modules::system::service::CONTEXT / crate::modules::system::domain::vo。
// auth：orm 迁移前的旧实现，仍引用 crate::service::CONTEXT / crate::domain::vo
//   （这两个模块在迁移后已不存在），因此不参与编译，仅留作历史参考。
pub mod auth_actix;
