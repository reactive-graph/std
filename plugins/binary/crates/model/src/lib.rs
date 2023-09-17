#![feature(lazy_cell)]

pub use component::binary_data::*;
pub use component::load_binary_data::*;
pub use component::save_binary_data::*;
pub use entity::load_binary_data::*;
pub use entity::save_binary_data::*;

pub mod component;
pub mod entity;

use inexor_rgf_behaviour_api as behaviour_api;

pub const NAMESPACE_BINARY: &str = "binary";
