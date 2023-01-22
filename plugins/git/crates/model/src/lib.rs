pub use component::*;
pub use entity::*;

pub mod component;
pub mod entity;

use inexor_rgf_core_model as model;
use inexor_rgf_model_base as model_base;
use inexor_rgf_model_file as model_file;
use inexor_rgf_model_http as model_http;
use inexor_rgf_model_trigger as model_trigger;

pub const NAMESPACE_GIT: &str = "git";
