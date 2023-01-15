use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use crate::api::RuntimeManager;
use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::component::fs_notify::FsNotify;
use crate::behaviour::component::fs_notify::FS_NOTIFY;
use crate::di::*;
use crate::model::ReactiveEntityInstance;
use crate::plugins::ComponentBehaviourProvider;

#[wrapper]
pub struct FsNotifyStorage(RwLock<HashMap<Uuid, Arc<FsNotify>>>);

#[provides]
fn create_fs_notify_storage() -> FsNotifyStorage {
    FsNotifyStorage(RwLock::new(HashMap::new()))
}

#[async_trait]
pub trait FileComponentBehaviourProvider: ComponentBehaviourProvider + Send + Sync {
    fn create_fs_notify(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_fs_notify(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

#[component]
pub struct FileComponentBehaviourProviderImpl {
    fs_notify: FsNotifyStorage,

    runtime_manager: Wrc<dyn RuntimeManager>,
}

interfaces!(FileComponentBehaviourProviderImpl: dyn ComponentBehaviourProvider);

impl FileComponentBehaviourProviderImpl {}

#[async_trait]
#[provides]
impl FileComponentBehaviourProvider for FileComponentBehaviourProviderImpl {
    fn create_fs_notify(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        match FsNotify::new(entity_instance.clone(), self.runtime_manager.get_handle()) {
            Ok(fs_notify) => {
                self.fs_notify.0.write().unwrap().insert(id, Arc::new(fs_notify));
                entity_instance.add_behaviour(FS_NOTIFY);
                debug!("Added component behaviour {} to entity instance {}", FS_NOTIFY, id);
            }
            Err(_) => {
                debug!("Failed to add component behaviour {} to entity instance {}", FS_NOTIFY, id);
            }
        }
    }

    fn remove_fs_notify(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.fs_notify.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(FS_NOTIFY);
        debug!("Removed component behaviour {} from entity instance {}", FS_NOTIFY, entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.fs_notify.0.write().unwrap().contains_key(&id) {
            self.fs_notify.0.write().unwrap().remove(&id);
            debug!("Removed component behaviour {} from entity instance {}", FS_NOTIFY, id);
        }
    }
}

impl ComponentBehaviourProvider for FileComponentBehaviourProviderImpl {
    fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if entity_instance.is_a(FS_NOTIFY) {
            self.create_fs_notify(entity_instance);
        }
    }

    fn add_behaviours_to_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        if component.name == FS_NOTIFY {
            self.create_fs_notify(entity_instance)
        }
    }

    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if entity_instance.behaves_as(FS_NOTIFY) {
            self.remove_fs_notify(entity_instance);
        }
    }

    fn remove_behaviours_from_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        if component.name == FS_NOTIFY {
            self.remove_fs_notify(entity_instance);
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
