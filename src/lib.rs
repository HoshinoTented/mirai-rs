//! [mirai](https://github.com/mamoe/mirai) is a protocol library for Tencent QQ, you can use it to write a QQ Bot or other things.
//!
//! mirai-rs is a library that base on a mirai-api-http server, you may should set up a mirai-api-http server first.
//!
//! First, you need to make a connection to the server, please see: [mod session].
//!
//! After authorization and verification, you can use [mod message] to receive and send messages.
//!
//! Have a good time!

pub mod session;
pub mod error;
pub mod message;
pub mod member_list;
pub mod common;
pub mod group;
pub mod image;

pub type Target = u64;
pub type Code = u16;