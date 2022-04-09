use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::entity::array_contains::ArrayContains;
use crate::behaviour::entity::array_contains::ARRAY_CONTAINS;
use crate::behaviour::entity::array_get_by_index::ArrayGetByIndex;
use crate::behaviour::entity::array_get_by_index::ARRAY_GET_BY_INDEX;
use crate::behaviour::entity::array_length::ArrayLength;
use crate::behaviour::entity::array_length::ARRAY_LENGTH;
use crate::behaviour::entity::array_pop::ArrayPop;
use crate::behaviour::entity::array_pop::ARRAY_POP;
use crate::behaviour::entity::array_push::ArrayPush;
use crate::behaviour::entity::array_push::ARRAY_PUSH;
use crate::behaviour::entity::array_reverse::ArrayReverse;
use crate::behaviour::entity::array_reverse::ARRAY_REVERSE;
use crate::behaviour::entity::load_json::LoadJson;
use crate::behaviour::entity::load_json::LOAD_JSON;
use crate::behaviour::entity::object_get_property::ObjectGetProperty;
use crate::behaviour::entity::object_get_property::OBJECT_GET_PROPERTY;
use crate::behaviour::entity::object_keys::ObjectKeys;
use crate::behaviour::entity::object_keys::OBJECT_KEYS;
use crate::behaviour::entity::object_remove_property::ObjectRemoveProperty;
use crate::behaviour::entity::object_remove_property::OBJECT_REMOVE_PROPERTY;
use crate::behaviour::entity::object_set_property::ObjectSetProperty;
use crate::behaviour::entity::object_set_property::OBJECT_SET_PROPERTY;
use crate::behaviour::entity::save_json::SaveJson;
use crate::behaviour::entity::save_json::SAVE_JSON;
use crate::di::*;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct ArrayContainsStorage(RwLock<HashMap<Uuid, Arc<ArrayContains>>>);

#[wrapper]
pub struct ArrayGetByIndexStorage(RwLock<HashMap<Uuid, Arc<ArrayGetByIndex>>>);

#[wrapper]
pub struct ArrayLengthStorage(RwLock<HashMap<Uuid, Arc<ArrayLength>>>);

#[wrapper]
pub struct ArrayPopStorage(RwLock<HashMap<Uuid, Arc<ArrayPop>>>);

