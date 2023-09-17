use inexor_rgf_behaviour::entity_behaviour;
use inexor_rgf_behaviour::PropertyObserverContainer;
use inexor_rgf_behaviour_api::behaviour_validator;
use inexor_rgf_behaviour_api::prelude::*;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive::ReactiveEntity;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_arithmetic::ArithmeticOperationProperties::LHS;
use inexor_rgf_model_arithmetic::ArithmeticOperationProperties::RESULT;

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
        self.property_observers
            .observe_with_handle(LHS.as_ref(), move |v: &Value| match as_i64(v.clone()) {
                Some(v) => {
                    reactive_instance.set(RESULT, json!(f(v)));
                }
                None => {}
            });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for ArithmeticOperationI64BehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ArithmeticOperationI64BehaviourTransitions {}
