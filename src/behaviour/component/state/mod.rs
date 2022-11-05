use lazy_static::lazy_static;

pub use behaviour::*;

use crate::model::ComponentTypeId;

pub mod behaviour;

lazy_static! {
    pub static ref STATE_COMPONENTS: Vec<ComponentTypeId> = vec![
        ComponentTypeId::new_from_type("state", "state_array"),
        ComponentTypeId::new_from_type("state", "state_boolean"),
        ComponentTypeId::new_from_type("state", "state_number"),
        ComponentTypeId::new_from_type("state", "state_object"),
        ComponentTypeId::new_from_type("state", "state_string"),
    ]
    .into_iter()
    .collect();
}
