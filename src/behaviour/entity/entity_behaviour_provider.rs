use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;
use waiter_di::*;

use crate::behaviour::entity::array_get_by_index::ArrayGetByIndex;
use crate::behaviour::entity::array_get_by_index::ARRAY_GET_BY_INDEX;
use crate::behaviour::entity::array_length::ArrayLength;
use crate::behaviour::entity::array_length::ARRAY_LENGTH;
use crate::behaviour::entity::array_pop::ArrayPop;
use crate::behaviour::entity::array_pop::ARRAY_POP;
use crate::behaviour::entity::array_push::ArrayPush;
use crate::behaviour::entity::array_push::ARRAY_PUSH;
use crate::behaviour::entity::object_get_property::ObjectGetProperty;
use crate::behaviour::entity::object_get_property::OBJECT_GET_PROPERTY;
use crate::behaviour::entity::object_keys::ObjectKeys;
use crate::behaviour::entity::object_keys::OBJECT_KEYS;
use crate::behaviour::entity::object_remove_property::ObjectRemoveProperty;
use crate::behaviour::entity::object_remove_property::OBJECT_REMOVE_PROPERTY;
use crate::behaviour::entity::object_set_property::ObjectSetProperty;
use crate::behaviour::entity::object_set_property::OBJECT_SET_PROPERTY;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct ArrayPushStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<ArrayPush<'static>>>>);

#[wrapper]
pub struct ArrayPopStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<ArrayPop>>>);

#[wrapper]
pub struct ArrayGetByIndexStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<ArrayGetByIndex>>>);

#[wrapper]
pub struct ArrayLengthStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<ArrayLength>>>);

#[wrapper]
pub struct ObjectGetPropertyStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<ObjectGetProperty>>>);

#[wrapper]
pub struct ObjectSetPropertyStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<ObjectSetProperty>>>);

#[wrapper]
pub struct ObjectRemovePropertyStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<ObjectRemoveProperty>>>);

#[wrapper]
pub struct ObjectKeysStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<ObjectKeys>>>);

