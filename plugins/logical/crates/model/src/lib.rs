#![feature(lazy_cell)]

pub use component::*;
pub use entity::*;

pub mod component;
pub mod entity;

use inexor_rgf_behaviour_api as behaviour_api;

pub const NAMESPACE_LOGICAL: &str = "logical";
