use crate::behaviour::as_f64;
use serde_json::json;

use crate::behaviour::entity::gate::function::ArithmeticGateF64Function;

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
    ArithmeticGateF64,
    ArithmeticGateF64Factory,
    ArithmeticGateF64Fsm,
    ArithmeticGateF64BehaviourTransitions,
    ArithmeticGateF64Validator,
    f,
    ArithmeticGateF64Function
);

behaviour_validator!(ArithmeticGateF64Validator, ReactiveEntityInstance, LHS.as_ref(), RHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for ArithmeticGateF64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).and_then(as_f64).ok_or(BehaviourInitializationFailed {})?;
        let rhs = self.reactive_instance.get(RHS).and_then(as_f64).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        let r = f(lhs, rhs);
        self.reactive_instance.set(RESULT, json!(r));
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for ArithmeticGateF64BehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |v| match v.as_f64() {
            Some(lhs) => match reactive_instance.get(RHS).and_then(as_f64) {
                Some(rhs) => {
                    reactive_instance.set(RESULT, json!(f(lhs, rhs)));
                }
                None => {}
            },
            None => {}
        });
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(RHS.as_ref(), move |v| match v.as_f64() {
            Some(rhs) => match reactive_instance.get(LHS).and_then(as_f64) {
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
impl BehaviourShutdown<ReactiveEntityInstance> for ArithmeticGateF64BehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for ArithmeticGateF64BehaviourTransitions {}
