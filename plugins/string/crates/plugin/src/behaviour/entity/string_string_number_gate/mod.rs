use serde_json::json;
use serde_json::Value;

pub use function::StringStringNumberFunction;
pub use function::STRING_STRING_NUMBER_GATES;

use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::model_result::ResultNumberU64Properties::RESULT;
use crate::model_string::StringStringNumberGateProperties::LHS;
use crate::model_string::StringStringNumberGateProperties::RHS;
use crate::reactive::behaviour_validator;
use crate::reactive::entity_behaviour;
use crate::reactive::BehaviourConnect;
use crate::reactive::BehaviourConnectFailed;
use crate::reactive::BehaviourDisconnect;
use crate::reactive::BehaviourFsm;
use crate::reactive::BehaviourInit;
use crate::reactive::BehaviourInitializationFailed;
use crate::reactive::BehaviourShutdown;
use crate::reactive::BehaviourTransitions;
use crate::reactive::PropertyObserverContainer;

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

behaviour_validator!(StringStringNumberGateValidator, ReactiveEntityInstance, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for StringStringNumberGateBehaviourTransitions {
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

impl BehaviourConnect<ReactiveEntityInstance> for StringStringNumberGateBehaviourTransitions {
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

impl BehaviourShutdown<ReactiveEntityInstance> for StringStringNumberGateBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for StringStringNumberGateBehaviourTransitions {}
