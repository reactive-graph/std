use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::component::load_json::LoadJson;
use crate::behaviour::component::load_json::LOAD_JSON;
use crate::behaviour::component::save_json::SaveJson;
use crate::behaviour::component::save_json::SAVE_JSON;
use crate::di::*;
use crate::model::ReactiveEntityInstance;
use crate::plugins::ComponentBehaviourProvider;

#[wrapper]
pub struct LoadJsonStorage(RwLock<HashMap<Uuid, Arc<LoadJson>>>);

#[wrapper]
pub struct SaveJsonStorage(RwLock<HashMap<Uuid, Arc<SaveJson>>>);

#[provides]
fn create_load_json_storage() -> LoadJsonStorage {
    LoadJsonStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_save_json_storage() -> SaveJsonStorage {
    SaveJsonStorage(RwLock::new(HashMap::new()))
}

#[async_trait]
pub trait JsonComponentBehaviourProvider: ComponentBehaviourProvider + Send + Sync {
    fn create_load_json(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_save_json(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_load_json(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_save_json(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

#[component]
pub struct JsonComponentBehaviourProviderImpl {
    load_json: LoadJsonStorage,
    save_json: SaveJsonStorage,
}

interfaces!(JsonComponentBehaviourProviderImpl: dyn ComponentBehaviourProvider);

impl JsonComponentBehaviourProviderImpl {}

#[async_trait]
#[provides]
impl JsonComponentBehaviourProvider for JsonComponentBehaviourProviderImpl {
    fn create_load_json(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        if let Ok(load_json) = LoadJson::new(entity_instance.clone()) {
            self.load_json.0.write().unwrap().insert(id, Arc::new(load_json));
            entity_instance.add_behaviour(LOAD_JSON);
            debug!("Added behaviour {} to entity instance {}", LOAD_JSON, id);
        }
    }

    fn create_save_json(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        if let Ok(save_json) = SaveJson::new(entity_instance.clone()) {
            self.save_json.0.write().unwrap().insert(id, Arc::new(save_json));
            entity_instance.add_behaviour(SAVE_JSON);
            debug!("Added behaviour {} to entity instance {}", SAVE_JSON, id);
        }
    }

    fn remove_load_json(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if let Some(_) = self.load_json.0.write().unwrap().remove(&entity_instance.id) {
            entity_instance.remove_behaviour(LOAD_JSON);
            debug!("Removed behaviour {} from entity instance {}", LOAD_JSON, entity_instance.id);
        }
    }

    fn remove_save_json(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if let Some(_) = self.save_json.0.write().unwrap().remove(&entity_instance.id) {
            entity_instance.remove_behaviour(SAVE_JSON);
            debug!("Removed behaviour {} from entity instance {}", SAVE_JSON, entity_instance.id);
        }
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.load_json.0.write().unwrap().contains_key(&id) {
            if let Some(_) = self.load_json.0.write().unwrap().remove(&id) {
                debug!("Removed behaviour {} from entity instance {}", LOAD_JSON, id);
            }
        }
        if self.save_json.0.write().unwrap().contains_key(&id) {
            if let Some(_) = self.save_json.0.write().unwrap().remove(&id) {
                debug!("Removed behaviour {} from entity instance {}", SAVE_JSON, id);
            }
        }
    }
}

impl ComponentBehaviourProvider for JsonComponentBehaviourProviderImpl {
    fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if entity_instance.is_a(LOAD_JSON) {
            self.create_load_json(entity_instance.clone());
        }
        if entity_instance.is_a(SAVE_JSON) {
            self.create_save_json(entity_instance.clone());
        }
    }

    fn add_behaviours_to_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        match component.name.as_str() {
            LOAD_JSON => self.create_load_json(entity_instance),
            SAVE_JSON => self.create_save_json(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if entity_instance.behaves_as(LOAD_JSON) {
            self.remove_load_json(entity_instance.clone());
        }
        if entity_instance.behaves_as(SAVE_JSON) {
            self.remove_save_json(entity_instance.clone());
        }
    }

    fn remove_behaviours_from_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        match component.name.as_str() {
            LOAD_JSON => self.remove_load_json(entity_instance),
            SAVE_JSON => self.remove_save_json(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
