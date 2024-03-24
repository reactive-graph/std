use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_arithmetic::ArithmeticOperationProperties::LHS;
use inexor_rgf_model_arithmetic::ArithmeticOperationProperties::RESULT;

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
