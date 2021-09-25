use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;
use waiter_di::*;

use crate::behaviour::entity::config_file::ConfigFile;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

const CONFIG_FILE: &'static str = "config_file";

#[wrapper]
pub struct ConfigFileStorage(
    std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<ConfigFile>>>,
);

#[waiter_di::provides]
fn create_config_file_behaviours_storage() -> ConfigFileStorage {
    ConfigFileStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait ConfigEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_config_file_behaviour(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_config_file_behaviour(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

// #[derive(Clone)]
pub struct ConfigEntityBehaviourProviderImpl {
    config_file_behaviours: ConfigFileStorage,
}

interfaces!(ConfigEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl ConfigEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            config_file_behaviours: create_config_file_behaviours_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl ConfigEntityBehaviourProvider for ConfigEntityBehaviourProviderImpl {
    fn create_config_file_behaviour(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let config_file = ConfigFile::new(entity_instance.clone());
        if config_file.is_ok() {
            let config_file = Arc::new(config_file.unwrap());
            self.config_file_behaviours
                .0
                .write()
                .unwrap()
                .insert(id, config_file);
            debug!("Added behaviour {} to entity instance {}", CONFIG_FILE, id);
            // The initial tick is necessary for reading the config file the first time
            entity_instance.tick();
        }
    }

    fn remove_config_file_behaviour(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.config_file_behaviours
            .0
            .write()
            .unwrap()
            .remove(&entity_instance.id);
        debug!(
            "Removed behaviour {} from entity instance {}",
            CONFIG_FILE, entity_instance.id
        );
    }

    fn remove_by_id(&self, id: Uuid) {
        if self
            .config_file_behaviours
            .0
            .write()
            .unwrap()
            .contains_key(&id)
        {
            self.config_file_behaviours.0.write().unwrap().remove(&id);
            debug!(
                "Removed behaviour {} from entity instance {}",
                CONFIG_FILE, id
            );
        }
    }
}

impl EntityBehaviourProvider for ConfigEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            CONFIG_FILE => self.create_config_file_behaviour(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            CONFIG_FILE => self.remove_config_file_behaviour(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
