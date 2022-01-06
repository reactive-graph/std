use std::convert::AsRef;
use std::sync::Arc;

use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::reactive::BehaviourCreationError;
use log::{error, trace};

use crate::behaviour::entity::ObjectSetPropertyProperties;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;

pub const OBJECT_SET_PROPERTY: &'static str = "object_set_property";

pub struct ObjectSetProperty {
    pub entity: Arc<ReactiveEntityInstance>,

    pub object_handle_id: u128,

    pub value_handle_id: u128,
}

impl ObjectSetProperty {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<ObjectSetProperty, BehaviourCreationError> {
        let object = e.properties.get(ObjectSetPropertyProperties::OBJECT.as_ref());
        if object.is_none() {
            error!("Missing property {}", ObjectSetPropertyProperties::OBJECT.as_ref());
            return Err(BehaviourCreationError.into());
        }
        let object_handle_id = object.unwrap().id.as_u128();

        let property_name = e.properties.get(ObjectSetPropertyProperties::PROPERTY_NAME.as_ref());
        if property_name.is_none() {
            error!("Missing property {}", ObjectSetPropertyProperties::PROPERTY_NAME.as_ref());
            return Err(BehaviourCreationError.into());
        }

        let value = e.properties.get(ObjectSetPropertyProperties::VALUE.as_ref());
        if value.is_none() {
            error!("Missing property {}", ObjectSetPropertyProperties::VALUE.as_ref());
            return Err(BehaviourCreationError.into());
        }
        let value_handle_id = value.unwrap().id.as_u128();

        let o_entity = e.clone();
        e.properties
            .get(ObjectSetPropertyProperties::OBJECT.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |object| match o_entity.get(ObjectSetPropertyProperties::PROPERTY_NAME.as_ref()).unwrap().as_str() {
                    Some(property_name) => {
                        let mut result = object.clone();
                        let value = o_entity.get(ObjectSetPropertyProperties::VALUE.as_ref()).unwrap();
                        result[property_name] = value;
                        o_entity.set(ObjectSetPropertyProperties::RESULT.as_ref(), result);
                    }
                    None => {}
                },
                object_handle_id,
            );
        let v_entity = e.clone();
        e.properties
            .get(ObjectSetPropertyProperties::VALUE.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |value| match v_entity.get(ObjectSetPropertyProperties::PROPERTY_NAME.as_ref()).unwrap().as_str() {
                    Some(property_name) => {
                        let mut result = v_entity.get(ObjectSetPropertyProperties::OBJECT.as_ref()).unwrap().clone();
                        result[property_name] = value.clone();
                        v_entity.set(ObjectSetPropertyProperties::RESULT.as_ref(), result);
                    }
                    None => {}
                },
                value_handle_id,
            );
        Ok(ObjectSetProperty {
            entity: e.clone(),
            object_handle_id,
            value_handle_id,
        })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for ObjectSetProperty {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", OBJECT_SET_PROPERTY, self.object_handle_id);
        match self.entity.properties.get(ObjectSetPropertyProperties::OBJECT.as_ref()) {
            Some(property) => property.stream.read().unwrap().remove(self.object_handle_id),
            _ => {}
        }
        trace!("Disconnecting {} with id {}", OBJECT_SET_PROPERTY, self.value_handle_id);
        match self.entity.properties.get(ObjectSetPropertyProperties::VALUE.as_ref()) {
            Some(property) => property.stream.read().unwrap().remove(self.value_handle_id),
            _ => {}
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for ObjectSetProperty {
    fn drop(&mut self) {
        self.disconnect();
    }
}
