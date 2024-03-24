use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

pub use function::StringStringNumberFunction;
pub use function::STRING_STRING_NUMBER_GATES;

use inexor_rgf_model_result::ResultNumberU64Properties::RESULT;
use inexor_rgf_model_string::StringStringNumberGateProperties::LHS;
use inexor_rgf_model_string::StringStringNumberGateProperties::RHS;

pub mod function;

entity_behaviour!(
    StringStringNumberGate,
    StringStringNumberGateFactory,
    StringStringNumberGateFsm,
    StringStringNumberGateBehaviourTransitions,
    StringStringNumberGateValidator,
    f,
    StringStringNumberFunction
);

behaviour_validator!(StringStringNumberGateValidator, Uuid, ReactiveEntity, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for StringStringNumberGateBehaviourTransitions {
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

impl BehaviourConnect<Uuid, ReactiveEntity> for StringStringNumberGateBehaviourTransitions {
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

impl BehaviourShutdown<Uuid, ReactiveEntity> for StringStringNumberGateBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for StringStringNumberGateBehaviourTransitions {}