#[wrapper]
pub struct ArrayPushStorage(RwLock<HashMap<Uuid, Arc<ArrayPush<'static>>>>);

#[wrapper]
pub struct ArrayReverseStorage(RwLock<HashMap<Uuid, Arc<ArrayReverse>>>);

#[wrapper]
pub struct ObjectGetPropertyStorage(RwLock<HashMap<Uuid, Arc<ObjectGetProperty>>>);

#[wrapper]
pub struct ObjectKeysStorage(RwLock<HashMap<Uuid, Arc<ObjectKeys>>>);

#[wrapper]
pub struct ObjectRemovePropertyStorage(RwLock<HashMap<Uuid, Arc<ObjectRemoveProperty>>>);

#[wrapper]
pub struct ObjectSetPropertyStorage(RwLock<HashMap<Uuid, Arc<ObjectSetProperty>>>);

#[wrapper]
pub struct LoadJsonStorage(RwLock<HashMap<Uuid, Arc<LoadJson>>>);

#[wrapper]
pub struct SaveJsonStorage(RwLock<HashMap<Uuid, Arc<SaveJson>>>);

#[provides]
fn create_array_contains_storage() -> ArrayContainsStorage {
    ArrayContainsStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_array_get_by_index_storage() -> ArrayGetByIndexStorage {
    ArrayGetByIndexStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_array_length_storage() -> ArrayLengthStorage {
    ArrayLengthStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_array_pop_storage() -> ArrayPopStorage {
    ArrayPopStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_array_push_storage() -> ArrayPushStorage {
    ArrayPushStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_array_reverse_storage() -> ArrayReverseStorage {
    ArrayReverseStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_object_get_property_storage() -> ObjectGetPropertyStorage {
    ObjectGetPropertyStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_object_keys_storage() -> ObjectKeysStorage {
    ObjectKeysStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_object_remove_property_storage() -> ObjectRemovePropertyStorage {
    ObjectRemovePropertyStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_object_set_property_storage() -> ObjectSetPropertyStorage {
    ObjectSetPropertyStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_load_json_storage() -> LoadJsonStorage {
    LoadJsonStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_save_json_storage() -> SaveJsonStorage {
    SaveJsonStorage(RwLock::new(HashMap::new()))
}

#[async_trait]
pub trait JsonEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_array_contains(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_array_get_by_index(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_array_length(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_array_pop(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_array_push(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_array_reverse(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_object_get_property(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_object_keys(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_object_remove_property(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_object_set_property(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_load_json(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_save_json(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_array_contains(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_array_get_by_index(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_array_length(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_array_pop(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_array_push(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_array_reverse(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_object_get_property(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_object_keys(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_object_remove_property(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_object_set_property(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_load_json(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_save_json(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct JsonEntityBehaviourProviderImpl {
    array_contains: ArrayContainsStorage,
    array_get_by_index: ArrayGetByIndexStorage,
    array_length: ArrayLengthStorage,
    array_pop: ArrayPopStorage,
    array_push: ArrayPushStorage,
    array_reverse: ArrayReverseStorage,
    object_get_property: ObjectGetPropertyStorage,
    object_keys: ObjectKeysStorage,
    object_remove_property: ObjectRemovePropertyStorage,
    object_set_property: ObjectSetPropertyStorage,
    load_json: LoadJsonStorage,
    save_json: SaveJsonStorage,
}

interfaces!(JsonEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl JsonEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            array_contains: create_array_contains_storage(),
            array_get_by_index: create_array_get_by_index_storage(),
            array_length: create_array_length_storage(),
            array_pop: create_array_pop_storage(),
            array_push: create_array_push_storage(),
            array_reverse: create_array_reverse_storage(),
            object_get_property: create_object_get_property_storage(),
            object_keys: create_object_keys_storage(),
            object_remove_property: create_object_remove_property_storage(),
            object_set_property: create_object_set_property_storage(),
            load_json: create_load_json_storage(),
            save_json: create_save_json_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl JsonEntityBehaviourProvider for JsonEntityBehaviourProviderImpl {
    fn create_array_contains(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ArrayContains::new(entity_instance.clone()) {
            Ok(array_contains) => {
                self.array_contains.0.write().unwrap().insert(id, Arc::new(array_contains));
                entity_instance.add_behaviour(ARRAY_CONTAINS);
                debug!("Added behaviour {} to entity instance {}", ARRAY_CONTAINS, id);
            }
            _ => {}
        }
    }

    fn create_array_get_by_index(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ArrayGetByIndex::new(entity_instance.clone()) {
            Ok(array_get_by_index) => {
                self.array_get_by_index.0.write().unwrap().insert(id, Arc::new(array_get_by_index));
                entity_instance.add_behaviour(ARRAY_GET_BY_INDEX);
                debug!("Added behaviour {} to entity instance {}", ARRAY_GET_BY_INDEX, id);
            }
            _ => {}
        }
    }

    fn create_array_length(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ArrayLength::new(entity_instance.clone()) {
            Ok(array_length) => {
                self.array_length.0.write().unwrap().insert(id, Arc::new(array_length));
                entity_instance.add_behaviour(ARRAY_LENGTH);
                debug!("Added behaviour {} to entity instance {}", ARRAY_LENGTH, id);
            }
            _ => {}
        }
    }

    fn create_array_pop(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ArrayPop::new(entity_instance.clone()) {
            Ok(array_pop) => {
                self.array_pop.0.write().unwrap().insert(id, Arc::new(array_pop));
                entity_instance.add_behaviour(ARRAY_POP);
                debug!("Added behaviour {} to entity instance {}", ARRAY_POP, id);
            }
            _ => {}
        }
    }

    fn create_array_push(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ArrayPush::new(entity_instance.clone()) {
            Ok(array_push) => {
                self.array_push.0.write().unwrap().insert(id, Arc::new(array_push));
                entity_instance.add_behaviour(ARRAY_PUSH);
                debug!("Added behaviour {} to entity instance {}", ARRAY_PUSH, id);
            }
            _ => {}
        }
    }

    fn create_array_reverse(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ArrayReverse::new(entity_instance.clone()) {
            Ok(array_reverse) => {
                self.array_reverse.0.write().unwrap().insert(id, Arc::new(array_reverse));
                entity_instance.add_behaviour(ARRAY_REVERSE);
                debug!("Added behaviour {} to entity instance {}", ARRAY_REVERSE, id);
            }
            _ => {}
        }
    }

    fn create_object_get_property(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ObjectGetProperty::new(entity_instance.clone()) {
            Ok(object_get_property) => {
                self.object_get_property.0.write().unwrap().insert(id, Arc::new(object_get_property));
                entity_instance.add_behaviour(OBJECT_GET_PROPERTY);
                debug!("Added behaviour {} to entity instance {}", OBJECT_GET_PROPERTY, id);
            }
            _ => {}
        }
    }

    fn create_object_keys(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ObjectKeys::new(entity_instance.clone()) {
            Ok(object_keys) => {
                self.object_keys.0.write().unwrap().insert(id, Arc::new(object_keys));
                entity_instance.add_behaviour(OBJECT_KEYS);
                debug!("Added behaviour {} to entity instance {}", OBJECT_KEYS, id);
            }
            _ => {}
        }
    }

    fn create_object_remove_property(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ObjectRemoveProperty::new(entity_instance.clone()) {
            Ok(object_remove_property) => {
                self.object_remove_property.0.write().unwrap().insert(id, Arc::new(object_remove_property));
                entity_instance.add_behaviour(OBJECT_REMOVE_PROPERTY);
                debug!("Added behaviour {} to entity instance {}", OBJECT_REMOVE_PROPERTY, id);
            }
            _ => {}
        }
    }

    fn create_object_set_property(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match ObjectSetProperty::new(entity_instance.clone()) {
            Ok(object_set_property) => {
                self.object_set_property.0.write().unwrap().insert(id, Arc::new(object_set_property));
                entity_instance.add_behaviour(OBJECT_SET_PROPERTY);
                debug!("Added behaviour {} to entity instance {}", OBJECT_SET_PROPERTY, id);
            }
            _ => {}
        }
    }

    fn create_load_json(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match LoadJson::new(entity_instance.clone()) {
            Ok(load_json) => {
                self.load_json.0.write().unwrap().insert(id, Arc::new(load_json));
                entity_instance.add_behaviour(LOAD_JSON);
                debug!("Added behaviour {} to entity instance {}", LOAD_JSON, id);
            }
            _ => {}
        }
    }

    fn create_save_json(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match SaveJson::new(entity_instance.clone()) {
            Ok(save_json) => {
                self.save_json.0.write().unwrap().insert(id, Arc::new(save_json));
                entity_instance.add_behaviour(SAVE_JSON);
                debug!("Added behaviour {} to entity instance {}", SAVE_JSON, id);
            }
            _ => {}
        }
    }

    fn remove_array_contains(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.array_contains.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(ARRAY_CONTAINS);
        debug!("Removed behaviour {} from entity instance {}", ARRAY_CONTAINS, entity_instance.id);
    }

    fn remove_array_get_by_index(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.array_get_by_index.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(ARRAY_GET_BY_INDEX);
        debug!("Removed behaviour {} from entity instance {}", ARRAY_GET_BY_INDEX, entity_instance.id);
    }

    fn remove_array_length(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.array_length.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(ARRAY_LENGTH);
        debug!("Removed behaviour {} from entity instance {}", ARRAY_LENGTH, entity_instance.id);
    }

    fn remove_array_pop(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.array_pop.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(ARRAY_POP);
        debug!("Removed behaviour {} from entity instance {}", ARRAY_POP, entity_instance.id);
    }

    fn remove_array_push(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.array_push.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(ARRAY_PUSH);
        debug!("Removed behaviour {} from entity instance {}", ARRAY_PUSH, entity_instance.id);
    }

    fn remove_array_reverse(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.array_reverse.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(ARRAY_REVERSE);
        debug!("Removed behaviour {} from entity instance {}", ARRAY_REVERSE, entity_instance.id);
    }

    fn remove_object_get_property(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.object_get_property.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(OBJECT_GET_PROPERTY);
        debug!("Removed behaviour {} from entity instance {}", OBJECT_GET_PROPERTY, entity_instance.id);
    }

    fn remove_object_keys(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.object_keys.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(OBJECT_KEYS);
        debug!("Removed behaviour {} from entity instance {}", OBJECT_KEYS, entity_instance.id);
    }

    fn remove_object_remove_property(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.object_remove_property.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(OBJECT_REMOVE_PROPERTY);
        debug!("Removed behaviour {} from entity instance {}", OBJECT_REMOVE_PROPERTY, entity_instance.id);
    }

    fn remove_object_set_property(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.object_set_property.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(OBJECT_SET_PROPERTY);
        debug!("Removed behaviour {} from entity instance {}", OBJECT_SET_PROPERTY, entity_instance.id);
    }

    fn remove_load_json(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.load_json.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(LOAD_JSON);
        debug!("Removed behaviour {} from entity instance {}", LOAD_JSON, entity_instance.id);
    }

    fn remove_save_json(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.save_json.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(SAVE_JSON);
        debug!("Removed behaviour {} from entity instance {}", SAVE_JSON, entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.array_contains.0.write().unwrap().contains_key(&id) {
            self.array_contains.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", ARRAY_CONTAINS, id);
        }
        if self.array_get_by_index.0.write().unwrap().contains_key(&id) {
            self.array_get_by_index.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", ARRAY_GET_BY_INDEX, id);
        }
        if self.array_length.0.write().unwrap().contains_key(&id) {
            self.array_length.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", ARRAY_LENGTH, id);
        }
        if self.array_pop.0.write().unwrap().contains_key(&id) {
            self.array_pop.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", ARRAY_POP, id);
        }
        if self.array_push.0.write().unwrap().contains_key(&id) {
            self.array_push.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", ARRAY_PUSH, id);
        }
        if self.array_reverse.0.write().unwrap().contains_key(&id) {
            self.array_reverse.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", ARRAY_REVERSE, id);
        }
        if self.object_get_property.0.write().unwrap().contains_key(&id) {
            self.object_get_property.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", OBJECT_GET_PROPERTY, id);
        }
        if self.object_keys.0.write().unwrap().contains_key(&id) {
            self.object_keys.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", OBJECT_KEYS, id);
        }
        if self.object_remove_property.0.write().unwrap().contains_key(&id) {
            self.object_remove_property.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", OBJECT_REMOVE_PROPERTY, id);
        }
        if self.object_set_property.0.write().unwrap().contains_key(&id) {
            self.object_set_property.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", OBJECT_SET_PROPERTY, id);
        }
        if self.load_json.0.write().unwrap().contains_key(&id) {
            self.load_json.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", LOAD_JSON, id);
        }
        if self.save_json.0.write().unwrap().contains_key(&id) {
            self.save_json.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", SAVE_JSON, id);
        }
    }
}

impl EntityBehaviourProvider for JsonEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.type_name.as_str() {
            ARRAY_CONTAINS => self.create_array_contains(entity_instance),
            ARRAY_GET_BY_INDEX => self.create_array_get_by_index(entity_instance),
            ARRAY_LENGTH => self.create_array_length(entity_instance),
            ARRAY_POP => self.create_array_pop(entity_instance),
            ARRAY_PUSH => self.create_array_push(entity_instance),
            ARRAY_REVERSE => self.create_array_reverse(entity_instance),
            OBJECT_GET_PROPERTY => self.create_object_get_property(entity_instance),
            OBJECT_KEYS => self.create_object_keys(entity_instance),
            OBJECT_REMOVE_PROPERTY => self.create_object_remove_property(entity_instance),
            OBJECT_SET_PROPERTY => self.create_object_set_property(entity_instance),
            LOAD_JSON => self.create_load_json(entity_instance),
            SAVE_JSON => self.create_save_json(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            ARRAY_CONTAINS => self.remove_array_contains(entity_instance),
            ARRAY_GET_BY_INDEX => self.remove_array_get_by_index(entity_instance),
            ARRAY_LENGTH => self.remove_array_length(entity_instance),
            ARRAY_POP => self.remove_array_pop(entity_instance),
            ARRAY_PUSH => self.remove_array_push(entity_instance),
            ARRAY_REVERSE => self.remove_array_reverse(entity_instance),
            OBJECT_GET_PROPERTY => self.remove_object_get_property(entity_instance),
            OBJECT_KEYS => self.remove_object_keys(entity_instance),
            OBJECT_REMOVE_PROPERTY => self.remove_object_remove_property(entity_instance),
            OBJECT_SET_PROPERTY => self.remove_object_set_property(entity_instance),
            LOAD_JSON => self.remove_load_json(entity_instance),
            SAVE_JSON => self.remove_save_json(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
