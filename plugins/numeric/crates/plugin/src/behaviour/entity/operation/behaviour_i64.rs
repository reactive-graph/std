use serde_json::json;
use serde_json::Value;

use crate::behaviour::as_i64;
use crate::behaviour::entity::operation::function::NumericOperationI64Function;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::model_numeric::NumericOperationProperties::LHS;
use crate::model_result::ResultNumberI64Properties::RESULT;
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
    NumericOperationI64,
    NumericOperationI64Factory,
    NumericOperationI64Fsm,
    NumericOperationI64BehaviourTransitions,
    NumericOperationI64Validator,
    f,
    NumericOperationI64Function
);

behaviour_validator!(NumericOperationI64Validator, ReactiveEntityInstance, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for NumericOperationI64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).and_then(as_i64).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        let initial_value = f(lhs);
        self.reactive_instance.set(RESULT, json!(initial_value));
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for NumericOperationI64BehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |v: &Value| {
            if let Some(v) = as_i64(v.clone()) {
                reactive_instance.set(RESULT, json!(f(v)));
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for NumericOperationI64BehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for NumericOperationI64BehaviourTransitions {}
