use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use serde_json::json;
use uuid::Uuid;

use inexor_rgf_model_comparison::ComparisonGateProperties::LHS;
use inexor_rgf_model_comparison::ComparisonGateProperties::RHS;
use inexor_rgf_model_result::ResultBooleanProperties::RESULT;

use crate::behaviour::entity::gate::function::ComparisonGateFunction;

entity_behaviour!(
    ComparisonGate,
    ComparisonGateFactory,
    ComparisonGateFsm,
    ComparisonGateBehaviourTransitions,
    ComparisonGateValidator,
    f,
    ComparisonGateFunction
);

behaviour_validator!(ComparisonGateValidator, Uuid, ReactiveEntity, LHS.as_ref(), RHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ComparisonGateBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).ok_or(BehaviourInitializationFailed {})?;
        let rhs = self.reactive_instance.get(RHS).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        let initial_value = f(&lhs, &rhs);
        self.reactive_instance.set(RESULT, initial_value);
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ComparisonGateBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |lhs| {
            if let Some(rhs) = reactive_instance.get(RHS) {
                reactive_instance.set(RESULT, json!(f(lhs, &rhs)));
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(RHS.as_ref(), move |rhs| {
            if let Some(lhs) = reactive_instance.get(LHS) {
                reactive_instance.set(RESULT, json!(f(&lhs, rhs)));
            }
        });
        Ok(())
    }
}
impl BehaviourShutdown<Uuid, ReactiveEntity> for ComparisonGateBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ComparisonGateBehaviourTransitions {}
