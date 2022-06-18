use inexor_rgf_core_model::PropertyInstanceGetter;
use std::convert::AsRef;
use std::sync::Arc;

use crate::behaviour::entity::RandomIntegerWithinRangeProperties;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;
use log::trace;
use rand::Rng;
use serde_json::json;

pub const RANDOM_INTEGER_WITHIN_RANGE: &str = "random_integer_within_range";

pub struct RandomIntegerWithinRange {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl RandomIntegerWithinRange {
    pub fn new(e: Arc<ReactiveEntityInstance>) -> Result<RandomIntegerWithinRange, BehaviourCreationError> {
        let entity = e.clone();
        let handle_id = e.properties.get(RandomIntegerWithinRangeProperties::TRIGGER.as_ref()).unwrap().id.as_u128();
        let mut rng = rand::thread_rng();
        e.properties
            .get(RandomIntegerWithinRangeProperties::TRIGGER.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |trigger| {
                    if !trigger.is_boolean() || !trigger.as_bool().unwrap() {
                        return;
                    }
                    let low = entity.get(RandomIntegerWithinRangeProperties::LOW.as_ref()).and_then(|v| v.as_i64());
                    if low.is_none() {
                        return;
                    }
                    let low = low.unwrap();
                    let high = entity.get(RandomIntegerWithinRangeProperties::HIGH.as_ref()).and_then(|v| v.as_i64());
                    if high.is_none() {
                        return;
                    }
                    let high = high.unwrap();
                    if low >= high {
                        return;
                    }
                    let range = low..high;
                    let result: i64 = rng.gen_range(range);
                    entity.set(RandomIntegerWithinRangeProperties::RESULT.as_ref(), json!(result));
                },
                handle_id,
            );
        Ok(RandomIntegerWithinRange { entity: e, handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for RandomIntegerWithinRange {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", RANDOM_INTEGER_WITHIN_RANGE, self.entity.id);
        if let Some(property) = self.entity.properties.get(RandomIntegerWithinRangeProperties::TRIGGER.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for RandomIntegerWithinRange {
    fn drop(&mut self) {
        self.disconnect();
    }
}
