use serde_json::json;

use crate::behaviour::entity::gate::function::ComparisonGateFunction;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::model_comparison::ComparisonGateProperties::LHS;
use crate::model_comparison::ComparisonGateProperties::RHS;
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

entity_behaviour!(
    ComparisonGate,
    ComparisonGateFactory,
    ComparisonGateFsm,
    ComparisonGateBehaviourTransitions,
    ComparisonGateValidator,
    f,
    ComparisonGateFunction
);

behaviour_validator!(ComparisonGateValidator, ReactiveEntityInstance, LHS.as_ref(), RHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for ComparisonGateBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).ok_or(BehaviourInitializationFailed {})?;
        let rhs = self.reactive_instance.get(RHS).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        let initial_value = f(&lhs, &rhs);
        self.reactive_instance.set(RESULT, initial_value);
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for ComparisonGateBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |lhs| {
            if let Some(rhs) = reactive_instance.get(RHS) {
                reactive_instance.set(RESULT, json!(f(lhs, &rhs)));
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(RHS.as_ref(), move |rhs| {
            if let Some(lhs) = reactive_instance.get(LHS) {
                reactive_instance.set(RESULT, json!(f(&lhs, rhs)));
            }
        });
        Ok(())
    }
}
impl BehaviourShutdown<ReactiveEntityInstance> for ComparisonGateBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for ComparisonGateBehaviourTransitions {}
