use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::entity::gate::NumericGate;
use crate::behaviour::entity::gate::NUMERIC_GATE;
use crate::behaviour::entity::gate::NUMERIC_GATES;
use crate::behaviour::entity::operation::NumericOperation;
use crate::behaviour::entity::operation::NUMERIC_OPERATION;
use crate::behaviour::entity::operation::NUMERIC_OPERATIONS;
use crate::di::*;
use crate::model::ReactiveBehaviourContainer;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct NumericOperationStorage(RwLock<BTreeMap<Uuid, Arc<NumericOperation<'static>>>>);

#[wrapper]
pub struct NumericGateStorage(RwLock<BTreeMap<Uuid, Arc<NumericGate<'static>>>>);

#[provides]
fn create_numeric_operation_storage() -> NumericOperationStorage {
    NumericOperationStorage(RwLock::new(BTreeMap::new()))
}

#[provides]
fn create_numeric_gate_storage() -> NumericGateStorage {
    NumericGateStorage(RwLock::new(BTreeMap::new()))
}

#[async_trait]
pub trait NumericEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_numeric_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_numeric_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_numeric_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_numeric_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct NumericEntityBehaviourProviderImpl {
    numeric_operations: NumericOperationStorage,
    numeric_gates: NumericGateStorage,
}

interfaces!(NumericEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl NumericEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            numeric_operations: create_numeric_operation_storage(),
            numeric_gates: create_numeric_gate_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl NumericEntityBehaviourProvider for NumericEntityBehaviourProviderImpl {
    fn create_numeric_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let numeric_operation = NUMERIC_OPERATIONS
            .get(entity_instance.type_name.as_str())
            .map(|function| Arc::new(NumericOperation::new(entity_instance.clone(), *function)));
        if let Some(numeric_operation) = numeric_operation {
            self.numeric_operations.0.write().unwrap().insert(id, numeric_operation);
            entity_instance.add_behaviour(entity_instance.type_name.as_str());
            debug!("Added behaviour {} {} to entity instance {}", NUMERIC_OPERATION, entity_instance.type_name.as_str(), id);
        }
    }

    fn create_numeric_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let numeric_gate = NUMERIC_GATES
            .get(entity_instance.type_name.as_str())
            .map(|function| Arc::new(NumericGate::new(entity_instance.clone(), *function)));
        if let Some(numeric_gate) = numeric_gate {
            self.numeric_gates.0.write().unwrap().insert(id, numeric_gate);
            entity_instance.add_behaviour(entity_instance.type_name.as_str());
            debug!("Added behaviour {} {} to entity instance {}", NUMERIC_GATE, entity_instance.type_name.as_str(), id);
        }
    }

    fn remove_numeric_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if self.numeric_operations.0.write().unwrap().remove(&entity_instance.id).is_some() {
            entity_instance.remove_behaviour(entity_instance.type_name.as_str());
            debug!(
                "Removed behaviour {} {} from entity instance {}",
                NUMERIC_OPERATION,
                entity_instance.type_name.as_str(),
                entity_instance.id
            );
        }
    }

    fn remove_numeric_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if self.numeric_gates.0.write().unwrap().remove(&entity_instance.id).is_some() {
            entity_instance.remove_behaviour(entity_instance.type_name.as_str());
            debug!(
                "Removed behaviour {} {} from entity instance {}",
                NUMERIC_GATE,
                entity_instance.type_name.as_str(),
                entity_instance.id
            );
        }
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.numeric_operations.0.write().unwrap().contains_key(&id) && self.numeric_operations.0.write().unwrap().remove(&id).is_some() {
            debug!("Removed behaviour {} from entity instance {}", NUMERIC_OPERATION, id);
        }
        if self.numeric_gates.0.write().unwrap().contains_key(&id) && self.numeric_gates.0.write().unwrap().remove(&id).is_some() {
            debug!("Removed behaviour {} from entity instance {}", NUMERIC_GATE, id);
        }
    }
}

impl EntityBehaviourProvider for NumericEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.create_numeric_operation(entity_instance.clone());
        self.create_numeric_gate(entity_instance);
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.remove_numeric_operation(entity_instance.clone());
        self.remove_numeric_gate(entity_instance);
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
