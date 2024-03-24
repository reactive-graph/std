use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use log::debug;
use serde_json::Value;
use uuid::Uuid;

pub use functions::*;
use inexor_rgf_graph::PropertyInstanceGetter;
use inexor_rgf_graph::PropertyInstanceSetter;
use inexor_rgf_model_state::StateProperties;

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
