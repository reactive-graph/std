use std::convert::AsRef;
use std::sync::Arc;

use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::reactive::BehaviourCreationError;
use log::{error, trace};
use serde_json::Value;

use crate::behaviour::entity::ArrayGetByIndexProperties;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;

pub const ARRAY_GET_BY_INDEX: &'static str = "array_get_by_index";

pub struct ArrayGetByIndex {
    pub entity: Arc<ReactiveEntityInstance>,

    pub index_handle_id: u128,

    pub array_handle_id: u128,
}

impl ArrayGetByIndex {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<ArrayGetByIndex, BehaviourCreationError> {
        let index = e.properties.get(ArrayGetByIndexProperties::INDEX.as_ref());
        if index.is_none() {
            error!("Missing property {}", ArrayGetByIndexProperties::INDEX.as_ref());
            return Err(BehaviourCreationError.into());
        }
        let index_handle_id = index.unwrap().id.as_u128();

        let array = e.properties.get(ArrayGetByIndexProperties::ARRAY.as_ref());
        if array.is_none() {
            error!("Missing property {}", ArrayGetByIndexProperties::ARRAY.as_ref());
            return Err(BehaviourCreationError.into());
        }
        let array_handle_id = array.unwrap().id.as_u128();

        let a_entity = e.clone();
        e.properties
            .get(ArrayGetByIndexProperties::ARRAY.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |array| match get_by_index(array, &a_entity.get(ArrayGetByIndexProperties::INDEX.as_ref()).unwrap()) {
                    Some(result) => {
                        a_entity.set(ArrayGetByIndexProperties::RESULT.as_ref(), result);
                    }
                    None => {}
                },
                array_handle_id,
            );
        let i_entity = e.clone();
        e.properties
            .get(ArrayGetByIndexProperties::INDEX.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |index| match get_by_index(&i_entity.get(ArrayGetByIndexProperties::ARRAY.as_ref()).unwrap(), index) {
                    Some(result) => {
                        i_entity.set(ArrayGetByIndexProperties::RESULT.as_ref(), result);
                    }
                    None => {}
                },
                index_handle_id,
            );
        Ok(ArrayGetByIndex {
            entity: e.clone(),
            index_handle_id,
            array_handle_id,
        })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for ArrayGetByIndex {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", ARRAY_GET_BY_INDEX, self.array_handle_id);
        match self.entity.properties.get(ArrayGetByIndexProperties::ARRAY.as_ref()) {
            Some(property) => property.stream.read().unwrap().remove(self.array_handle_id),
            _ => {}
        }
        trace!("Disconnecting {} with id {}", ARRAY_GET_BY_INDEX, self.index_handle_id);
        match self.entity.properties.get(ArrayGetByIndexProperties::INDEX.as_ref()) {
            Some(property) => property.stream.read().unwrap().remove(self.index_handle_id),
            _ => {}
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for ArrayGetByIndex {
    fn drop(&mut self) {
        self.disconnect();
    }
}

fn get_by_index(array: &Value, index: &Value) -> Option<Value> {
    match index.as_u64() {
        Some(index) => {
            match array.as_array() {
                Some(array) => match array.get(index as usize) {
                    Some(value) => Some(value.clone()), // PERF: late clone
                    None => None,
                },
                None => None,
            }
        }
        _ => None,
    }
}
