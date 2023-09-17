use inexor_rgf_behaviour::entity_behaviour;
use inexor_rgf_behaviour::PropertyObserverContainer;
use inexor_rgf_behaviour_api::behaviour_validator;
use inexor_rgf_behaviour_api::prelude::*;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive::ReactiveEntity;
use serde_json::json;
use uuid::Uuid;

pub use function::LogicalGateFunction;

use inexor_rgf_model_logical::LogicalGateProperties::LHS;
use inexor_rgf_model_logical::LogicalGateProperties::RHS;
use inexor_rgf_model_result::ResultBooleanProperties::RESULT;

pub mod function;

entity_behaviour!(
    LogicalGate,
    LogicalGateFactory,
    LogicalGateFsm,
    LogicalGateBehaviourTransitions,
    LogicalGateValidator,
    f,
    LogicalGateFunction
);

behaviour_validator!(LogicalGateValidator, Uuid, ReactiveEntity, LHS.as_ref(), RHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for LogicalGateBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self
            .reactive_instance
            .get(LHS)
            .and_then(|v| v.as_bool())
            .ok_or(BehaviourInitializationFailed {})?;
        let rhs = self
            .reactive_instance
            .get(RHS)
            .and_then(|v| v.as_bool())
            .ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        let r = f(lhs, rhs);
        self.reactive_instance.set(RESULT, json!(r));
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for LogicalGateBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |v| {
            if let Some(lhs) = v.as_bool() {
                if let Some(rhs) = reactive_instance.get(RHS).and_then(|v| v.as_bool()) {
                    reactive_instance.set(RESULT, json!(f(lhs, rhs)));
                }
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(RHS.as_ref(), move |v| {
            if let Some(rhs) = v.as_bool() {
                if let Some(lhs) = reactive_instance.get(LHS).and_then(|v| v.as_bool()) {
                    reactive_instance.set(RESULT, json!(f(lhs, rhs)));
                }
            }
        });
        Ok(())
    }
}
impl BehaviourShutdown<Uuid, ReactiveEntity> for LogicalGateBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for LogicalGateBehaviourTransitions {}
