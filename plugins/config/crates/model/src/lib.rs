#![feature(lazy_cell)]

pub use component::config_file::*;
pub use entity::config_file::*;

pub mod component;
pub mod entity;

use inexor_rgf_behaviour_api as behaviour_api;

pub const NAMESPACE_CONFIG: &str = "config";
