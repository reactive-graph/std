use std::collections::HashMap;

use log::debug;
use log::trace;
use serde_json::Value;

use lazy_static::lazy_static;

pub type ValueDebuggerFunction = fn(Value);

pub const FN_LOG_DEBUG: ValueDebuggerFunction = |v| {
    debug!("{}", v);
};

pub const FN_LOG_TRACE: ValueDebuggerFunction = |v| {
    trace!("{}", v);
};

lazy_static! {
    pub static ref VALUE_DEBUGGERS: HashMap<&'static str, ValueDebuggerFunction> =
        vec![("value_debugger_debug", FN_LOG_DEBUG), ("value_debugger_trace", FN_LOG_TRACE),]
            .into_iter()
            .collect();
}
