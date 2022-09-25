use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::entity::counter::Counter;
use crate::behaviour::entity::counter::COUNTER;
use crate::behaviour::entity::gate::ArithmeticGate;
use crate::behaviour::entity::gate::ARITHMETIC_GATES;
use crate::behaviour::entity::operation::ArithmeticOperation;
use crate::behaviour::entity::operation::ARITHMETIC_OPERATIONS;
use crate::di::*;
use crate::model::ReactiveBehaviourContainer;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct ArithmeticOperationStorage(RwLock<HashMap<Uuid, Arc<ArithmeticOperation<'static>>>>);

#[wrapper]
pub struct ArithmeticGateStorage(RwLock<HashMap<Uuid, Arc<ArithmeticGate<'static>>>>);

#[wrapper]
pub struct CounterStorage(RwLock<HashMap<Uuid, Arc<Counter>>>);

#[provides]
fn create_arithmetic_operation_storage() -> ArithmeticOperationStorage {
    ArithmeticOperationStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_arithmetic_gate_storage() -> ArithmeticGateStorage {
    ArithmeticGateStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_counter_storage() -> CounterStorage {
    CounterStorage(RwLock::new(HashMap::new()))
}

#[async_trait]
pub trait ArithmeticEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_counter(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_counter(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct ArithmeticEntityBehaviourProviderImpl {
    arithmetic_operations: ArithmeticOperationStorage,
    arithmetic_gates: ArithmeticGateStorage,
    counters: CounterStorage,
}

interfaces!(ArithmeticEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl ArithmeticEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            arithmetic_operations: create_arithmetic_operation_storage(),
            arithmetic_gates: create_arithmetic_gate_storage(),
            counters: create_counter_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl ArithmeticEntityBehaviourProvider for ArithmeticEntityBehaviourProviderImpl {
    fn create_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let type_name = entity_instance.type_name.as_str();
        if let Some(arithmetic_operation) = ARITHMETIC_OPERATIONS
            .get(type_name)
            .map(|function| Arc::new(ArithmeticOperation::new(entity_instance.clone(), *function)))
        {
            self.arithmetic_operations.0.write().unwrap().insert(id, arithmetic_operation);
            entity_instance.add_behaviour(type_name);
            debug!("Added behaviour {} to entity instance {}", type_name, id);
        }
    }

    fn create_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let type_name = entity_instance.type_name.as_str();
        if let Some(arithmetic_gate) = ARITHMETIC_GATES
            .get(type_name)
            .map(|function| Arc::new(ArithmeticGate::new(entity_instance.clone(), *function)))
        {
            self.arithmetic_gates.0.write().unwrap().insert(id, arithmetic_gate);
            entity_instance.add_behaviour(type_name);
            debug!("Added behaviour {} to entity instance {}", type_name, id);
        }
    }

    fn create_counter(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        if let Ok(counter) = Counter::new(entity_instance.clone()) {
            let counter = Arc::new(counter);
            self.counters.0.write().unwrap().insert(id, counter);
            entity_instance.add_behaviour(COUNTER);
            debug!("Added behaviour {} to entity instance {}", COUNTER, id);
        }
    }

    fn remove_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if self.arithmetic_operations.0.write().unwrap().remove(&entity_instance.id).is_some() {
            entity_instance.remove_behaviour(entity_instance.type_name.as_str());
            debug!("Removed behaviour arithmetic_operation from entity instance {}", entity_instance.id);
        }
    }

    fn remove_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if self.arithmetic_gates.0.write().unwrap().remove(&entity_instance.id).is_some() {
            entity_instance.remove_behaviour(entity_instance.type_name.as_str());
            debug!("Removed behaviour arithmetic_gates from entity instance {}", entity_instance.id);
        }
    }

    fn remove_counter(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if self.counters.0.write().unwrap().remove(&entity_instance.id).is_some() {
            entity_instance.remove_behaviour(COUNTER);
            debug!("Removed behaviour {} from entity instance {}", COUNTER, entity_instance.id);
        }
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.arithmetic_operations.0.write().unwrap().contains_key(&id) && self.arithmetic_operations.0.write().unwrap().remove(&id).is_some() {
            debug!("Removed behaviour arithmetic_operation from entity instance {}", id);
        }
        if self.arithmetic_gates.0.write().unwrap().contains_key(&id) && self.arithmetic_gates.0.write().unwrap().remove(&id).is_some() {
            debug!("Removed behaviour arithmetic_gates from entity instance {}", id);
        }
        if self.counters.0.write().unwrap().contains_key(&id) && self.counters.0.write().unwrap().remove(&id).is_some() {
            debug!("Removed behaviour {} from entity instance {}", COUNTER, id);
        }
    }
}

impl EntityBehaviourProvider for ArithmeticEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.create_operation(entity_instance.clone());
        self.create_gate(entity_instance.clone());
        match entity_instance.type_name.as_str() {
            COUNTER => self.create_counter(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.remove_operation(entity_instance.clone());
        self.remove_gate(entity_instance.clone());
        match entity_instance.type_name.as_str() {
            COUNTER => self.remove_counter(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
