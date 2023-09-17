#![feature(lazy_cell)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]

pub use component::value::*;
pub use entity_type::value::*;

pub mod component;
pub mod entity_type;

pub const NAMESPACE_VALUE: &str = "value";
