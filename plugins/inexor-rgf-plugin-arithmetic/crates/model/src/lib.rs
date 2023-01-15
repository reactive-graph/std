pub use component::*;
pub use entity::*;

pub mod component;
pub mod entity;

use inexor_rgf_core_model as model;

pub const NAMESPACE_ARITHMETIC: &str = "arithmetic";
pub const NAMESPACE_ARITHMETIC_F64: &str = "arithmetic_f64";
pub const NAMESPACE_ARITHMETIC_I64: &str = "arithmetic_i64";
pub const NAMESPACE_ARITHMETIC_U64: &str = "arithmetic_u64";
