use std::convert::AsRef;
use std::sync::Arc;

use crate::behaviour::entity::ArrayLengthProperties;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;
use log::trace;
use serde_json::json;

pub const ARRAY_LENGTH: &'static str = "array_length";

pub struct ArrayLength {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl ArrayLength {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<ArrayLength, BehaviourCreationError> {
        // TODO: handle result based on outbound property id and inbound property id
        let entity = e.clone();
        let handle_id = e.properties.get(ArrayLengthProperties::ARRAY.as_ref()).unwrap().id.as_u128();
        e.properties
            .get(ArrayLengthProperties::ARRAY.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |array| {
                    if !array.is_array() {
                        return;
                    }
                    match i64::try_from(array.as_array().unwrap().len()) {
                        Ok(length) => {
                            entity.set(ArrayLengthProperties::LENGTH.to_string(), json!(length));
                        }
                        Err(_) => {}
                    }
                },
                handle_id,
            );
        Ok(ArrayLength { entity: e.clone(), handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for ArrayLength {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", ARRAY_LENGTH, self.entity.id);
        let property = self.entity.properties.get(ArrayLengthProperties::ARRAY.as_ref());
        if property.is_some() {
            property.unwrap().stream.read().unwrap().remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for ArrayLength {
    fn drop(&mut self) {
        self.disconnect();
    }
}
