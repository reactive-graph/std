use std::convert::AsRef;
use std::sync::Arc;

use log::trace;
use serde_json::Value;

use crate::behaviour::entity::ArrayPopProperties;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const ARRAY_POP: &'static str = "array_pop";

pub struct ArrayPop {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl ArrayPop {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<ArrayPop, BehaviourCreationError> {
        // TODO: handle result based on outbound property id and inbound property id
        let entity = e.clone();
        let handle_id = e.properties.get(ArrayPopProperties::ARRAY.as_ref()).unwrap().id.as_u128();
        e.properties
            .get(ArrayPopProperties::ARRAY.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |array| {
                    let (result, value) = pop_array(&array);
                    entity.set(ArrayPopProperties::RESULT.to_string(), result);
                    match value {
                        Some(value) => {
                            entity.set(ArrayPopProperties::VALUE.to_string(), value);
                        }
                        None => {}
                    }
                },
                handle_id,
            );
        Ok(ArrayPop { entity: e.clone(), handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for ArrayPop {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", ARRAY_POP, self.entity.id);
        let property = self.entity.properties.get(ArrayPopProperties::ARRAY.as_ref());
        if property.is_some() {
            property.unwrap().stream.read().unwrap().remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for ArrayPop {
    fn drop(&mut self) {
        self.disconnect();
    }
}

fn pop_array(array: &Value) -> (Value, Option<Value>) {
    match array.as_array() {
        Some(array) => {
            let mut array = array.clone();
            let value = array.pop();
            (Value::Array(array), value)
        }
        None => (array.clone(), None),
    }
}
