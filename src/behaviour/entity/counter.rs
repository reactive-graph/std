use std::convert::AsRef;
use std::sync::Arc;

use log::error;
use serde_json::{json, Value};

use crate::behaviour::entity::counter_properties::CounterProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const COUNTER: &str = "counter";

pub struct Counter {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl Counter {
    pub fn new(e: Arc<ReactiveEntityInstance>) -> Result<Counter, BehaviourCreationError> {
        let condition = e.properties.get(CounterProperties::TRIGGER.as_ref());
        if condition.is_none() {
            error!("Missing property {}", CounterProperties::TRIGGER.as_ref());
            return Err(BehaviourCreationError);
        }
        let result = e.properties.get(CounterProperties::RESULT.as_ref());
        if result.is_none() {
            error!("Missing property {}", CounterProperties::RESULT.as_ref());
            return Err(BehaviourCreationError);
        }

        let entity_instance = e.clone();
        let handle_id = e.properties.get(CounterProperties::TRIGGER.as_ref()).unwrap().id.as_u128();
        e.properties
            .get(CounterProperties::TRIGGER.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |v: &Value| {
                    if !v.is_boolean() {
                        // Invalid type
                        return;
                    }
                    if !v.as_bool().unwrap() {
                        // Counter only on true (= high)
                        return;
                    }
                    match entity_instance.get(CounterProperties::RESULT).and_then(|v| v.as_i64()) {
                        Some(current_value) => {
                            entity_instance.set(CounterProperties::RESULT, json!(current_value + 1));
                        }
                        None => {
                            entity_instance.set(CounterProperties::RESULT, json!(0));
                        }
                    }
                },
                handle_id,
            );

        Ok(Counter { entity: e.clone(), handle_id })
    }
}

impl Disconnectable for Counter {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(CounterProperties::TRIGGER.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        self.disconnect();
    }
}
