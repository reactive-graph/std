#![feature(lazy_cell)]

pub use component::load_json::*;
pub use component::save_json::*;
pub use entity::array_contains::*;
pub use entity::array_get_by_index::*;
pub use entity::array_length::*;
pub use entity::array_pop::*;
pub use entity::array_push::*;
pub use entity::array_reverse::*;
pub use entity::load_json::*;
pub use entity::object_get_property::*;
pub use entity::object_keys::*;
pub use entity::object_remove_property::*;
pub use entity::object_set_property::*;
pub use entity::save_json::*;

pub mod component;
pub mod entity;

pub const NAMESPACE_JSON: &str = "json";
