use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;
use waiter_di::*;

use crate::behaviour::entity::gate::ArithmeticGate;
use crate::behaviour::entity::gate::ARITHMETIC_GATES;
use crate::behaviour::entity::operation::ArithmeticOperation;
use crate::behaviour::entity::operation::ARITHMETIC_OPERATIONS;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct ArithmeticOperationStorage(
    std::sync::RwLock<
        std::collections::HashMap<Uuid, std::sync::Arc<ArithmeticOperation<'static>>>,
    >,
);

#[wrapper]
pub struct ArithmeticGateStorage(
    std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<ArithmeticGate<'static>>>>,
);

#[waiter_di::provides]
fn create_arithmetic_operation_storage() -> ArithmeticOperationStorage {
    ArithmeticOperationStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[waiter_di::provides]
fn create_arithmetic_gate_storage() -> ArithmeticGateStorage {
    ArithmeticGateStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait ArithmeticEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct ArithmeticEntityBehaviourProviderImpl {
    arithmetic_operations: ArithmeticOperationStorage,
    arithmetic_gates: ArithmeticGateStorage,
}

interfaces!(ArithmeticEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl ArithmeticEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            arithmetic_operations: create_arithmetic_operation_storage(),
            arithmetic_gates: create_arithmetic_gate_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl ArithmeticEntityBehaviourProvider for ArithmeticEntityBehaviourProviderImpl {
    fn create_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let function = ARITHMETIC_OPERATIONS.get(entity_instance.type_name.as_str());
        let arithmetic_operation = match function {
            Some(function) => Some(Arc::new(ArithmeticOperation::new(
                entity_instance.clone(),
                *function,
            ))),
            None => None,
        };
        if arithmetic_operation.is_some() {
            // let arithmetic_operation = Arc::new(arithmetic_operation.unwrap());
            self.arithmetic_operations
                .0
                .write()
                .unwrap()
                .insert(id, arithmetic_operation.unwrap());
            debug!(
                "Added behaviour arithmetic_operation to entity instance {}",
                id
            );
        }
    }

    fn create_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let function = ARITHMETIC_GATES.get(entity_instance.type_name.as_str());
        let arithmetic_gate = match function {
            Some(function) => Some(Arc::new(ArithmeticGate::new(
                entity_instance.clone(),
                *function,
            ))),
            None => None,
        };
        if arithmetic_gate.is_some() {
            self.arithmetic_gates
                .0
                .write()
                .unwrap()
                .insert(id, arithmetic_gate.unwrap());
            debug!("Added behaviour arithmetic_gate to entity instance {}", id);
        }
    }

    fn remove_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.arithmetic_operations
            .0
            .write()
            .unwrap()
            .remove(&entity_instance.id);
        debug!(
            "Removed behaviour arithmetic_operation from entity instance {}",
            entity_instance.id
        );
    }

    fn remove_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.arithmetic_gates
            .0
            .write()
            .unwrap()
            .remove(&entity_instance.id);
        debug!(
            "Removed behaviour arithmetic_gates from entity instance {}",
            entity_instance.id
        );
    }

    fn remove_by_id(&self, id: Uuid) {
        if self
            .arithmetic_operations
            .0
            .write()
            .unwrap()
            .contains_key(&id)
        {
            self.arithmetic_operations.0.write().unwrap().remove(&id);
            debug!(
                "Removed behaviour arithmetic_operation from entity instance {}",
                id
            );
        }
        if self.arithmetic_gates.0.write().unwrap().contains_key(&id) {
            self.arithmetic_gates.0.write().unwrap().remove(&id);
            debug!(
                "Removed behaviour arithmetic_gates from entity instance {}",
                id
            );
        }
    }
}

impl EntityBehaviourProvider for ArithmeticEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.create_operation(entity_instance.clone());
        self.create_gate(entity_instance.clone());
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.remove_operation(entity_instance.clone());
        self.remove_gate(entity_instance.clone());
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
