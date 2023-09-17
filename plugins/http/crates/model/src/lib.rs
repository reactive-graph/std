#![feature(lazy_cell)]

pub use component::http::*;
pub use component::json_rpc::*;
pub use component::request::*;
pub use component::response::*;
pub use component::url::*;
pub use entity::http::*;
pub use entity::json_rpc::*;

pub mod component;
pub mod entity;

use inexor_rgf_behaviour_api as behaviour_api;

pub const NAMESPACE_HTTP: &str = "http";
