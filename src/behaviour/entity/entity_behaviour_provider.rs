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
        let id = entity_instance.id;
        let function = COMPARISON_GATES.get(entity_instance.type_name.as_str());
        let comparison_gate = match function {
            Some(function) => Some(Arc::new(ComparisonGate::new(entity_instance.clone(), *function))),
            None => None,
        };
        if comparison_gate.is_some() {
            self.comparison_gates.0.write().unwrap().insert(id, comparison_gate.unwrap());
            debug!("Added behaviour comparison_gate to entity instance {}", id);
        }
    }

    fn remove_comparison_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.comparison_gates.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour comparison_gates from entity instance {}", entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.comparison_gates.0.write().unwrap().contains_key(&id) {
            self.comparison_gates.0.write().unwrap().remove(&id);
            debug!("Removed behaviour comparison_gates from entity instance {}", id);
        }
    }
}

impl EntityBehaviourProvider for ComparisonEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.create_comparison_gate(entity_instance.clone());
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.remove_comparison_gate(entity_instance.clone());
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
