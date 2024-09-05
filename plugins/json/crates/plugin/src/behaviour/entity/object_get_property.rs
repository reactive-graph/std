use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use uuid::Uuid;

use reactive_graph_model_json::ObjectGetPropertyProperties::OBJECT;
use reactive_graph_model_json::ObjectGetPropertyProperties::PROPERTY_NAME;
use reactive_graph_model_result::ResultAnyProperties::RESULT;

entity_behaviour!(
    ObjectGetProperty,
    ObjectGetPropertyFactory,
    ObjectGetPropertyFsm,
    ObjectGetPropertyBehaviourTransitions,
    ObjectGetPropertyValidator
);

behaviour_validator!(ObjectGetPropertyValidator, Uuid, ReactiveEntity, OBJECT.as_ref(), RESULT.as_ref(), PROPERTY_NAME.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ObjectGetPropertyBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(object) = self.reactive_instance.get(OBJECT) {
            if let Some(property_name) = self.reactive_instance.get(PROPERTY_NAME) {
                if let Some(result) = get_property_by_name(&object, &property_name) {
                    self.reactive_instance.set(RESULT, result);
                }
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ObjectGetPropertyBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(OBJECT.as_ref(), move |object: &Value| {
            if let Some(property_name) = reactive_instance.get(PROPERTY_NAME) {
                if let Some(result) = get_property_by_name(object, &property_name) {
                    reactive_instance.set(RESULT, result);
                }
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers
            .observe_with_handle(PROPERTY_NAME.as_ref(), move |property_name: &Value| {
                if let Some(object) = reactive_instance.get(OBJECT) {
                    if let Some(result) = get_property_by_name(&object, property_name) {
                        reactive_instance.set(RESULT, result);
                    }
                }
            });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for ObjectGetPropertyBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ObjectGetPropertyBehaviourTransitions {}

fn get_property_by_name(object: &Value, property_name: &Value) -> Option<Value> {
    match property_name.as_str() {
        Some(property_name) => {
            match object.as_object() {
                Some(object) => match object.get(property_name) {
                    Some(value) => Some(value.clone()), // PERF: late clone
                    None => None,
                },
                None => None,
            }
        }
        _ => None,
    }
}

// use std::convert::AsRef;
// use std::sync::Arc;
//
// use reactive_graph_graph::{PropertyInstanceGetter, PropertyInstanceSetter};
// use crate::reactive::BehaviourCreationError;
// use log::{error, trace};
// use serde_json::Value;
//
// use crate::behaviour::entity::ObjectGetPropertyProperties;
// use reactive_graph_graph::ReactiveEntityInstance;
// use crate::reactive::entity::Disconnectable;
//
// pub const OBJECT_GET_PROPERTY: &'static str = "object_get_property";
//
// pub struct ObjectGetProperty {
//     pub entity: Arc<ReactiveEntityInstance>,
//
//     pub property_name_handle_id: u128,
//
//     pub object_handle_id: u128,
// }
//
// impl ObjectGetProperty {
//     pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<ObjectGetProperty, BehaviourCreationError> {
//         let property_name = e.properties.get(ObjectGetPropertyProperties::PROPERTY_NAME.as_ref());
//         if property_name.is_none() {
//             error!("Missing property {}", ObjectGetPropertyProperties::PROPERTY_NAME.as_ref());
//             return Err(BehaviourCreationError.into());
//         }
//         let property_name_handle_id = property_name.unwrap().id.as_u128();
//
//         let object = e.properties.get(ObjectGetPropertyProperties::OBJECT.as_ref());
//         if object.is_none() {
//             error!("Missing property {}", ObjectGetPropertyProperties::OBJECT.as_ref());
//             return Err(BehaviourCreationError.into());
//         }
//         let object_handle_id = object.unwrap().id.as_u128();
//
//         let o_entity = e.clone();
//         e.properties
//             .get(ObjectGetPropertyProperties::OBJECT.as_ref())
//             .unwrap()
//             .stream
//             .read()
//             .unwrap()
//             .observe_with_handle(
//                 move |object| match get_property_by_name(object, &o_entity.get(ObjectGetPropertyProperties::PROPERTY_NAME.as_ref()).unwrap()) {
//                     Some(result) => {
//                         o_entity.set(ObjectGetPropertyProperties::RESULT.as_ref(), result);
//                     }
//                     None => {}
//                 },
//                 object_handle_id,
//             );
//         let p_entity = e.clone();
//         e.properties
//             .get(ObjectGetPropertyProperties::PROPERTY_NAME.as_ref())
//             .unwrap()
//             .stream
//             .read()
//             .unwrap()
//             .observe_with_handle(
//                 move |property_name| match get_property_by_name(&p_entity.get(ObjectGetPropertyProperties::OBJECT.as_ref()).unwrap(), property_name) {
//                     Some(result) => {
//                         p_entity.set(ObjectGetPropertyProperties::RESULT.as_ref(), result);
//                     }
//                     None => {}
//                 },
//                 property_name_handle_id,
//             );
//         Ok(ObjectGetProperty {
//             entity: e.clone(),
//             property_name_handle_id,
//             object_handle_id,
//         })
//     }
//
//     pub fn type_name(&self) -> String {
//         self.entity.type_name.clone()
//     }
// }
//
// impl Disconnectable for ObjectGetProperty {
//     fn disconnect(&self) {
//         trace!("Disconnecting {} with id {}", OBJECT_GET_PROPERTY, self.object_handle_id);
//         match self.entity.properties.get(ObjectGetPropertyProperties::OBJECT.as_ref()) {
//             Some(property) => property.stream.read().unwrap().remove(self.object_handle_id),
//             _ => {}
//         }
//         trace!("Disconnecting {} with id {}", OBJECT_GET_PROPERTY, self.property_name_handle_id);
//         match self.entity.properties.get(ObjectGetPropertyProperties::PROPERTY_NAME.as_ref()) {
//             Some(property) => property.stream.read().unwrap().remove(self.property_name_handle_id),
//             _ => {}
//         }
//     }
// }
//
// /// Automatically disconnect streams on destruction
// impl Drop for ObjectGetProperty {
//     fn drop(&mut self) {
//         self.disconnect();
//     }
// }
