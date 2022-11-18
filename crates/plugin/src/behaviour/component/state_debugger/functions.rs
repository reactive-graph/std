use std::collections::HashMap;
use std::sync::Arc;

use lazy_static::lazy_static;
use log::debug;
use log::trace;
use serde_json::Value;

use crate::model::BehaviourTypeId;
use crate::model::ReactiveEntityInstance;
use crate::model_value::*;

pub type StateDebuggerFunction = fn(Value, Arc<ReactiveEntityInstance>);

pub const FN_LOG_DEBUG: StateDebuggerFunction = |v, entity_instance| {
    debug!("{} {}", entity_instance.id, v);
};

pub const FN_LOG_TRACE: StateDebuggerFunction = |v, entity_instance| {
    trace!("{} {}", entity_instance.id, v);
};

lazy_static! {
    pub static ref STATE_DEBUGGER_BEHAVIOURS: HashMap<BehaviourTypeId, StateDebuggerFunction> = vec![
        (BehaviourTypeId::new_from_type(NAMESPACE_STATE, "state_debugger_debug"), FN_LOG_DEBUG),
        (BehaviourTypeId::new_from_type(NAMESPACE_STATE, "state_debugger_trace"), FN_LOG_TRACE),
    ]
    .into_iter()
    .collect();
}
