use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;
use waiter_di::*;

use crate::behaviour::entity::gate::LogicalGate;
use crate::behaviour::entity::gate::LOGICAL_GATES;
use crate::behaviour::entity::operation::LogicalOperation;
use crate::behaviour::entity::operation::LOGICAL_OPERATIONS;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct LogicalOperationStorage(
    std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<LogicalOperation<'static>>>>,
);

#[wrapper]
pub struct LogicalGateStorage(
    std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<LogicalGate<'static>>>>,
);

#[waiter_di::provides]
fn create_logical_operation_storage() -> LogicalOperationStorage {
    LogicalOperationStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[waiter_di::provides]
fn create_logical_gate_storage() -> LogicalGateStorage {
    LogicalGateStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait LogicalEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_logical_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_logical_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_logical_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_logical_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

// #[derive(Clone)]
pub struct LogicalEntityBehaviourProviderImpl {
    logical_operations: LogicalOperationStorage,
    logical_gates: LogicalGateStorage,
}

interfaces!(LogicalEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl LogicalEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            logical_operations: create_logical_operation_storage(),
            logical_gates: create_logical_gate_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl LogicalEntityBehaviourProvider for LogicalEntityBehaviourProviderImpl {
    fn create_logical_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let function = LOGICAL_OPERATIONS.get(entity_instance.type_name.as_str());
        let logical_operation = match function {
            Some(function) => Some(Arc::new(LogicalOperation::new(
                entity_instance.clone(),
                *function,
            ))),
            None => None,
        };
        if logical_operation.is_some() {
            // let logical_operation = Arc::new(logical_operation.unwrap());
            self.logical_operations
                .0
                .write()
                .unwrap()
                .insert(id, logical_operation.unwrap());
            debug!(
                "Added behaviour logical_operation to entity instance {}",
                id
            );
        }
    }

    fn create_logical_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let function = LOGICAL_GATES.get(entity_instance.type_name.as_str());
        let logical_gate = match function {
            Some(function) => Some(Arc::new(LogicalGate::new(
                entity_instance.clone(),
                *function,
            ))),
            None => None,
        };
        if logical_gate.is_some() {
            // let logical_operation = Arc::new(logical_operation.unwrap());
            self.logical_gates
                .0
                .write()
                .unwrap()
                .insert(id, logical_gate.unwrap());
            debug!("Added behaviour logical_gate to entity instance {}", id);
        }
    }

    fn remove_logical_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.logical_operations
            .0
            .write()
            .unwrap()
            .remove(&entity_instance.id);
        debug!(
            "Removed behaviour logical_operation from entity instance {}",
            entity_instance.id
        );
    }

    fn remove_logical_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.logical_gates
            .0
            .write()
            .unwrap()
            .remove(&entity_instance.id);
        debug!(
            "Removed behaviour logical_gates from entity instance {}",
            entity_instance.id
        );
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.logical_operations.0.write().unwrap().contains_key(&id) {
            self.logical_operations.0.write().unwrap().remove(&id);
            debug!(
                "Removed behaviour logical_operation from entity instance {}",
                id
            );
        }
        if self.logical_gates.0.write().unwrap().contains_key(&id) {
            self.logical_gates.0.write().unwrap().remove(&id);
            debug!(
                "Removed behaviour logical_gates from entity instance {}",
                id
            );
        }
    }
}

impl EntityBehaviourProvider for LogicalEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.create_logical_operation(entity_instance.clone());
        self.create_logical_gate(entity_instance.clone());
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.remove_logical_operation(entity_instance.clone());
        self.remove_logical_gate(entity_instance.clone());
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
