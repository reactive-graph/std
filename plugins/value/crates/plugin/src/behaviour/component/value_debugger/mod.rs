use inexor_rgf_behaviour::entity_behaviour;
use inexor_rgf_behaviour_api::behaviour_validator;
use inexor_rgf_behaviour_api::prelude::*;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive::ReactiveEntity;
use log::debug;
use serde_json::Value;
use uuid::Uuid;

pub use functions::*;
use inexor_rgf_model_value::ValueProperties;

pub mod functions;

entity_behaviour!(
    ValueDebugger,
    ValueDebuggerFactory,
    ValueDebuggerFsm,
    ValueDebuggerBehaviourTransitions,
    ValueDebuggerValidator,
    f,
    ValueDebuggerFunction
);

behaviour_validator!(ValueDebuggerValidator, Uuid, ReactiveEntity, ValueProperties::VALUE.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ValueDebuggerBehaviourTransitions {}

impl BehaviourConnect<Uuid, ReactiveEntity> for ValueDebuggerBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let f = self.f;
        self.property_observers
            .observe_with_handle(ValueProperties::VALUE.as_ref(), move |v: &Value| f(v.clone()));
        debug!("Starting debugging of {}[{}]", &self.property_observers.reactive_instance, ValueProperties::VALUE.as_ref());
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for ValueDebuggerBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ValueDebuggerBehaviourTransitions {}
