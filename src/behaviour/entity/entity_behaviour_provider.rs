use std::sync::Arc;

use crate::di::*;
use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::entity::gate::ComparisonGate;
use crate::behaviour::entity::gate::COMPARISON_GATES;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct ComparisonGateStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<ComparisonGate<'static>>>>);

#[provides]
fn create_comparison_gate_storage() -> ComparisonGateStorage {
    ComparisonGateStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait ComparisonEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_comparison_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_comparison_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

// #[derive(Clone)]
pub struct ComparisonEntityBehaviourProviderImpl {
    comparison_gates: ComparisonGateStorage,
}

interfaces!(ComparisonEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl ComparisonEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            comparison_gates: create_comparison_gate_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl ComparisonEntityBehaviourProvider for ComparisonEntityBehaviourProviderImpl {
    fn create_comparison_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let type_name = entity_instance.type_name.as_str();
        let id = entity_instance.id;
        if let Some(comparison_gate) = COMPARISON_GATES
            .get(type_name)
            .map(|function| Arc::new(ComparisonGate::new(entity_instance.clone(), *function)))
        {
            self.comparison_gates.0.write().unwrap().insert(id, comparison_gate);
            entity_instance.add_behaviour(type_name);
            debug!("Added behaviour {} to entity instance {}", type_name, id);
        }
    }

    fn remove_comparison_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let type_name = entity_instance.type_name.as_str();
        if let Some(_) = self.comparison_gates.0.write().unwrap().remove(&entity_instance.id) {
            entity_instance.remove_behaviour(type_name);
            debug!("Removed behaviour comparison_gates {} from entity instance {}", type_name, entity_instance.id);
        }
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.comparison_gates.0.write().unwrap().contains_key(&id) {
            if let Some(_) = self.comparison_gates.0.write().unwrap().remove(&id) {
                debug!("Removed behaviour comparison_gates from entity instance {}", id);
            }
        }
    }
}

impl EntityBehaviourProvider for ComparisonEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.create_comparison_gate(entity_instance);
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.remove_comparison_gate(entity_instance);
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
