use std::sync::Arc;

use crate::di::*;
use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::entity::gate::LogicalGate;
use crate::behaviour::entity::gate::LOGICAL_GATES;
use crate::behaviour::entity::if_then_else::IfThenElse;
use crate::behaviour::entity::if_then_else::IF_THEN_ELSE;
use crate::behaviour::entity::operation::LogicalOperation;
use crate::behaviour::entity::operation::LOGICAL_OPERATIONS;
use crate::behaviour::entity::trigger::Trigger;
use crate::behaviour::entity::trigger::TRIGGER;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct LogicalOperationStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<LogicalOperation<'static>>>>);

#[wrapper]
pub struct LogicalGateStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<LogicalGate<'static>>>>);

#[wrapper]
pub struct TriggerStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<Trigger>>>);

#[wrapper]
pub struct IfThenElseStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<IfThenElse>>>);

#[provides]
fn create_logical_operation_storage() -> LogicalOperationStorage {
    LogicalOperationStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_logical_gate_storage() -> LogicalGateStorage {
    LogicalGateStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_trigger_storage() -> TriggerStorage {
    TriggerStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_if_then_else_storage() -> IfThenElseStorage {
    IfThenElseStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait LogicalEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_logical_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_logical_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_trigger(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_if_then_else(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_logical_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_logical_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_trigger(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_if_then_else(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

// #[derive(Clone)]
pub struct LogicalEntityBehaviourProviderImpl {
    logical_operations: LogicalOperationStorage,
    logical_gates: LogicalGateStorage,
    triggers: TriggerStorage,
    if_then_else_behaviours: IfThenElseStorage,
}

interfaces!(LogicalEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl LogicalEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            logical_operations: create_logical_operation_storage(),
            logical_gates: create_logical_gate_storage(),
            triggers: create_trigger_storage(),
            if_then_else_behaviours: create_if_then_else_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl LogicalEntityBehaviourProvider for LogicalEntityBehaviourProviderImpl {
    fn create_logical_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let type_name = entity_instance.type_name.as_str();
        let id = entity_instance.id;
        let function = LOGICAL_OPERATIONS.get(type_name);
        let logical_operation = match function {
            Some(function) => Some(Arc::new(LogicalOperation::new(entity_instance.clone(), *function))),
            None => None,
        };
        if logical_operation.is_some() {
            self.logical_operations.0.write().unwrap().insert(id, logical_operation.unwrap());
            debug!("Added behaviour {} to entity instance {}", type_name, id);
        }
    }

    fn create_logical_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let type_name = entity_instance.type_name.as_str();
        let id = entity_instance.id;
        let function = LOGICAL_GATES.get(type_name);
        let logical_gate = match function {
            Some(function) => Some(Arc::new(LogicalGate::new(entity_instance.clone(), *function))),
            None => None,
        };
        if logical_gate.is_some() {
            self.logical_gates.0.write().unwrap().insert(id, logical_gate.unwrap());
            debug!("Added behaviour {} to entity instance {}", type_name, id);
        }
    }

    fn create_trigger(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let trigger = Trigger::new(entity_instance);
        if trigger.is_ok() {
            let trigger = Arc::new(trigger.unwrap());
            self.triggers.0.write().unwrap().insert(id, trigger);
            debug!("Added behaviour {} to entity instance {}", TRIGGER, id);
        }
    }

    fn create_if_then_else(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let if_then_else = IfThenElse::new(entity_instance);
        if if_then_else.is_ok() {
            let if_then_else = Arc::new(if_then_else.unwrap());
            self.if_then_else_behaviours.0.write().unwrap().insert(id, if_then_else);
            debug!("Added behaviour {} to entity instance {}", IF_THEN_ELSE, id);
        }
    }

    fn remove_logical_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.logical_operations.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour logical_operation from entity instance {}", entity_instance.id);
    }

    fn remove_logical_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.logical_gates.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour logical_gates from entity instance {}", entity_instance.id);
    }

    fn remove_trigger(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.triggers.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour {} from entity instance {}", TRIGGER, entity_instance.id);
    }

    fn remove_if_then_else(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.if_then_else_behaviours.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour {} from entity instance {}", IF_THEN_ELSE, entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.logical_operations.0.write().unwrap().contains_key(&id) {
            self.logical_operations.0.write().unwrap().remove(&id);
            debug!("Removed behaviour logical_operation from entity instance {}", id);
        }
        if self.logical_gates.0.write().unwrap().contains_key(&id) {
            self.logical_gates.0.write().unwrap().remove(&id);
            debug!("Removed behaviour logical_gates from entity instance {}", id);
        }
        if self.triggers.0.write().unwrap().contains_key(&id) {
            self.triggers.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", TRIGGER, id);
        }
        if self.if_then_else_behaviours.0.write().unwrap().contains_key(&id) {
            self.if_then_else_behaviours.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", IF_THEN_ELSE, id);
        }
    }
}

impl EntityBehaviourProvider for LogicalEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.create_logical_operation(entity_instance.clone());
        self.create_logical_gate(entity_instance.clone());
        match entity_instance.type_name.as_str() {
            TRIGGER => self.create_trigger(entity_instance),
            IF_THEN_ELSE => self.create_if_then_else(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.remove_logical_operation(entity_instance.clone());
        self.remove_logical_gate(entity_instance.clone());
        match entity_instance.type_name.as_str() {
            TRIGGER => self.remove_trigger(entity_instance),
            IF_THEN_ELSE => self.remove_if_then_else(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
