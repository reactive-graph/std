use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

pub use function::StringNumberFunction;
pub use function::STRING_NUMBER_OPERATIONS;

use inexor_rgf_model_result::ResultNumberU64Properties::RESULT;
use inexor_rgf_model_string::StringNumberOperationProperties::LHS;

pub mod function;

entity_behaviour!(
    StringNumberOperation,
    StringNumberOperationFactory,
    StringNumberOperationFsm,
    StringNumberOperationBehaviourTransitions,
    StringNumberOperationValidator,
    f,
    StringNumberFunction
);

behaviour_validator!(StringNumberOperationValidator, Uuid, ReactiveEntity, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for StringNumberOperationBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self
            .reactive_instance
            .get(LHS)
            .and_then(|lhs| lhs.as_str().map(|lhs| lhs.to_string()))
            .ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        self.reactive_instance.set(RESULT, json!(f(lhs)));
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for StringNumberOperationBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |lhs: &Value| {
            if let Some(lhs) = lhs.as_str().map(|lhs| lhs.to_string()) {
                reactive_instance.set(RESULT, json!(f(lhs)));
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for StringNumberOperationBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for StringNumberOperationBehaviourTransitions {}
