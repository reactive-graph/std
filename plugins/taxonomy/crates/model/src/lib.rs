#![feature(lazy_cell)]

pub use component::*;
pub use entity::*;
pub use relation::*;

pub mod component;
pub mod entity;
pub mod relation;

pub const NAMESPACE_TAXONOMY: &str = "taxonomy";
