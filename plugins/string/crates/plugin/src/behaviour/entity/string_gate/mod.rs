use inexor_rgf_behaviour::entity_behaviour;
use inexor_rgf_behaviour::PropertyObserverContainer;
use inexor_rgf_behaviour_api::behaviour_validator;
use inexor_rgf_behaviour_api::prelude::*;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive::ReactiveEntity;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

pub use function::StringGateFunction;
pub use function::STRING_GATES;

use inexor_rgf_model_result::ResultBooleanProperties::RESULT;
use inexor_rgf_model_string::StringGateProperties::LHS;
use inexor_rgf_model_string::StringGateProperties::RHS;

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
