use std::sync::Arc;

use crate::di::*;
use crate::plugins::ComponentBehaviourProvider;
use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::component::debug_value::DebugValue;
use crate::behaviour::component::debug_value::DEBUG_VALUE;
use crate::model::ReactiveEntityInstance;

#[wrapper]
pub struct DebugValueStorage(
    std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<DebugValue>>>,
);

#[provides]
fn create_debug_value_storage() -> DebugValueStorage {
    DebugValueStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait ValueComponentBehaviourProvider: ComponentBehaviourProvider + Send + Sync {
    fn create_debug_value(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_debug_value(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct ValueComponentBehaviourProviderImpl {
    debug_value: DebugValueStorage,
}

interfaces!(ValueComponentBehaviourProviderImpl: dyn ComponentBehaviourProvider);

#[component]
impl ValueComponentBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            debug_value: create_debug_value_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl ValueComponentBehaviourProvider for ValueComponentBehaviourProviderImpl {
    fn create_debug_value(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        if let Ok(debug_value) = DebugValue::new(entity_instance.clone()) {
            self.debug_value
                .0
                .write()
                .unwrap()
                .insert(id, Arc::new(debug_value));
            entity_instance.add_behaviour(DEBUG_VALUE);
            debug!("Added behaviour {} to entity instance {}", DEBUG_VALUE, id);
        }
    }

    fn remove_debug_value(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.debug_value
            .0
            .write()
            .unwrap()
            .remove(&entity_instance.id);
        entity_instance.remove_behaviour(DEBUG_VALUE);
        debug!(
            "Removed behaviour {} from entity instance {}",
            DEBUG_VALUE, entity_instance.id
        );
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.debug_value.0.write().unwrap().contains_key(&id) {
            self.debug_value.0.write().unwrap().remove(&id);
            debug!(
                "Removed behaviour {} from entity instance {}",
                DEBUG_VALUE, id
            );
        }
    }
}

impl ComponentBehaviourProvider for ValueComponentBehaviourProviderImpl {
    fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if entity_instance.is_a(DEBUG_VALUE) {
            self.create_debug_value(entity_instance);
        }
    }

    fn add_behaviours_to_entity_component(
        &self,
        entity_instance: Arc<ReactiveEntityInstance>,
        component: crate::model::Component,
    ) {
        if component.name == DEBUG_VALUE {
            self.create_debug_value(entity_instance)
        }
    }

    // fn add_behaviours_to_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) {}

    // fn add_behaviours_to_relation_component(
    //     &self,
    //     relation_instance: Arc<ReactiveRelationInstance>,
    //     component: crate::model::Component,
    // ) {
    // }

    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if entity_instance.behaves_as(DEBUG_VALUE) {
            self.remove_debug_value(entity_instance);
        }
    }

    fn remove_behaviours_from_entity_component(
        &self,
        entity_instance: Arc<ReactiveEntityInstance>,
        component: crate::model::Component,
    ) {
        if component.name == DEBUG_VALUE {
            self.remove_debug_value(entity_instance);
        }
    }

    // fn remove_behaviours_from_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) {}

    // fn remove_behaviours_from_relation_component(
    //     &self,
    //     relation_instance: Arc<ReactiveRelationInstance>,
    //     component: crate::model::Component,
    // ) {
    // }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }

    // fn remove_behaviours_by_key(&self, edge_key: EdgeKey) {}
}