#[waiter_di::provides]
fn create_array_push_storage() -> ArrayPushStorage {
    ArrayPushStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[waiter_di::provides]
fn create_array_pop_storage() -> ArrayPopStorage {
    ArrayPopStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[waiter_di::provides]
fn create_array_get_by_index_storage() -> ArrayGetByIndexStorage {
    ArrayGetByIndexStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[waiter_di::provides]
fn create_array_length_storage() -> ArrayLengthStorage {
    ArrayLengthStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[waiter_di::provides]
fn create_object_get_property_storage() -> ObjectGetPropertyStorage {
    ObjectGetPropertyStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[waiter_di::provides]
fn create_object_set_property_storage() -> ObjectSetPropertyStorage {
    ObjectSetPropertyStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[waiter_di::provides]
fn create_object_remove_property_storage() -> ObjectRemovePropertyStorage {
    ObjectRemovePropertyStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[waiter_di::provides]
fn create_object_keys_storage() -> ObjectKeysStorage {
    ObjectKeysStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait JsonEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_array_push(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_array_pop(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_array_get_by_index(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_array_length(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_object_get_property(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_object_set_property(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_object_remove_property(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_object_keys(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_array_push(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_array_pop(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_array_get_by_index(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_array_length(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_object_get_property(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_object_set_property(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_object_remove_property(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_object_keys(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct JsonEntityBehaviourProviderImpl {
    array_push: ArrayPushStorage,
    array_pop: ArrayPopStorage,
    array_get_by_index: ArrayGetByIndexStorage,
    array_length: ArrayLengthStorage,
    object_get_property: ObjectGetPropertyStorage,
    object_set_property: ObjectSetPropertyStorage,
    object_remove_property: ObjectRemovePropertyStorage,
    object_keys: ObjectKeysStorage,
}

interfaces!(JsonEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl JsonEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            array_push: create_array_push_storage(),
            array_pop: create_array_pop_storage(),
            array_get_by_index: create_array_get_by_index_storage(),
            array_length: create_array_length_storage(),
            object_get_property: create_object_get_property_storage(),
            object_set_property: create_object_set_property_storage(),
            object_remove_property: create_object_remove_property_storage(),
            object_keys: create_object_keys_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl JsonEntityBehaviourProvider for JsonEntityBehaviourProviderImpl {
    fn create_array_push(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ArrayPush::new(entity_instance) {
            Ok(array_push) => {
                self.array_push.0.write().unwrap().insert(id, Arc::new(array_push));
                debug!("Added behaviour {} to entity instance {}", ARRAY_PUSH, id);
            }
            _ => {}
        }
    }

    fn create_array_pop(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ArrayPop::new(entity_instance) {
            Ok(array_pop) => {
                self.array_pop.0.write().unwrap().insert(id, Arc::new(array_pop));
                debug!("Added behaviour {} to entity instance {}", ARRAY_POP, id);
            }
            _ => {}
        }
    }

    fn create_array_get_by_index(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ArrayGetByIndex::new(entity_instance) {
            Ok(array_get_by_index) => {
                self.array_get_by_index.0.write().unwrap().insert(id, Arc::new(array_get_by_index));
                debug!("Added behaviour {} to entity instance {}", ARRAY_GET_BY_INDEX, id);
            }
            _ => {}
        }
    }

    fn create_array_length(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ArrayLength::new(entity_instance) {
            Ok(array_length) => {
                self.array_length.0.write().unwrap().insert(id, Arc::new(array_length));
                debug!("Added behaviour {} to entity instance {}", ARRAY_LENGTH, id);
            }
            _ => {}
        }
    }

    fn create_object_get_property(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ObjectGetProperty::new(entity_instance) {
            Ok(object_get_property) => {
                self.object_get_property.0.write().unwrap().insert(id, Arc::new(object_get_property));
                debug!("Added behaviour {} to entity instance {}", OBJECT_GET_PROPERTY, id);
            }
            _ => {}
        }
    }

    fn create_object_set_property(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ObjectSetProperty::new(entity_instance) {
            Ok(object_set_property) => {
                self.object_set_property.0.write().unwrap().insert(id, Arc::new(object_set_property));
                debug!("Added behaviour {} to entity instance {}", OBJECT_SET_PROPERTY, id);
            }
            _ => {}
        }
    }

    fn create_object_remove_property(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ObjectRemoveProperty::new(entity_instance) {
            Ok(object_remove_property) => {
                self.object_remove_property.0.write().unwrap().insert(id, Arc::new(object_remove_property));
                debug!("Added behaviour {} to entity instance {}", OBJECT_REMOVE_PROPERTY, id);
            }
            _ => {}
        }
    }

    fn create_object_keys(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ObjectKeys::new(entity_instance) {
            Ok(object_keys) => {
                self.object_keys.0.write().unwrap().insert(id, Arc::new(object_keys));
                debug!("Added behaviour {} to entity instance {}", OBJECT_KEYS, id);
            }
            _ => {}
        }
    }

    fn remove_array_push(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.array_push.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour {} from entity instance {}", ARRAY_PUSH, entity_instance.id);
    }

    fn remove_array_pop(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.array_pop.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour {} from entity instance {}", ARRAY_POP, entity_instance.id);
    }

    fn remove_array_get_by_index(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.array_get_by_index.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour {} from entity instance {}", ARRAY_GET_BY_INDEX, entity_instance.id);
    }

    fn remove_array_length(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.array_length.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour {} from entity instance {}", ARRAY_LENGTH, entity_instance.id);
    }

    fn remove_object_get_property(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.object_get_property.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour {} from entity instance {}", OBJECT_GET_PROPERTY, entity_instance.id);
    }

    fn remove_object_set_property(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.object_set_property.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour {} from entity instance {}", OBJECT_SET_PROPERTY, entity_instance.id);
    }

    fn remove_object_remove_property(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.object_remove_property.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour {} from entity instance {}", OBJECT_REMOVE_PROPERTY, entity_instance.id);
    }

    fn remove_object_keys(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.object_keys.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour {} from entity instance {}", OBJECT_KEYS, entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.array_push.0.write().unwrap().contains_key(&id) {
            self.array_push.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", ARRAY_PUSH, id);
        }
        if self.array_pop.0.write().unwrap().contains_key(&id) {
            self.array_pop.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", ARRAY_POP, id);
        }
        if self.array_get_by_index.0.write().unwrap().contains_key(&id) {
            self.array_get_by_index.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", ARRAY_GET_BY_INDEX, id);
        }
        if self.array_length.0.write().unwrap().contains_key(&id) {
            self.array_length.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", ARRAY_LENGTH, id);
        }
        if self.object_get_property.0.write().unwrap().contains_key(&id) {
            self.object_get_property.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", OBJECT_GET_PROPERTY, id);
        }
        if self.object_set_property.0.write().unwrap().contains_key(&id) {
            self.object_set_property.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", OBJECT_SET_PROPERTY, id);
        }
        if self.object_remove_property.0.write().unwrap().contains_key(&id) {
            self.object_remove_property.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", OBJECT_REMOVE_PROPERTY, id);
        }
        if self.object_keys.0.write().unwrap().contains_key(&id) {
            self.object_keys.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", OBJECT_KEYS, id);
        }
    }
}

impl EntityBehaviourProvider for JsonEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.type_name.as_str() {
            ARRAY_PUSH => self.create_array_push(entity_instance),
            ARRAY_POP => self.create_array_pop(entity_instance),
            ARRAY_GET_BY_INDEX => self.create_array_get_by_index(entity_instance),
            ARRAY_LENGTH => self.create_array_length(entity_instance),
            OBJECT_GET_PROPERTY => self.create_object_get_property(entity_instance),
            OBJECT_SET_PROPERTY => self.create_object_set_property(entity_instance),
            OBJECT_REMOVE_PROPERTY => self.create_object_remove_property(entity_instance),
            OBJECT_KEYS => self.create_object_keys(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            ARRAY_PUSH => self.remove_array_push(entity_instance),
            ARRAY_POP => self.remove_array_pop(entity_instance),
            ARRAY_GET_BY_INDEX => self.remove_array_get_by_index(entity_instance),
            ARRAY_LENGTH => self.remove_array_length(entity_instance),
            OBJECT_GET_PROPERTY => self.remove_object_get_property(entity_instance),
            OBJECT_SET_PROPERTY => self.remove_object_set_property(entity_instance),
            OBJECT_REMOVE_PROPERTY => self.remove_object_remove_property(entity_instance),
            OBJECT_KEYS => self.remove_object_keys(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
