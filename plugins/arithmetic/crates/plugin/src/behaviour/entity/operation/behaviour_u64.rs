use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_model_arithmetic::ArithmeticOperationProperties::LHS;
use reactive_graph_model_arithmetic::ArithmeticOperationProperties::RESULT;

use crate::behaviour::as_u64;
use crate::behaviour::entity::operation::function::ArithmeticOperationU64Function;

entity_behaviour!(
    ArithmeticOperationU64,
    ArithmeticOperationU64Factory,
    ArithmeticOperationU64Fsm,
    ArithmeticOperationU64BehaviourTransitions,
    ArithmeticOperationU64Validator,
    f,
    ArithmeticOperationU64Function
);

behaviour_validator!(ArithmeticOperationU64Validator, Uuid, ReactiveEntity, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ArithmeticOperationU64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).and_then(as_u64).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        self.reactive_instance.set(RESULT, json!(f(lhs)));
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ArithmeticOperationU64BehaviourTransitions {
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

impl BehaviourShutdown<Uuid, ReactiveEntity> for ArithmeticOperationU64BehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ArithmeticOperationU64BehaviourTransitions {}
