pub use component::*;
pub use entity::*;
pub use relation::*;

pub mod component;
pub mod entity;
pub mod relation;

use inexor_rgf_core_model as model;
use inexor_rgf_model_base as model_base;

pub const NAMESPACE_TAXONOMY: &str = "taxonomy";
