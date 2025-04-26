use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

pub use function::STRING_GATES;
pub use function::StringGateFunction;

use reactive_graph_std_result_model::ResultBooleanProperties::RESULT;
use reactive_graph_std_string_model::StringGateProperties::LHS;
use reactive_graph_std_string_model::StringGateProperties::RHS;

pub mod function;

entity_behaviour!(
    StringGate,
    StringGateFactory,
    StringGateFsm,
    StringGateBehaviourTransitions,
    StringGateValidator,
    f,
    StringGateFunction
);

behaviour_validator!(StringGateValidator, Uuid, ReactiveEntity, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for StringGateBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self
            .reactive_instance
            .get(LHS)
            .and_then(|lhs| lhs.as_str().map(|lhs| lhs.to_string()))
            .ok_or(BehaviourInitializationFailed {})?;
        let rhs = self
            .reactive_instance
            .get(RHS)
            .and_then(|rhs| rhs.as_str().map(|rhs| rhs.to_string()))
            .ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        self.reactive_instance.set(RESULT, json!(f(lhs, rhs)));
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for StringGateBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |lhs: &Value| {
            if let Some(lhs) = lhs.as_str().map(|lhs| lhs.to_string()) {
                if let Some(rhs) = reactive_instance.get(RHS).and_then(|rhs| rhs.as_str().map(|rhs| rhs.to_string())) {
                    reactive_instance.set(RESULT, json!(f(lhs, rhs)));
                }
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(RHS.as_ref(), move |rhs: &Value| {
            if let Some(rhs) = rhs.as_str().map(|rhs| rhs.to_string()) {
                if let Some(lhs) = reactive_instance.get(LHS).and_then(|lhs| lhs.as_str().map(|lhs| lhs.to_string())) {
                    reactive_instance.set(RESULT, json!(f(lhs, rhs)));
                }
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for StringGateBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for StringGateBehaviourTransitions {}
