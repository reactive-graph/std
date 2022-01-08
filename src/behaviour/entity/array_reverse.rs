use std::convert::AsRef;
use std::sync::Arc;

use log::trace;
use serde_json::Value;

use crate::behaviour::entity::ArrayReverseProperties;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const ARRAY_REVERSE: &'static str = "array_reverse";

pub struct ArrayReverse {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl ArrayReverse {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<ArrayReverse, BehaviourCreationError> {
        let entity = e.clone();
        let handle_id = e.properties.get(ArrayReverseProperties::ARRAY.as_ref()).unwrap().id.as_u128();
        e.properties
            .get(ArrayReverseProperties::ARRAY.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |value| {
                    if !value.is_array() {
                        return;
                    }
                    let reversed: Vec<&Value> = value.as_array().unwrap().into_iter().rev().collect();
                    let result = Value::Array(reversed.into_iter().map(|v| v.clone()).collect());
                    entity.set(ArrayReverseProperties::RESULT.to_string(), result);
                },
                handle_id,
            );
        Ok(ArrayReverse { entity: e.clone(), handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for ArrayReverse {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", ARRAY_REVERSE, self.entity.id);
        let property = self.entity.properties.get(ArrayReverseProperties::ARRAY.as_ref());
        if property.is_some() {
            property.unwrap().stream.read().unwrap().remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for ArrayReverse {
    fn drop(&mut self) {
        self.disconnect();
    }
}
