pub use component::*;
pub use entity::*;

pub mod component;
pub mod entity;

use inexor_rgf_core_model as model;
use inexor_rgf_model_base as model_base;
use inexor_rgf_model_file as model_file;
use inexor_rgf_model_result as model_result;
use inexor_rgf_model_runtime as model_runtime;

pub const NAMESPACE_JSON: &str = "json";
