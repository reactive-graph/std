use serde_json::json;
use serde_json::Value;

use crate::behaviour::as_f64;
use crate::behaviour::entity::operation::function::ArithmeticOperationF64Function;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::model_arithmetic::ArithmeticOperationProperties::LHS;
use crate::model_arithmetic::ArithmeticOperationProperties::RESULT;
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
use crate::reactive::Operation;
use crate::reactive::PropertyObserverContainer;

entity_behaviour!(
    ArithmeticOperationF64,
    ArithmeticOperationF64Factory,
    ArithmeticOperationF64Fsm,
    ArithmeticOperationF64BehaviourTransitions,
    ArithmeticOperationF64Validator,
    f,
    ArithmeticOperationF64Function
);

behaviour_validator!(ArithmeticOperationF64Validator, ReactiveEntityInstance, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for ArithmeticOperationF64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).and_then(as_f64).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        self.reactive_instance.set(RESULT, json!(f(lhs)));
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for ArithmeticOperationF64BehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        let f = self.f;
        self.property_observers
            .observe_with_handle(LHS.as_ref(), move |v: &Value| match as_f64(v.clone()) {
                Some(v) => {
                    reactive_instance.set(RESULT, json!(f(v)));
                }
                None => {}
            });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for ArithmeticOperationF64BehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for ArithmeticOperationF64BehaviourTransitions {}

impl Operation for ArithmeticOperationF64 {
    fn lhs(&self, value: Value) {
        if value.is_f64() {
            self.reactive_instance.set(LHS, value);
        }
    }

    fn result(&self) -> Value {
        self.reactive_instance.get(RESULT).unwrap_or(RESULT.default_value())
    }
}
