use std::convert::AsRef;
use std::sync::Arc;

use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::reactive::BehaviourCreationError;
use log::{error, trace};
use serde_json::{json, Value};

use crate::behaviour::entity::ArrayContainsProperties;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;

pub const ARRAY_CONTAINS: &'static str = "array_contains";

pub struct ArrayContains {
    pub entity: Arc<ReactiveEntityInstance>,

    pub search_handle_id: u128,

    pub array_handle_id: u128,
}

impl ArrayContains {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<ArrayContains, BehaviourCreationError> {
        let search = e.properties.get(ArrayContainsProperties::SEARCH.as_ref());
        if search.is_none() {
            error!("Missing property {}", ArrayContainsProperties::SEARCH.as_ref());
            return Err(BehaviourCreationError.into());
        }
        let search_handle_id = search.unwrap().id.as_u128();

        let array = e.properties.get(ArrayContainsProperties::ARRAY.as_ref());
        if array.is_none() {
            error!("Missing property {}", ArrayContainsProperties::ARRAY.as_ref());
            return Err(BehaviourCreationError.into());
        }
        let array_handle_id = array.unwrap().id.as_u128();

        let a_entity = e.clone();
        e.properties
            .get(ArrayContainsProperties::ARRAY.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |array| match array_contains(array, &a_entity.get(ArrayContainsProperties::SEARCH.as_ref()).unwrap()) {
                    Some(result) => {
                        a_entity.set(ArrayContainsProperties::RESULT.as_ref(), result);
                    }
                    None => {}
                },
                array_handle_id,
            );
        let s_entity = e.clone();
        e.properties
            .get(ArrayContainsProperties::SEARCH.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |index| match array_contains(&s_entity.get(ArrayContainsProperties::ARRAY.as_ref()).unwrap(), index) {
                    Some(result) => {
                        s_entity.set(ArrayContainsProperties::RESULT.as_ref(), result);
                    }
                    None => {}
                },
                search_handle_id,
            );
        Ok(ArrayContains {
            entity: e.clone(),
            search_handle_id,
            array_handle_id,
        })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for ArrayContains {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", ARRAY_CONTAINS, self.array_handle_id);
        match self.entity.properties.get(ArrayContainsProperties::ARRAY.as_ref()) {
            Some(property) => property.stream.read().unwrap().remove(self.array_handle_id),
            _ => {}
        }
        trace!("Disconnecting {} with id {}", ARRAY_CONTAINS, self.search_handle_id);
        match self.entity.properties.get(ArrayContainsProperties::SEARCH.as_ref()) {
            Some(property) => property.stream.read().unwrap().remove(self.search_handle_id),
            _ => {}
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for ArrayContains {
    fn drop(&mut self) {
        self.disconnect();
    }
}

fn array_contains(array: &Value, search: &Value) -> Option<Value> {
    match array.as_array() {
        Some(array) => Some(json!(array.contains(search))),
        None => None,
    }
}
