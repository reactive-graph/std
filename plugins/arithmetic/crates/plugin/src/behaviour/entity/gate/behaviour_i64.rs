use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_model_arithmetic::ArithmeticGateProperties::LHS;
use reactive_graph_model_arithmetic::ArithmeticGateProperties::RESULT;
use reactive_graph_model_arithmetic::ArithmeticGateProperties::RHS;

use crate::behaviour::as_i64;
use crate::behaviour::entity::gate::function::ArithmeticGateI64Function;

entity_behaviour!(
    ArithmeticGateI64,
    ArithmeticGateI64Factory,
    ArithmeticGateI64Fsm,
    ArithmeticGateI64BehaviourTransitions,
    ArithmeticGateI64Validator,
    f,
    ArithmeticGateI64Function
);

behaviour_validator!(ArithmeticGateI64Validator, Uuid, ReactiveEntity, LHS.as_ref(), RHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ArithmeticGateI64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).and_then(as_i64).ok_or(BehaviourInitializationFailed {})?;
        let rhs = self.reactive_instance.get(RHS).and_then(as_i64).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        let r = f(lhs, rhs);
        self.reactive_instance.set(RESULT, json!(r));
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ArithmeticGateI64BehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |v| {
            if let Some(lhs) = as_i64(v.clone()) {
                if let Some(rhs) = reactive_instance.get(RHS).and_then(as_i64) {
                    reactive_instance.set(RESULT, json!(f(lhs, rhs)));
                }
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(RHS.as_ref(), move |v| {
            if let Some(rhs) = as_i64(v.clone()) {
                if let Some(lhs) = reactive_instance.get(LHS).and_then(as_i64) {
                    reactive_instance.set(RESULT, json!(f(lhs, rhs)));
                }
            }
        });

        Ok(())
    }
}
impl BehaviourShutdown<Uuid, ReactiveEntity> for ArithmeticGateI64BehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ArithmeticGateI64BehaviourTransitions {}
