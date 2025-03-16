use rand::Rng;
use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_model_result::ResultBooleanProperties::RESULT;

entity_behaviour!(RandomBool, RandomBoolFactory, RandomBoolFsm, RandomBoolBehaviourTransitions, RandomBoolValidator);

behaviour_validator!(RandomBoolValidator, Uuid, ReactiveEntity, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for RandomBoolBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            self.reactive_instance.set(RESULT, random());
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for RandomBoolBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            reactive_instance.set(RESULT, random());
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for RandomBoolBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for RandomBoolBehaviourTransitions {}

fn random() -> Value {
    let mut rng = rand::rng();
    json!(rng.random::<bool>())
}
