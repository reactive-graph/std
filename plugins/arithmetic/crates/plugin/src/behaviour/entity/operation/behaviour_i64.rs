use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_std_arithmetic_model::ArithmeticOperationProperties::LHS;
use reactive_graph_std_arithmetic_model::ArithmeticOperationProperties::RESULT;

use crate::behaviour::as_i64;
use crate::behaviour::entity::operation::function::ArithmeticOperationI64Function;

entity_behaviour!(
    ArithmeticOperationI64,
    ArithmeticOperationI64Factory,
    ArithmeticOperationI64Fsm,
    ArithmeticOperationI64BehaviourTransitions,
    ArithmeticOperationI64Validator,
    f,
    ArithmeticOperationI64Function
);

behaviour_validator!(ArithmeticOperationI64Validator, Uuid, ReactiveEntity, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ArithmeticOperationI64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).and_then(as_i64).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        self.reactive_instance.set(RESULT, json!(f(lhs)));
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ArithmeticOperationI64BehaviourTransitions {
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

impl BehaviourShutdown<Uuid, ReactiveEntity> for ArithmeticOperationI64BehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ArithmeticOperationI64BehaviourTransitions {}
