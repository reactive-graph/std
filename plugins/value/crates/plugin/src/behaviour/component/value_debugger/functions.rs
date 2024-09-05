use std::sync::Arc;
use std::sync::LazyLock;

use log::debug;
use log::trace;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFactoryCreator;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctions;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctionsStorage;
use serde_json::Value;

use reactive_graph_model_value::NAMESPACE_VALUE;

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
