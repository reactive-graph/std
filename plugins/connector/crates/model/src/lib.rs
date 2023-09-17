#![feature(lazy_cell)]

pub use component::*;

pub mod component;

use inexor_rgf_behaviour_api as behaviour_api;

pub const NAMESPACE_CONNECTOR: &str = "connector";
