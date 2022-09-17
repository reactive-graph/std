use lazy_static::lazy_static;

pub use behaviour::*;

pub mod behaviour;

lazy_static! {
    pub static ref STATE_COMPONENTS: Vec<&'static str> = vec!["state_array", "state_boolean", "state_number", "state_object", "state_string",]
        .into_iter()
        .collect();
}
