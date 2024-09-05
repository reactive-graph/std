use std::sync::Arc;
use std::sync::LazyLock;

use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFactoryCreator;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctions;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctionsStorage;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use log::debug;
use log::trace;
use serde_json::Value;

use reactive_graph_model_state::NAMESPACE_STATE;

use crate::behaviour::component::state_debugger::StateDebuggerFactory;

pub type StateDebuggerFunction = fn(Value, ReactiveEntity);

const FN_LOG_DEBUG: StateDebuggerFunction = |v, entity_instance| {
    debug!("{} {}", entity_instance.id, v);
};

const FN_LOG_TRACE: StateDebuggerFunction = |v, entity_instance| {
    trace!("{} {}", entity_instance.id, v);
};

const FACTORY_CREATOR: EntityBehaviourFactoryCreator<StateDebuggerFunction> = |ty, f| Arc::new(StateDebuggerFactory::new(ty.clone(), f));

/// Defines behaviour types and their functions
/// Global, lazy, thread-safe, readonly, iterable
pub static STATE_DEBUGGER_BEHAVIOURS: EntityBehaviourFunctionsStorage<StateDebuggerFunction> = LazyLock::new(|| {
    EntityBehaviourFunctions::<StateDebuggerFunction>::with_namespace(NAMESPACE_STATE, FACTORY_CREATOR)
        .behaviour("state_debugger_debug", FN_LOG_DEBUG)
        .behaviour("state_debugger_trace", FN_LOG_TRACE)
        .get()
});

// behaviour_functions!(
//     STATE_DEBUGGER_BEHAVIOURS,
//     StateDebuggerFunction,
//     NAMESPACE_STATE,
//     ("state_debugger_debug", FN_LOG_DEBUG),
//     ("state_debugger_trace", FN_LOG_TRACE)
// );
