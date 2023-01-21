use std::sync::Arc;

use log::debug;
use log::trace;
use serde_json::Value;

use crate::model::ReactiveEntityInstance;
use crate::model_state::*;
use crate::reactive::behaviour_functions;

pub type StateDebuggerFunction = fn(Value, Arc<ReactiveEntityInstance>);

pub const FN_LOG_DEBUG: StateDebuggerFunction = |v, entity_instance| {
    debug!("{} {}", entity_instance.id, v);
};

pub const FN_LOG_TRACE: StateDebuggerFunction = |v, entity_instance| {
    trace!("{} {}", entity_instance.id, v);
};

behaviour_functions!(
    STATE_DEBUGGER_BEHAVIOURS,
    StateDebuggerFunction,
    NAMESPACE_STATE,
    ("state_debugger_debug", FN_LOG_DEBUG),
    ("state_debugger_trace", FN_LOG_TRACE)
);
