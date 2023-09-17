#![feature(lazy_cell)]

use inexor_rgf_behaviour_api as behaviour_api;

pub use component::*;
pub use entity::*;
pub use relation::*;

pub mod component;
pub mod entity;
pub mod relation;

pub const NAMESPACE_DATE_TIME: &str = "date_time";
