#![feature(lazy_cell)]

pub use component::*;
pub use entity::*;

pub mod component;
pub mod entity;

pub const NAMESPACE_STRING: &str = "string";
