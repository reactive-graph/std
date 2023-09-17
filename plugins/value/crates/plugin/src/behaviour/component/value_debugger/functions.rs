use std::sync::Arc;
use std::sync::LazyLock;

use inexor_rgf_behaviour::entity::EntityBehaviourFactoryCreator;
use inexor_rgf_behaviour::entity::EntityBehaviourFunctions;
use inexor_rgf_behaviour::entity::EntityBehaviourFunctionsStorage;
use log::debug;
use log::trace;
use serde_json::Value;

use inexor_rgf_model_value::NAMESPACE_VALUE;

use crate::behaviour::component::value_debugger::ValueDebuggerFactory;

pub type ValueDebuggerFunction = fn(Value);

const FN_LOG_DEBUG: ValueDebuggerFunction = |v| {
    debug!("{}", v);
};

const FN_LOG_TRACE: ValueDebuggerFunction = |v| {
    trace!("{}", v);
};

/// Factory
const FACTORY_CREATOR: EntityBehaviourFactoryCreator<ValueDebuggerFunction> = |ty, f| Arc::new(ValueDebuggerFactory::new(ty.clone(), f));

/// Defines behaviour types and their functions
/// Global, lazy, thread-safe, readonly, iterable
pub static VALUE_DEBUGGER_BEHAVIOURS: EntityBehaviourFunctionsStorage<ValueDebuggerFunction> = LazyLock::new(|| {
    EntityBehaviourFunctions::<ValueDebuggerFunction>::with_namespace(NAMESPACE_VALUE, FACTORY_CREATOR)
        .behaviour("value_debugger_debug", FN_LOG_DEBUG)
        .behaviour("value_debugger_trace", FN_LOG_TRACE)
        .get()
});

// LazyLock<BehaviourFunctionsReadOnlyView<Uuid, ReactiveEntity, ValueDebuggerFunction>>
// BehaviourFunctions::<Uuid, ReactiveEntity, ValueDebuggerFunction>

// entity_behaviour_functions!(VALUE_DEBUGGER_BEHAVIOURS, ValueDebuggerFunction, || {
//     BehaviourFunctions::with_namespace(NAMESPACE_VALUE, FACTORY_CREATOR)
//         .behaviour("value_debugger_debug", FN_LOG_DEBUG)
//         .behaviour("value_debugger_trace", FN_LOG_TRACE)
//         .get()
// });

// behaviour_functions!(
//     VALUE_DEBUGGER_BEHAVIOURS,
//     ValueDebuggerFunction,
//     NAMESPACE_VALUE,
//     ("value_debugger_debug", FN_LOG_DEBUG),
//     ("value_debugger_trace", FN_LOG_TRACE)
// );
