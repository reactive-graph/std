use inexor_rgf_core_model::PropertyInstanceGetter;
use std::convert::AsRef;
use std::sync::Arc;

use crate::behaviour::entity::PseudoRandomNumberProperties;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;
use log::trace;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde_json::json;

pub const PSEUDO_RANDOM_NUMBER: &str = "pseudo_random_number";

pub struct PseudoRandomNumber {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl PseudoRandomNumber {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<PseudoRandomNumber, BehaviourCreationError> {
        let entity = e.clone();
        let handle_id = e.properties.get(PseudoRandomNumberProperties::TRIGGER.as_ref()).unwrap().id.as_u128();

        let seed = e.get(PseudoRandomNumberProperties::SEED.as_ref()).and_then(|v| v.as_u64());
        if seed.is_none() {
            return Err(BehaviourCreationError);
        }
        let seed = seed.unwrap();

        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        e.properties
            .get(PseudoRandomNumberProperties::TRIGGER.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |trigger| {
                    if !trigger.is_boolean() || !trigger.as_bool().unwrap() {
                        return;
                    }
                    entity.set(PseudoRandomNumberProperties::RESULT.as_ref(), json!(rng.gen::<f64>()));
                },
                handle_id,
            );
        Ok(PseudoRandomNumber { entity: e, handle_id })
    }
}

impl Disconnectable for PseudoRandomNumber {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(PseudoRandomNumberProperties::TRIGGER.as_ref()) {
            trace!("Disconnecting {} with id {}", PSEUDO_RANDOM_NUMBER, self.entity.id);
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Drop for PseudoRandomNumber {
    fn drop(&mut self) {
        self.disconnect();
    }
}
