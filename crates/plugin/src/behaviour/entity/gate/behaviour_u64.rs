use crate::behaviour::as_u64;
use serde_json::json;

use crate::behaviour::entity::gate::function::ArithmeticGateU64Function;

use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::model_arithmetic::ArithmeticGateProperties::LHS;
use crate::model_arithmetic::ArithmeticGateProperties::RESULT;
use crate::model_arithmetic::ArithmeticGateProperties::RHS;
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
    ArithmeticGateU64,
    ArithmeticGateU64Factory,
    ArithmeticGateU64Fsm,
    ArithmeticGateU64BehaviourTransitions,
    ArithmeticGateU64Validator,
    f,
    ArithmeticGateU64Function
);

behaviour_validator!(ArithmeticGateU64Validator, ReactiveEntityInstance, LHS.as_ref(), RHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for ArithmeticGateU64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).and_then(as_u64).ok_or(BehaviourInitializationFailed {})?;
        let rhs = self.reactive_instance.get(RHS).and_then(as_u64).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        let r = f(lhs, rhs);
        self.reactive_instance.set(RESULT, json!(r));
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for ArithmeticGateU64BehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |v| match as_u64(v.clone()) {
            Some(lhs) => match reactive_instance.get(RHS).and_then(as_u64) {
                Some(rhs) => {
                    reactive_instance.set(RESULT, json!(f(lhs, rhs)));
                }
                None => {}
            },
            None => {}
        });
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(RHS.as_ref(), move |v| match as_u64(v.clone()) {
            Some(rhs) => match reactive_instance.get(LHS).and_then(as_u64) {
                Some(lhs) => {
                    reactive_instance.set(RESULT, json!(f(lhs, rhs)));
                }
                None => {}
            },
            None => {}
        });

        Ok(())
    }
}
impl BehaviourShutdown<ReactiveEntityInstance> for ArithmeticGateU64BehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for ArithmeticGateU64BehaviourTransitions {}
