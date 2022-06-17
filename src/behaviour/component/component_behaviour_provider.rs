use std::sync::Arc;

use crate::di::*;
use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::component::load_binary_data::LoadBinaryData;
use crate::behaviour::component::load_binary_data::LOAD_BINARY_DATA;
use crate::behaviour::component::save_binary_data::SaveBinaryData;
use crate::behaviour::component::save_binary_data::SAVE_BINARY_DATA;
use crate::model::ReactiveEntityInstance;
use crate::plugins::ComponentBehaviourProvider;

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
pub trait BinaryComponentBehaviourProvider: ComponentBehaviourProvider + Send + Sync {
    fn create_load_binary_data(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_save_binary_data(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_load_binary_data(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_save_binary_data(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

#[component]
pub struct BinaryComponentBehaviourProviderImpl {
    load_binary_data: LoadBinaryDataStorage,
    save_binary_data: SaveBinaryDataStorage,
}

interfaces!(BinaryComponentBehaviourProviderImpl: dyn ComponentBehaviourProvider);

impl BinaryComponentBehaviourProviderImpl {}

#[async_trait]
#[provides]
impl BinaryComponentBehaviourProvider for BinaryComponentBehaviourProviderImpl {
    fn create_load_binary_data(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        if let Ok(load_binary_data) = LoadBinaryData::new(entity_instance.clone()) {
            let load_binary_data = Arc::new(load_binary_data);
            self.load_binary_data.0.write().unwrap().insert(id, load_binary_data);
            entity_instance.add_component("binary_data");
            entity_instance.add_behaviour(LOAD_BINARY_DATA);
            debug!("Added behaviour {} to entity instance {}", LOAD_BINARY_DATA, id);
        }
    }

    fn create_save_binary_data(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        if let Ok(save_binary_data) = SaveBinaryData::new(entity_instance.clone()) {
            let save_binary_data = Arc::new(save_binary_data);
            self.save_binary_data.0.write().unwrap().insert(id, save_binary_data);
            entity_instance.add_component("binary_data");
            entity_instance.add_behaviour(SAVE_BINARY_DATA);
            debug!("Added behaviour {} to entity instance {}", SAVE_BINARY_DATA, id);
        }
    }

    fn remove_load_binary_data(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if let Some(_) = self.load_binary_data.0.write().unwrap().remove(&entity_instance.id) {
            entity_instance.remove_behaviour(LOAD_BINARY_DATA);
            debug!("Removed behaviour {} from entity instance {}", LOAD_BINARY_DATA, entity_instance.id);
        }
    }

    fn remove_save_binary_data(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if let Some(_) = self.save_binary_data.0.write().unwrap().remove(&entity_instance.id) {
            entity_instance.remove_behaviour(SAVE_BINARY_DATA);
            debug!("Removed behaviour {} from entity instance {}", SAVE_BINARY_DATA, entity_instance.id);
        }
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.load_binary_data.0.write().unwrap().contains_key(&id) {
            if let Some(_) = self.load_binary_data.0.write().unwrap().remove(&id) {
                debug!("Removed behaviour {} from entity instance {}", LOAD_BINARY_DATA, id);
            }
        }
        if self.save_binary_data.0.write().unwrap().contains_key(&id) {
            if let Some(_) = self.save_binary_data.0.write().unwrap().remove(&id) {
                debug!("Removed behaviour {} from entity instance {}", SAVE_BINARY_DATA, id);
            }
        }
    }
}

impl ComponentBehaviourProvider for BinaryComponentBehaviourProviderImpl {
    fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if entity_instance.is_a(LOAD_BINARY_DATA) {
            self.create_load_binary_data(entity_instance.clone());
        }
        if entity_instance.is_a(SAVE_BINARY_DATA) {
            self.create_save_binary_data(entity_instance);
        }
    }

    fn add_behaviours_to_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        match component.name.as_str() {
            LOAD_BINARY_DATA => self.create_load_binary_data(entity_instance),
            SAVE_BINARY_DATA => self.create_save_binary_data(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if entity_instance.behaves_as(LOAD_BINARY_DATA) {
            self.remove_load_binary_data(entity_instance.clone());
        }
        if entity_instance.behaves_as(SAVE_BINARY_DATA) {
            self.remove_save_binary_data(entity_instance);
        }
    }

    fn remove_behaviours_from_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        match component.name.as_str() {
            LOAD_BINARY_DATA => self.remove_load_binary_data(entity_instance),
            SAVE_BINARY_DATA => self.remove_save_binary_data(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
