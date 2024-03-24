use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use serde_json::json;
use uuid::Uuid;

use inexor_rgf_model_arithmetic::ArithmeticGateProperties::RHS;
use inexor_rgf_model_arithmetic::ArithmeticOperationProperties::LHS;
use inexor_rgf_model_arithmetic::ArithmeticOperationProperties::RESULT;

use crate::behaviour::as_u64;
use crate::behaviour::entity::gate::function::ArithmeticGateU64Function;

entity_behaviour!(
    ArithmeticGateU64,
    ArithmeticGateU64Factory,
    ArithmeticGateU64Fsm,
    ArithmeticGateU64BehaviourTransitions,
    ArithmeticGateU64Validator,
    f,
    ArithmeticGateU64Function
);

behaviour_validator!(ArithmeticGateU64Validator, Uuid, ReactiveEntity, LHS.as_ref(), RHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ArithmeticGateU64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).and_then(as_u64).ok_or(BehaviourInitializationFailed {})?;
        let rhs = self.reactive_instance.get(RHS).and_then(as_u64).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        let r = f(lhs, rhs);
        self.reactive_instance.set(RESULT, json!(r));
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ArithmeticGateU64BehaviourTransitions {
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
impl BehaviourShutdown<Uuid, ReactiveEntity> for ArithmeticGateU64BehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ArithmeticGateU64BehaviourTransitions {}
