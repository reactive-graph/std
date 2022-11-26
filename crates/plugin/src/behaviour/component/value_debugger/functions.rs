use log::debug;
use log::trace;
use serde_json::Value;

use crate::model_value::*;
use crate::reactive::behaviour_functions;

pub type ValueDebuggerFunction = fn(Value);

pub const FN_LOG_DEBUG: ValueDebuggerFunction = |v| {
    debug!("{}", v);
};

pub const FN_LOG_TRACE: ValueDebuggerFunction = |v| {
    trace!("{}", v);
};

behaviour_functions!(
    VALUE_DEBUGGER_BEHAVIOURS,
    ValueDebuggerFunction,
    NAMESPACE_VALUE,
    "value_debugger_debug",
    FN_LOG_DEBUG,
    "value_debugger_trace",
    FN_LOG_TRACE
);
