use serde_json::json;
use serde_json::Value;

use crate::behaviour::as_f64;
use crate::behaviour::entity::operation::function::NumericOperationF64Function;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::model_numeric::NumericOperationProperties::LHS;
use crate::model_result::ResultNumberF64Properties::RESULT;
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

entity_behaviour!(
    NumericOperationF64,
    NumericOperationF64Factory,
    NumericOperationF64Fsm,
    NumericOperationF64BehaviourTransitions,
    NumericOperationF64Validator,
    f,
    NumericOperationF64Function
);

behaviour_validator!(NumericOperationF64Validator, ReactiveEntityInstance, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for NumericOperationF64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).and_then(as_f64).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        let initial_value = f(lhs);
        self.reactive_instance.set(RESULT, json!(initial_value));
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for NumericOperationF64BehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |v: &Value| {
            if let Some(v) = as_f64(v.clone()) {
                reactive_instance.set(RESULT, json!(f(v)));
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for NumericOperationF64BehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for NumericOperationF64BehaviourTransitions {}
