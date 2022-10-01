use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use indradb::EdgeKey;
use log::debug;

use crate::behaviour::component::behaviour::PROPAGATION_COUNTER;
use crate::behaviour::component::PropagationCounter;
use crate::di::*;
use crate::model::ComponentContainer;
use crate::model::ReactiveBehaviourContainer;
use crate::model::ReactiveRelationInstance;
use crate::plugins::ComponentBehaviourProvider;

#[wrapper]
pub struct PropagationCounterStorage(RwLock<HashMap<EdgeKey, Arc<PropagationCounter>>>);

#[provides]
fn create_propagation_counter_storage() -> PropagationCounterStorage {
    PropagationCounterStorage(RwLock::new(HashMap::new()))
}

#[async_trait]
pub trait ConnectorComponentBehaviourProvider: ComponentBehaviourProvider + Send + Sync {
    fn create_propagation_counter(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_propagation_counter(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_by_key(&self, edge_key: EdgeKey);
}

pub struct ConnectorComponentBehaviourProviderImpl {
    propagation_counters: PropagationCounterStorage,
}

interfaces!(ConnectorComponentBehaviourProviderImpl: dyn ComponentBehaviourProvider);

#[component]
impl ConnectorComponentBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            propagation_counters: create_propagation_counter_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl ConnectorComponentBehaviourProvider for ConnectorComponentBehaviourProviderImpl {
    fn create_propagation_counter(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        if let Some(edge_key) = relation_instance.get_key() {
            if let Ok(behaviour) = PropagationCounter::new(relation_instance.clone()) {
                self.propagation_counters.0.write().unwrap().insert(edge_key.clone(), Arc::new(behaviour));
                relation_instance.add_behaviour(PROPAGATION_COUNTER);
                debug!("Added behaviour {} to relation instance {:?}", PROPAGATION_COUNTER, edge_key);
            }
        }
    }

    fn remove_propagation_counter(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        if let Some(edge_key) = relation_instance.get_key() {
            if self.propagation_counters.0.write().unwrap().remove(&edge_key).is_some() {
                relation_instance.remove_behaviour(PROPAGATION_COUNTER);
                debug!("Removed behaviour {} from relation instance {:?}", PROPAGATION_COUNTER, edge_key);
            }
        }
    }

    fn remove_by_key(&self, edge_key: EdgeKey) {
        if self.propagation_counters.0.write().unwrap().contains_key(&edge_key) {
            self.propagation_counters.0.write().unwrap().remove(&edge_key);
            debug!("Removed behaviour {} from relation instance {:?}", PROPAGATION_COUNTER, edge_key);
        }
    }
}

impl ComponentBehaviourProvider for ConnectorComponentBehaviourProviderImpl {
    fn add_behaviours_to_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        if relation_instance.is_a(PROPAGATION_COUNTER) {
            self.create_propagation_counter(relation_instance);
        }
    }

    fn add_behaviours_to_relation_component(&self, relation_instance: Arc<ReactiveRelationInstance>, component: crate::model::Component) {
        if component.name == PROPAGATION_COUNTER {
            self.create_propagation_counter(relation_instance);
        }
    }

    fn remove_behaviours_from_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        if relation_instance.behaves_as(PROPAGATION_COUNTER) {
            self.remove_propagation_counter(relation_instance);
        }
    }

    fn remove_behaviours_from_relation_component(&self, relation_instance: Arc<ReactiveRelationInstance>, component: crate::model::Component) {
        if component.name == PROPAGATION_COUNTER {
            self.remove_propagation_counter(relation_instance);
        }
    }

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey) {
        self.remove_by_key(edge_key);
    }
}
