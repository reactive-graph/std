use std::sync::Arc;

use crate::di::*;
use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::entity::comparison::{StringComparison, STRING_COMPARISONS};
use crate::behaviour::entity::gate::{StringGate, STRING_GATES};
use crate::behaviour::entity::operation::{StringOperation, STRING_OPERATIONS};
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct StringOperationStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<StringOperation<'static>>>>);

#[wrapper]
pub struct StringGateStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<StringGate<'static>>>>);

#[wrapper]
pub struct StringComparisonStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<StringComparison<'static>>>>);

#[provides]
fn create_string_operation_storage() -> StringOperationStorage {
    StringOperationStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_string_gate_storage() -> StringGateStorage {
    StringGateStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_string_comparison_storage() -> StringComparisonStorage {
    StringComparisonStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait StringEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_string_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_string_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_string_comparison(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_string_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_string_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_string_comparison(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct StringEntityBehaviourProviderImpl {
    string_operations: StringOperationStorage,
    string_gates: StringGateStorage,
    string_comparisons: StringComparisonStorage,
}

interfaces!(StringEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl StringEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            string_operations: create_string_operation_storage(),
            string_gates: create_string_gate_storage(),
            string_comparisons: create_string_comparison_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl StringEntityBehaviourProvider for StringEntityBehaviourProviderImpl {
    fn create_string_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let function = STRING_OPERATIONS.get(entity_instance.type_name.as_str());
        let string_operation = match function {
            Some(function) => Some(Arc::new(StringOperation::new(entity_instance.clone(), *function))),
            None => None,
        };
        if string_operation.is_some() {
            self.string_operations.0.write().unwrap().insert(id, string_operation.unwrap());
            debug!("Added behaviour string_operation to entity instance {}", id);
        }
    }

    fn create_string_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let function = STRING_GATES.get(entity_instance.type_name.as_str());
        let string_gate = match function {
            Some(function) => Some(Arc::new(StringGate::new(entity_instance.clone(), *function))),
            None => None,
        };
        if string_gate.is_some() {
            self.string_gates.0.write().unwrap().insert(id, string_gate.unwrap());
            debug!("Added behaviour string_gate to entity instance {}", id);
        }
    }

    fn create_string_comparison(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let function = STRING_COMPARISONS.get(entity_instance.type_name.as_str());
        let string_comparison = match function {
            Some(function) => Some(Arc::new(StringComparison::new(entity_instance.clone(), *function))),
            None => None,
        };
        if string_comparison.is_some() {
            self.string_comparisons.0.write().unwrap().insert(id, string_comparison.unwrap());
            debug!("Added behaviour string_comparison to entity instance {}", id);
        }
    }

    fn remove_string_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.string_operations.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour string_operation from entity instance {}", entity_instance.id);
    }

    fn remove_string_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.string_gates.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour string_gate from entity instance {}", entity_instance.id);
    }

    fn remove_string_comparison(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.string_comparisons.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour string_comparison from entity instance {}", entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.string_operations.0.write().unwrap().contains_key(&id) {
            self.string_operations.0.write().unwrap().remove(&id);
            debug!("Removed behaviour string_operation from entity instance {}", id);
        }
        if self.string_gates.0.write().unwrap().contains_key(&id) {
            self.string_gates.0.write().unwrap().remove(&id);
            debug!("Removed behaviour string_gate from entity instance {}", id);
        }
    }
}

impl EntityBehaviourProvider for StringEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.create_string_operation(entity_instance.clone());
        self.create_string_gate(entity_instance.clone());
        self.create_string_comparison(entity_instance.clone());
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.remove_string_operation(entity_instance.clone());
        self.remove_string_gate(entity_instance.clone());
        self.remove_string_comparison(entity_instance.clone());
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
