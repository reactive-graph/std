#![feature(lazy_cell)]

pub use component::repository::*;
pub use component::transfer_progress::*;
pub use entity::repository::*;

pub mod component;
pub mod entity;

use inexor_rgf_behaviour_api as behaviour_api;

pub const NAMESPACE_GIT: &str = "git";
