use std::sync::Arc;

use crate::di::*;
use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::entity::pseudo_random_number::PseudoRandomNumber;
use crate::behaviour::entity::pseudo_random_number::PSEUDO_RANDOM_NUMBER;
use crate::behaviour::entity::random_bool::RandomBool;
use crate::behaviour::entity::random_bool::RANDOM_BOOL;
use crate::behaviour::entity::random_integer_within_range::RandomIntegerWithinRange;
use crate::behaviour::entity::random_integer_within_range::RANDOM_INTEGER_WITHIN_RANGE;
use crate::behaviour::entity::random_number::RandomNumber;
use crate::behaviour::entity::random_number::RANDOM_NUMBER;
use crate::behaviour::entity::random_string::RandomString;
use crate::behaviour::entity::random_string::RANDOM_STRING;
use crate::behaviour::entity::random_uuid::RandomUuid;
use crate::behaviour::entity::random_uuid::RANDOM_UUID;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct PseudoRandomNumberStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<PseudoRandomNumber>>>);

#[wrapper]
pub struct RandomBoolStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<RandomBool>>>);

#[wrapper]
pub struct RandomIntegerWithinRangeStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<RandomIntegerWithinRange>>>);

#[wrapper]
pub struct RandomNumberStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<RandomNumber>>>);

#[wrapper]
pub struct RandomStringStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<RandomString>>>);

#[wrapper]
pub struct RandomUuidStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<RandomUuid>>>);

