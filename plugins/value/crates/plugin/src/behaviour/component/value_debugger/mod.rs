use log::debug;
use reactive_graph_behaviour_model_api::BehaviourConnect;
use reactive_graph_behaviour_model_api::BehaviourConnectFailed;
use reactive_graph_behaviour_model_api::BehaviourDisconnect;
use reactive_graph_behaviour_model_api::BehaviourFsm;
use reactive_graph_behaviour_model_api::BehaviourInit;
use reactive_graph_behaviour_model_api::BehaviourShutdown;
use reactive_graph_behaviour_model_api::BehaviourTransitions;
use reactive_graph_behaviour_model_api::PropertyObserverContainer;
use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use uuid::Uuid;

pub use functions::*;
use reactive_graph_std_value_model::ValueProperties;

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
