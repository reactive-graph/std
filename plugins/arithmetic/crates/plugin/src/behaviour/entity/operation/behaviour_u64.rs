use crate::behaviour::as_u64;
use serde_json::json;
use serde_json::Value;

use crate::behaviour::entity::operation::function::ArithmeticOperationU64Function;

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
use crate::reactive::PropertyObserverContainer;

entity_behaviour!(
    ArithmeticOperationU64,
    ArithmeticOperationU64Factory,
    ArithmeticOperationU64Fsm,
    ArithmeticOperationU64BehaviourTransitions,
    ArithmeticOperationU64Validator,
    f,
    ArithmeticOperationU64Function
);

behaviour_validator!(ArithmeticOperationU64Validator, ReactiveEntityInstance, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for ArithmeticOperationU64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).and_then(as_u64).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        self.reactive_instance.set(RESULT, json!(f(lhs)));
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for ArithmeticOperationU64BehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        let f = self.f;
        self.property_observers
            .observe_with_handle(LHS.as_ref(), move |v: &Value| match as_u64(v.clone()) {
                Some(v) => {
                    reactive_instance.set(RESULT, json!(f(v)));
                }
                None => {}
            });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for ArithmeticOperationU64BehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for ArithmeticOperationU64BehaviourTransitions {}
