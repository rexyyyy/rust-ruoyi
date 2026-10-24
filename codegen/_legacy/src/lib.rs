#![allow(unused_variables)] //允许未使用的变量
#![allow(unused_must_use)]
pub mod error;
extern crate log;
extern crate rbatis;
extern crate crypto;//rust-crypto
// extern crate bcrypt;


#[macro_use]
pub mod config;
pub mod codegen;
pub mod util;
