use inexor_rgf_core_reactive::Operation;
use serde_json::json;
use serde_json::Value;

pub use function::LogicalOperationFunction;
pub use function::LOGICAL_OPERATIONS;

use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::model_logical::LogicalOperationProperties::LHS;
use crate::model_logical::LogicalOperationProperties::RESULT;
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
    LogicalOperation,
    LogicalOperationFactory,
    LogicalOperationFsm,
    LogicalOperationBehaviourTransitions,
    LogicalOperationValidator,
    f,
    LogicalOperationFunction
);

behaviour_validator!(LogicalOperationValidator, ReactiveEntityInstance, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for LogicalOperationBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self
            .reactive_instance
            .get(LHS)
            .and_then(|v| v.as_bool())
            .ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        self.reactive_instance.set(RESULT, json!(f(lhs)));
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for LogicalOperationBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |v: &Value| match v.as_bool() {
            Some(v) => {
                reactive_instance.set(RESULT, json!(f(v)));
            }
            None => {}
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for LogicalOperationBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for LogicalOperationBehaviourTransitions {}

impl Operation for LogicalOperationBehaviourTransitions {
    fn lhs(&self, value: Value) {
        if value.is_boolean() {
            self.reactive_instance.set(LHS, value);
        }
    }

    fn result(&self) -> Value {
        self.reactive_instance.get(RESULT).unwrap_or(RESULT.default_value())
    }
}
