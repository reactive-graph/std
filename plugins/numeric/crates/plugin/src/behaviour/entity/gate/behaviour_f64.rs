use crate::behaviour::as_f64;
use crate::behaviour::entity::gate::function::NumericGateF64Function;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::model_numeric::NumericGateProperties::LHS;
use crate::model_numeric::NumericGateProperties::RHS;
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
    NumericGateF64,
    NumericGateF64Factory,
    NumericGateF64Fsm,
    NumericGateF64BehaviourTransitions,
    NumericGateF64Validator,
    f,
    NumericGateF64Function
);

behaviour_validator!(NumericGateF64Validator, ReactiveEntityInstance, LHS.as_ref(), RHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for NumericGateF64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).and_then(as_f64).ok_or(BehaviourInitializationFailed {})?;
        let rhs = self.reactive_instance.get(RHS).and_then(as_f64).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        let initial_value = f(lhs, rhs);
        self.reactive_instance.set(RESULT, initial_value);
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for NumericGateF64BehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |v| {
            if let Some(lhs) = v.as_f64() {
                if let Some(rhs) = reactive_instance.get(RHS).and_then(as_f64) {
                    reactive_instance.set(RESULT, f(lhs, rhs));
                }
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(RHS.as_ref(), move |v| {
            if let Some(rhs) = v.as_f64() {
                if let Some(lhs) = reactive_instance.get(LHS).and_then(as_f64) {
                    reactive_instance.set(RESULT, f(lhs, rhs));
                }
            }
        });

        Ok(())
    }
}
impl BehaviourShutdown<ReactiveEntityInstance> for NumericGateF64BehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for NumericGateF64BehaviourTransitions {}
