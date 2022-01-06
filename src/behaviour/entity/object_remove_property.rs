use std::convert::AsRef;
use std::sync::Arc;

use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::reactive::BehaviourCreationError;
use log::{error, trace};
use serde_json::Value;

use crate::behaviour::entity::ObjectRemovePropertyProperties;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;

pub const OBJECT_REMOVE_PROPERTY: &'static str = "object_remove_property";

pub struct ObjectRemoveProperty {
    pub entity: Arc<ReactiveEntityInstance>,

    pub object_handle_id: u128,

    pub property_name_handle_id: u128,
}

impl ObjectRemoveProperty {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<ObjectRemoveProperty, BehaviourCreationError> {
        let object = e.properties.get(ObjectRemovePropertyProperties::OBJECT.as_ref());
        if object.is_none() {
            error!("Missing property {}", ObjectRemovePropertyProperties::OBJECT.as_ref());
            return Err(BehaviourCreationError.into());
        }
        let object_handle_id = object.unwrap().id.as_u128();

        let property_name = e.properties.get(ObjectRemovePropertyProperties::PROPERTY_NAME.as_ref());
        if property_name.is_none() {
            error!("Missing property {}", ObjectRemovePropertyProperties::PROPERTY_NAME.as_ref());
            return Err(BehaviourCreationError.into());
        }
        let property_name_handle_id = property_name.unwrap().id.as_u128();

        let o_entity = e.clone();
        e.properties
            .get(ObjectRemovePropertyProperties::OBJECT.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |object| match object.as_object() {
                    Some(object) => match o_entity.get(ObjectRemovePropertyProperties::PROPERTY_NAME.as_ref()) {
                        Some(property_name) => match property_name.as_str() {
                            Some(property_name) => {
                                let mut result = object.clone();
                                result.remove(property_name);
                                o_entity.set(ObjectRemovePropertyProperties::RESULT.as_ref(), Value::Object(result));
                            }
                            None => {}
                        },
                        None => {}
                    },
                    None => {}
                },
                object_handle_id,
            );
        let p_entity = e.clone();
        e.properties
            .get(ObjectRemovePropertyProperties::PROPERTY_NAME.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |property_name| match property_name.as_str() {
                    Some(property_name) => match p_entity.get(ObjectRemovePropertyProperties::OBJECT.as_ref()) {
                        Some(object) => match object.as_object() {
                            Some(object) => {
                                let mut result = object.clone();
                                result.remove(property_name);
                                p_entity.set(ObjectRemovePropertyProperties::RESULT.as_ref(), Value::Object(result));
                            }
                            None => {}
                        },
                        None => {}
                    },
                    None => {}
                },
                property_name_handle_id,
            );
        Ok(ObjectRemoveProperty {
            entity: e.clone(),
            object_handle_id,
            property_name_handle_id,
        })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for ObjectRemoveProperty {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", OBJECT_REMOVE_PROPERTY, self.object_handle_id);
        match self.entity.properties.get(ObjectRemovePropertyProperties::OBJECT.as_ref()) {
            Some(property) => property.stream.read().unwrap().remove(self.object_handle_id),
            _ => {}
        }
        trace!("Disconnecting {} with id {}", OBJECT_REMOVE_PROPERTY, self.property_name_handle_id);
        match self.entity.properties.get(ObjectRemovePropertyProperties::PROPERTY_NAME.as_ref()) {
            Some(property) => property.stream.read().unwrap().remove(self.property_name_handle_id),
            _ => {}
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for ObjectRemoveProperty {
    fn drop(&mut self) {
        self.disconnect();
    }
}
