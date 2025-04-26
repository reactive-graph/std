use chrono::DateTime;
use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_std_date_time_model::DateComparisonProperties::LHS;
use reactive_graph_std_date_time_model::DateComparisonProperties::RHS;
use reactive_graph_std_result_model::ResultBooleanProperties::RESULT;

entity_behaviour!(
    EqualsDateTime,
    EqualsDateTimeFactory,
    EqualsDateTimeFsm,
    EqualsDateTimeBehaviourTransitions,
    EqualsDateTimeValidator
);

behaviour_validator!(EqualsDateTimeValidator, Uuid, ReactiveEntity, LHS.as_ref(), RHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for EqualsDateTimeBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(lhs) = self
            .reactive_instance
            .as_string(LHS)
            .and_then(|lhs| DateTime::parse_from_rfc3339(lhs.as_str()).ok())
        {
            if let Some(rhs) = self
                .reactive_instance
                .as_string(RHS)
                .and_then(|rhs| DateTime::parse_from_rfc3339(rhs.as_str()).ok())
            {
                self.reactive_instance.set(RESULT, json!(lhs.eq(&rhs)))
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for EqualsDateTimeBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(LHS.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            reactive_instance.set(RESULT, json!(chrono::Utc::now().to_rfc3339()));
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for EqualsDateTimeBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for EqualsDateTimeBehaviourTransitions {}
