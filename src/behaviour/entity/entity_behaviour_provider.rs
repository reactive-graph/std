use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;
use waiter_di::*;

use crate::behaviour::entity::random_integer_within_range::RandomIntegerWithinRange;
use crate::behaviour::entity::random_integer_within_range::RANDOM_INTEGER_WITHIN_RANGE;
use crate::behaviour::entity::random_number::RandomNumber;
use crate::behaviour::entity::random_number::RANDOM_NUMBER;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct RandomNumberStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<RandomNumber>>>);

#[wrapper]
pub struct RandomIntegerWithinRangeStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<RandomIntegerWithinRange>>>);

#[waiter_di::provides]
fn create_random_number_storage() -> RandomNumberStorage {
    RandomNumberStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[waiter_di::provides]
fn create_random_integer_within_range_storage() -> RandomIntegerWithinRangeStorage {
    RandomIntegerWithinRangeStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait RandomEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_random_number(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_random_integer_within_range(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_random_number(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_random_integer_within_range(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct RandomEntityBehaviourProviderImpl {
    random_number: RandomNumberStorage,
    random_integer_within_range: RandomIntegerWithinRangeStorage,
}

interfaces!(RandomEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl RandomEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            random_number: create_random_number_storage(),
            random_integer_within_range: create_random_integer_within_range_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl RandomEntityBehaviourProvider for RandomEntityBehaviourProviderImpl {
    fn create_random_number(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match RandomNumber::new(entity_instance) {
            Ok(random_number) => {
                self.random_number.0.write().unwrap().insert(id, Arc::new(random_number));
                debug!("Added behaviour {} to entity instance {}", RANDOM_NUMBER, id);
            }
            _ => {}
        }
    }

    fn create_random_integer_within_range(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match RandomIntegerWithinRange::new(entity_instance) {
            Ok(random_integer_within_range) => {
                self.random_integer_within_range
                    .0
                    .write()
                    .unwrap()
                    .insert(id, Arc::new(random_integer_within_range));
                debug!("Added behaviour {} to entity instance {}", RANDOM_INTEGER_WITHIN_RANGE, id);
            }
            _ => {}
        }
    }

    fn remove_random_number(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.random_number.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour {} from entity instance {}", RANDOM_NUMBER, entity_instance.id);
    }

    fn remove_random_integer_within_range(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.random_integer_within_range.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour {} from entity instance {}", RANDOM_INTEGER_WITHIN_RANGE, entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.random_number.0.write().unwrap().contains_key(&id) {
            self.random_number.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", RANDOM_NUMBER, id);
        }
        if self.random_integer_within_range.0.write().unwrap().contains_key(&id) {
            self.random_integer_within_range.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", RANDOM_INTEGER_WITHIN_RANGE, id);
        }
    }
}

impl EntityBehaviourProvider for RandomEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.type_name.as_str() {
            RANDOM_NUMBER => self.create_random_number(entity_instance),
            RANDOM_INTEGER_WITHIN_RANGE => self.create_random_integer_within_range(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            RANDOM_NUMBER => self.remove_random_number(entity_instance),
            RANDOM_INTEGER_WITHIN_RANGE => self.remove_random_integer_within_range(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
