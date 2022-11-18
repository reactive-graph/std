use std::collections::HashMap;

use lazy_static::lazy_static;
use log::debug;
use log::trace;
use serde_json::Value;

use crate::model::BehaviourTypeId;
use crate::model_value::*;

pub type ValueDebuggerFunction = fn(Value);

pub const FN_LOG_DEBUG: ValueDebuggerFunction = |v| {
    debug!("{}", v);
};

pub const FN_LOG_TRACE: ValueDebuggerFunction = |v| {
    trace!("{}", v);
};

lazy_static! {
    pub static ref VALUE_DEBUGGER_BEHAVIOURS: HashMap<BehaviourTypeId, ValueDebuggerFunction> = vec![
        (BehaviourTypeId::new_from_type(NAMESPACE_VALUE, "value_debugger_debug"), FN_LOG_DEBUG),
        (BehaviourTypeId::new_from_type(NAMESPACE_VALUE, "value_debugger_trace"), FN_LOG_TRACE),
    ]
    .into_iter()
    .collect();
}
