pub use component::state::*;
pub use entity_type::state::*;

pub mod component;
pub mod entity_type;

use reactive_graph_model_value as model_value;

pub const NAMESPACE_STATE: &str = "state";
