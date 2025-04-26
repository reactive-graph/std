use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

pub use function::STRING_NUMBER_OPERATIONS;
pub use function::StringNumberFunction;

use reactive_graph_std_result_model::ResultNumberU64Properties::RESULT;
use reactive_graph_std_string_model::StringNumberOperationProperties::LHS;

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