#[provides]
fn create_pseudo_random_number_storage() -> PseudoRandomNumberStorage {
    PseudoRandomNumberStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_random_bool_storage() -> RandomBoolStorage {
    RandomBoolStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_random_integer_within_range_storage() -> RandomIntegerWithinRangeStorage {
    RandomIntegerWithinRangeStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_random_number_storage() -> RandomNumberStorage {
    RandomNumberStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_random_string_storage() -> RandomStringStorage {
    RandomStringStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_random_uuid_storage() -> RandomUuidStorage {
    RandomUuidStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait RandomEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_pseudo_random_number(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_random_bool(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_random_integer_within_range(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_random_number(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_random_string(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_random_uuid(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_pseudo_random_number(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_random_bool(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_random_integer_within_range(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_random_number(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_random_string(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_random_uuid(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct RandomEntityBehaviourProviderImpl {
    pseudo_random_number: PseudoRandomNumberStorage,
    random_bool: RandomBoolStorage,
    random_integer_within_range: RandomIntegerWithinRangeStorage,
    random_number: RandomNumberStorage,
    random_string: RandomStringStorage,
    random_uuid: RandomUuidStorage,
}

interfaces!(RandomEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl RandomEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            pseudo_random_number: create_pseudo_random_number_storage(),
            random_bool: create_random_bool_storage(),
            random_integer_within_range: create_random_integer_within_range_storage(),
            random_number: create_random_number_storage(),
            random_string: create_random_string_storage(),
            random_uuid: create_random_uuid_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl RandomEntityBehaviourProvider for RandomEntityBehaviourProviderImpl {
    fn create_pseudo_random_number(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match PseudoRandomNumber::new(entity_instance.clone()) {
            Ok(pseudo_random_number) => {
                self.pseudo_random_number.0.write().unwrap().insert(id, Arc::new(pseudo_random_number));
                entity_instance.add_behaviour(PSEUDO_RANDOM_NUMBER);
                debug!("Added behaviour {} to entity instance {}", PSEUDO_RANDOM_NUMBER, id);
            }
            _ => {}
        }
    }

    fn create_random_bool(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match RandomBool::new(entity_instance.clone()) {
            Ok(random_bool) => {
                self.random_bool.0.write().unwrap().insert(id, Arc::new(random_bool));
                entity_instance.add_behaviour(RANDOM_BOOL);
                debug!("Added behaviour {} to entity instance {}", RANDOM_BOOL, id);
            }
            _ => {}
        }
    }

    fn create_random_integer_within_range(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match RandomIntegerWithinRange::new(entity_instance.clone()) {
            Ok(random_integer_within_range) => {
                self.random_integer_within_range
                    .0
                    .write()
                    .unwrap()
                    .insert(id, Arc::new(random_integer_within_range));
                entity_instance.add_behaviour(RANDOM_INTEGER_WITHIN_RANGE);
                debug!("Added behaviour {} to entity instance {}", RANDOM_INTEGER_WITHIN_RANGE, id);
            }
            _ => {}
        }
    }

    fn create_random_number(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match RandomNumber::new(entity_instance.clone()) {
            Ok(random_number) => {
                self.random_number.0.write().unwrap().insert(id, Arc::new(random_number));
                entity_instance.add_behaviour(RANDOM_NUMBER);
                debug!("Added behaviour {} to entity instance {}", RANDOM_NUMBER, id);
            }
            _ => {}
        }
    }

    fn create_random_string(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match RandomString::new(entity_instance.clone()) {
            Ok(random_string) => {
                self.random_string.0.write().unwrap().insert(id, Arc::new(random_string));
                entity_instance.add_behaviour(RANDOM_STRING);
                debug!("Added behaviour {} to entity instance {}", RANDOM_STRING, id);
            }
            _ => {}
        }
    }

    fn create_random_uuid(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match RandomUuid::new(entity_instance.clone()) {
            Ok(random_uuid) => {
                self.random_uuid.0.write().unwrap().insert(id, Arc::new(random_uuid));
                entity_instance.add_behaviour(RANDOM_UUID);
                debug!("Added behaviour {} to entity instance {}", RANDOM_UUID, id);
            }
            _ => {}
        }
    }

    fn remove_pseudo_random_number(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.pseudo_random_number.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(PSEUDO_RANDOM_NUMBER);
        debug!("Removed behaviour {} from entity instance {}", PSEUDO_RANDOM_NUMBER, entity_instance.id);
    }

    fn remove_random_bool(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.random_bool.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(RANDOM_BOOL);
        debug!("Removed behaviour {} from entity instance {}", RANDOM_BOOL, entity_instance.id);
    }

    fn remove_random_integer_within_range(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.random_integer_within_range.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(RANDOM_INTEGER_WITHIN_RANGE);
        debug!("Removed behaviour {} from entity instance {}", RANDOM_INTEGER_WITHIN_RANGE, entity_instance.id);
    }

    fn remove_random_number(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.random_number.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(RANDOM_NUMBER);
        debug!("Removed behaviour {} from entity instance {}", RANDOM_NUMBER, entity_instance.id);
    }

    fn remove_random_string(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.random_number.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(RANDOM_STRING);
        debug!("Removed behaviour {} from entity instance {}", RANDOM_STRING, entity_instance.id);
    }

    fn remove_random_uuid(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.random_uuid.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(RANDOM_UUID);
        debug!("Removed behaviour {} from entity instance {}", RANDOM_UUID, entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.pseudo_random_number.0.write().unwrap().contains_key(&id) {
            self.pseudo_random_number.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", PSEUDO_RANDOM_NUMBER, id);
        }
        if self.random_bool.0.write().unwrap().contains_key(&id) {
            self.random_bool.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", RANDOM_BOOL, id);
        }
        if self.random_integer_within_range.0.write().unwrap().contains_key(&id) {
            self.random_integer_within_range.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", RANDOM_INTEGER_WITHIN_RANGE, id);
        }
        if self.random_number.0.write().unwrap().contains_key(&id) {
            self.random_number.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", RANDOM_NUMBER, id);
        }
        if self.random_string.0.write().unwrap().contains_key(&id) {
            self.random_string.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", RANDOM_STRING, id);
        }
        if self.random_uuid.0.write().unwrap().contains_key(&id) {
            self.random_uuid.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", RANDOM_UUID, id);
        }
    }
}

impl EntityBehaviourProvider for RandomEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.type_name.as_str() {
            PSEUDO_RANDOM_NUMBER => self.create_pseudo_random_number(entity_instance),
            RANDOM_BOOL => self.create_random_bool(entity_instance),
            RANDOM_INTEGER_WITHIN_RANGE => self.create_random_integer_within_range(entity_instance),
            RANDOM_NUMBER => self.create_random_number(entity_instance),
            RANDOM_STRING => self.create_random_string(entity_instance),
            RANDOM_UUID => self.create_random_uuid(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            PSEUDO_RANDOM_NUMBER => self.remove_pseudo_random_number(entity_instance),
            RANDOM_BOOL => self.remove_random_bool(entity_instance),
            RANDOM_INTEGER_WITHIN_RANGE => self.remove_random_integer_within_range(entity_instance),
            RANDOM_NUMBER => self.remove_random_number(entity_instance),
            RANDOM_STRING => self.remove_random_string(entity_instance),
            RANDOM_UUID => self.remove_random_uuid(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
