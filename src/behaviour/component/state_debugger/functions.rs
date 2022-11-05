use std::collections::HashMap;
use std::sync::Arc;

use lazy_static::lazy_static;
use log::debug;
use log::trace;
use serde_json::Value;

use crate::model::ComponentTypeId;
use crate::model::ReactiveEntityInstance;

pub type StateDebuggerFunction = fn(Value, Arc<ReactiveEntityInstance>);

pub const FN_LOG_DEBUG: StateDebuggerFunction = |v, entity_instance| {
    debug!("{} {}", entity_instance.id, v);
};

pub const FN_LOG_TRACE: StateDebuggerFunction = |v, entity_instance| {
    trace!("{} {}", entity_instance.id, v);
};

lazy_static! {
    pub static ref STATE_DEBUGGERS: HashMap<ComponentTypeId, StateDebuggerFunction> = vec![
        (ComponentTypeId::new_from_type("state", "state_debugger_debug"), FN_LOG_DEBUG),
        (ComponentTypeId::new_from_type("state", "state_debugger_trace"), FN_LOG_TRACE),
    ]
    .into_iter()
    .collect();
}
