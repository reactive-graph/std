use log::debug;
use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use uuid::Uuid;

pub use functions::*;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_std_state_model::StateProperties;

pub mod functions;

entity_behaviour!(
    StateDebugger,
    StateDebuggerFactory,
    StateDebuggerFsm,
    StateDebuggerBehaviourTransitions,
    StateDebuggerValidator,
    f,
    StateDebuggerFunction
);

behaviour_validator!(StateDebuggerValidator, Uuid, ReactiveEntity, StateProperties::STATE.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for StateDebuggerBehaviourTransitions {}

impl BehaviourConnect<Uuid, ReactiveEntity> for StateDebuggerBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        let f = self.f;
        self.property_observers
            .observe_with_handle(StateProperties::STATE.as_ref(), move |v: &Value| f(v.clone(), reactive_instance.clone()));
        debug!("Starting debugging of {}[{}]", &self.property_observers.reactive_instance, StateProperties::STATE.as_ref());
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for StateDebuggerBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for StateDebuggerBehaviourTransitions {}
