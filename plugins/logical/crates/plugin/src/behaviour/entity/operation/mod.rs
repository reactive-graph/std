use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

pub use function::LogicalOperationFunction;

use reactive_graph_model_logical::LogicalOperationProperties::LHS;
use reactive_graph_model_result::ResultBooleanProperties::RESULT;

pub mod function;

entity_behaviour!(
    LogicalOperation,
    LogicalOperationFactory,
    LogicalOperationFsm,
    LogicalOperationBehaviourTransitions,
    LogicalOperationValidator,
    f,
    LogicalOperationFunction
);

behaviour_validator!(LogicalOperationValidator, Uuid, ReactiveEntity, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for LogicalOperationBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self
            .reactive_instance
            .get(LHS)
            .and_then(|v| v.as_bool())
            .ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        self.reactive_instance.set(RESULT, json!(f(lhs)));
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for LogicalOperationBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |v: &Value| {
            if let Some(v) = v.as_bool() {
                reactive_instance.set(RESULT, json!(f(v)));
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for LogicalOperationBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for LogicalOperationBehaviourTransitions {}
