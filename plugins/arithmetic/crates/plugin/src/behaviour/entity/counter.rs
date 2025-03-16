use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_model_result::ResultNumberU64Properties::RESULT;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;

entity_behaviour!(Counter, CounterFactory, CounterFsm, CounterBehaviourTransitions, CounterValidator);

behaviour_validator!(CounterValidator, Uuid, ReactiveEntity, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for CounterBehaviourTransitions {}

impl BehaviourConnect<Uuid, ReactiveEntity> for CounterBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        // let counter = crate::model_arithmetic::Counter::from(self.reactive_instance.clone());
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            match reactive_instance.get(RESULT).and_then(|v| v.as_i64()) {
                Some(current_value) => {
                    reactive_instance.set(RESULT, json!(current_value + 1));
                }
                None => {
                    reactive_instance.set(RESULT, json!(0));
                }
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for CounterBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for CounterBehaviourTransitions {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    use crate::behaviour::entity::counter::CounterFactory;
    use reactive_graph_model_arithmetic::BEHAVIOUR_COUNTER;
    use reactive_graph_model_arithmetic::Counter1;
    use reactive_graph_model_arithmetic::ENTITY_TYPE_COUNTER;
    use reactive_graph_model_result::ResultNumberU64Properties::RESULT;
    use reactive_graph_reactive_model_api::ReactiveInstanceContainer;
    use reactive_graph_reactive_model_impl::ReactiveProperties;
    use reactive_graph_runtime_model::ActionProperties::TRIGGER;

    // TODO: This test should be moved to the reactive model
    #[cfg(feature = "rustc_nightly")]
    #[test]
    fn rx_counter_test() {
        let id = Uuid::new_v4();
        let properties = PropertyInstances::new().property(RESULT, json!(0)).property(TRIGGER, json!(false));

        let reactive_instance = ReactiveEntity::builder()
            .ty(ENTITY_TYPE_COUNTER.clone())
            .id(id)
            .properties(ReactiveProperties::new_with_id_from_properties(id, properties))
            .build();

        let counter = Counter1::from(reactive_instance.clone());

        let factory = CounterFactory::new(BEHAVIOUR_COUNTER.clone());
        {
            let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
            assert_eq!(BehaviourState::Connected, behaviour.get_state(), "The behaviour should be connected!");

            assert_eq!(0, counter.result, "The counter should have the default value!");
            (counter.trigger)(true);
            assert_eq!(1, counter.result, "The counter should have been increased!");
            (counter.trigger)(true);
            assert_eq!(2, counter.result, "The counter should have been increased!");
            (counter.trigger)(true);
            assert_eq!(3, counter.result, "The counter should have been increased!");
        }

        (counter.trigger)(true);
        assert_eq!(3, counter.result, "The behaviour should have been dropped.");
    }

    #[cfg(feature = "rustc_nightly")]
    #[test]
    fn rx_counter_2_test() {
        let counter = Counter1::new(false, 0u64);
        // let reactive_entity = counter.result.get_reactive_instance();

        let factory = CounterFactory::new(BEHAVIOUR_COUNTER.clone());
        {
            let behaviour = factory
                .create(counter.result.get_reactive_instance().clone())
                .expect("Failed to create behaviour");
            assert_eq!(BehaviourState::Connected, behaviour.get_state(), "The behaviour should be connected!");

            assert_eq!(0, counter.result, "The counter should have the default value!");
            (counter.trigger)(true);
            assert_eq!(1, counter.result, "The counter should have been increased!");
            (counter.trigger)(true);
            assert_eq!(2, counter.result, "The counter should have been increased!");
            (counter.trigger)(true);
            assert_eq!(3, counter.result, "The counter should have been increased!");
        }

        (counter.trigger)(true);
        assert_eq!(3, counter.result, "The behaviour should have been dropped.");
    }
}
