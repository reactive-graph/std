use serde_json::json;

pub use function::LogicalGateFunction;
pub use function::LOGICAL_GATES;

use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::model_logical::LogicalGateProperties::LHS;
use crate::model_logical::LogicalGateProperties::RHS;
use crate::model_result::ResultBooleanProperties::RESULT;
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
    LogicalGate,
    LogicalGateFactory,
    LogicalGateFsm,
    LogicalGateBehaviourTransitions,
    LogicalGateValidator,
    f,
    LogicalGateFunction
);

behaviour_validator!(LogicalGateValidator, ReactiveEntityInstance, LHS.as_ref(), RHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for LogicalGateBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self
            .reactive_instance
            .get(LHS)
            .and_then(|v| v.as_bool())
            .ok_or(BehaviourInitializationFailed {})?;
        let rhs = self
            .reactive_instance
            .get(RHS)
            .and_then(|v| v.as_bool())
            .ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        let r = f(lhs, rhs);
        self.reactive_instance.set(RESULT, json!(r));
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for LogicalGateBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |v| {
            if let Some(lhs) = v.as_bool() {
                if let Some(rhs) = reactive_instance.get(RHS).and_then(|v| v.as_bool()) {
                    reactive_instance.set(RESULT, json!(f(lhs, rhs)));
                }
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(RHS.as_ref(), move |v| {
            if let Some(rhs) = v.as_bool() {
                if let Some(lhs) = reactive_instance.get(LHS).and_then(|v| v.as_bool()) {
                    reactive_instance.set(RESULT, json!(f(lhs, rhs)));
                }
            }
        });
        Ok(())
    }
}
impl BehaviourShutdown<ReactiveEntityInstance> for LogicalGateBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for LogicalGateBehaviourTransitions {}
