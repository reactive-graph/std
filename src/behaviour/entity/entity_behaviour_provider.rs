use std::sync::Arc;

use crate::di::*;
use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::entity::load_binary_data::LoadBinaryData;
use crate::behaviour::entity::load_binary_data::LOAD_BINARY_DATA;
use crate::behaviour::entity::save_binary_data::SaveBinaryData;
use crate::behaviour::entity::save_binary_data::SAVE_BINARY_DATA;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct LoadBinaryDataStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<LoadBinaryData>>>);

#[wrapper]
pub struct SaveBinaryDataStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<SaveBinaryData>>>);

#[provides]
fn create_load_binary_data_storage() -> LoadBinaryDataStorage {
    LoadBinaryDataStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_save_binary_data_storage() -> SaveBinaryDataStorage {
    SaveBinaryDataStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait BinaryEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_load_binary_data(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_save_binary_data(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_load_binary_data(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_save_binary_data(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct BinaryEntityBehaviourProviderImpl {
    load_binary_data: LoadBinaryDataStorage,
    save_binary_data: SaveBinaryDataStorage,
}

interfaces!(BinaryEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl BinaryEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            load_binary_data: create_load_binary_data_storage(),
            save_binary_data: create_save_binary_data_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl BinaryEntityBehaviourProvider for BinaryEntityBehaviourProviderImpl {
    fn create_load_binary_data(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let load_binary_data = LoadBinaryData::new(entity_instance.clone());
        if load_binary_data.is_ok() {
            let load_binary_data = Arc::new(load_binary_data.unwrap());
            self.load_binary_data.0.write().unwrap().insert(id, load_binary_data);
            entity_instance.add_component("binary_data");
            entity_instance.add_behaviour(LOAD_BINARY_DATA);
            debug!("Added behaviour {} to entity instance {}", LOAD_BINARY_DATA, id);
        }
    }

    fn create_save_binary_data(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let save_binary_data = SaveBinaryData::new(entity_instance.clone());
        if save_binary_data.is_ok() {
            let save_binary_data = Arc::new(save_binary_data.unwrap());
            self.save_binary_data.0.write().unwrap().insert(id, save_binary_data);
            entity_instance.add_component("binary_data");
            entity_instance.add_behaviour(SAVE_BINARY_DATA);
            debug!("Added behaviour {} to entity instance {}", SAVE_BINARY_DATA, id);
        }
    }

    fn remove_load_binary_data(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.load_binary_data.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(LOAD_BINARY_DATA);
        debug!("Removed behaviour {} from entity instance {}", LOAD_BINARY_DATA, entity_instance.id);
    }

    fn remove_save_binary_data(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.save_binary_data.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(SAVE_BINARY_DATA);
        debug!("Removed behaviour {} from entity instance {}", SAVE_BINARY_DATA, entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.load_binary_data.0.write().unwrap().contains_key(&id) {
            self.load_binary_data.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", LOAD_BINARY_DATA, id);
        }
        if self.save_binary_data.0.write().unwrap().contains_key(&id) {
            self.save_binary_data.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", SAVE_BINARY_DATA, id);
        }
    }
}

impl EntityBehaviourProvider for BinaryEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            LOAD_BINARY_DATA => self.create_load_binary_data(entity_instance),
            SAVE_BINARY_DATA => self.create_save_binary_data(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            LOAD_BINARY_DATA => self.remove_load_binary_data(entity_instance),
            SAVE_BINARY_DATA => self.remove_save_binary_data(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
