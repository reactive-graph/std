use lazy_static::lazy_static;

pub use behaviour::*;

use crate::model::BehaviourTypeId;

pub mod behaviour;

lazy_static! {
    pub static ref STATE_BEHAVIOURS: Vec<BehaviourTypeId> = vec![
        BehaviourTypeId::new_from_type("state", "state_array"),
        BehaviourTypeId::new_from_type("state", "state_boolean"),
        BehaviourTypeId::new_from_type("state", "state_number"),
        BehaviourTypeId::new_from_type("state", "state_object"),
        BehaviourTypeId::new_from_type("state", "state_string"),
    ]
    .into_iter()
    .collect();
}
