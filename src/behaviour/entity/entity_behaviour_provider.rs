use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::entity::gate::LogicalGate;
use crate::behaviour::entity::gate::LOGICAL_GATES;
use crate::behaviour::entity::if_then_else::IfThenElse;
use crate::behaviour::entity::if_then_else::IF_THEN_ELSE;
use crate::behaviour::entity::operation::LogicalOperation;
use crate::behaviour::entity::operation::LOGICAL_OPERATIONS;
use crate::behaviour::entity::toggle::Toggle;
use crate::behaviour::entity::toggle::TOGGLE;
use crate::behaviour::entity::trigger::Trigger;
use crate::behaviour::entity::trigger::TRIGGER;
use crate::di::*;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct LogicalOperationStorage(RwLock<HashMap<Uuid, Arc<LogicalOperation<'static>>>>);

#[wrapper]
pub struct LogicalGateStorage(RwLock<HashMap<Uuid, Arc<LogicalGate<'static>>>>);

#[wrapper]
pub struct ToggleStorage(RwLock<HashMap<Uuid, Arc<Toggle>>>);

#[wrapper]
pub struct TriggerStorage(RwLock<HashMap<Uuid, Arc<Trigger>>>);

#[wrapper]
pub struct IfThenElseStorage(RwLock<HashMap<Uuid, Arc<IfThenElse>>>);

#[provides]
fn create_logical_operation_storage() -> LogicalOperationStorage {
    LogicalOperationStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_logical_gate_storage() -> LogicalGateStorage {
    LogicalGateStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_toggle_storage() -> ToggleStorage {
    ToggleStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_trigger_storage() -> TriggerStorage {
    TriggerStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_if_then_else_storage() -> IfThenElseStorage {
    IfThenElseStorage(RwLock::new(HashMap::new()))
}

#[async_trait]
pub trait LogicalEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_logical_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_logical_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_toggle(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_trigger(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_if_then_else(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_logical_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_logical_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_toggle(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_trigger(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_if_then_else(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct LogicalEntityBehaviourProviderImpl {
    logical_operations: LogicalOperationStorage,
    logical_gates: LogicalGateStorage,
    toggles: ToggleStorage,
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
            toggles: create_toggle_storage(),
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
        if let Some(logical_operation) = LOGICAL_OPERATIONS
            .get(type_name)
            .map(|function| Arc::new(LogicalOperation::new(entity_instance.clone(), *function)))
        {
            self.logical_operations.0.write().unwrap().insert(id, logical_operation);
            entity_instance.add_behaviour(type_name);
            debug!("Added behaviour {} to entity instance {}", type_name, id);
        }
    }

    fn create_logical_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let type_name = entity_instance.type_name.as_str();
        let id = entity_instance.id;
        if let Some(logical_gate) = LOGICAL_GATES
            .get(type_name)
            .map(|function| Arc::new(LogicalGate::new(entity_instance.clone(), *function)))
        {
            self.logical_gates.0.write().unwrap().insert(id, logical_gate);
            entity_instance.add_behaviour(type_name);
            debug!("Added behaviour {} to entity instance {}", type_name, id);
        }
    }

    fn create_toggle(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        if let Ok(toggle) = Toggle::new(entity_instance.clone()) {
            let toggle = Arc::new(toggle);
            self.toggles.0.write().unwrap().insert(id, toggle);
            entity_instance.add_behaviour(TOGGLE);
            debug!("Added behaviour {} to entity instance {}", TOGGLE, id);
        }
    }

    fn create_trigger(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        if let Ok(trigger) = Trigger::new(entity_instance.clone()) {
            let trigger = Arc::new(trigger);
            self.triggers.0.write().unwrap().insert(id, trigger);
            entity_instance.add_behaviour(TRIGGER);
            debug!("Added behaviour {} to entity instance {}", TRIGGER, id);
        }
    }

    fn create_if_then_else(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        if let Ok(if_then_else) = IfThenElse::new(entity_instance.clone()) {
            let if_then_else = Arc::new(if_then_else);
            self.if_then_else_behaviours.0.write().unwrap().insert(id, if_then_else);
            entity_instance.add_behaviour(IF_THEN_ELSE);
            debug!("Added behaviour {} to entity instance {}", IF_THEN_ELSE, id);
        }
    }

    fn remove_logical_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if let Some(_) = self.logical_operations.0.write().unwrap().remove(&entity_instance.id) {
            entity_instance.remove_behaviour(entity_instance.type_name.as_str());
            debug!("Removed behaviour logical_operation from entity instance {}", entity_instance.id);
        }
    }

    fn remove_logical_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if let Some(_) = self.logical_gates.0.write().unwrap().remove(&entity_instance.id) {
            entity_instance.remove_behaviour(entity_instance.type_name.as_str());
            debug!("Removed behaviour logical_gates from entity instance {}", entity_instance.id);
        }
    }

    fn remove_toggle(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if let Some(_) = self.toggles.0.write().unwrap().remove(&entity_instance.id) {
            entity_instance.remove_behaviour(TOGGLE);
            debug!("Removed behaviour {} from entity instance {}", TOGGLE, entity_instance.id);
        }
    }

    fn remove_trigger(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if let Some(_) = self.triggers.0.write().unwrap().remove(&entity_instance.id) {
            entity_instance.remove_behaviour(TRIGGER);
            debug!("Removed behaviour {} from entity instance {}", TRIGGER, entity_instance.id);
        }
    }

    fn remove_if_then_else(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if let Some(_) = self.if_then_else_behaviours.0.write().unwrap().remove(&entity_instance.id) {
            entity_instance.remove_behaviour(IF_THEN_ELSE);
            debug!("Removed behaviour {} from entity instance {}", IF_THEN_ELSE, entity_instance.id);
        }
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.logical_operations.0.write().unwrap().contains_key(&id) {
            if let Some(_) = self.logical_operations.0.write().unwrap().remove(&id) {
                debug!("Removed behaviour logical_operation from entity instance {}", id);
            }
        }
        if self.logical_gates.0.write().unwrap().contains_key(&id) {
            if let Some(_) = self.logical_gates.0.write().unwrap().remove(&id) {
                debug!("Removed behaviour logical_gates from entity instance {}", id);
            }
        }
        if self.toggles.0.write().unwrap().contains_key(&id) {
            if let Some(_) = self.toggles.0.write().unwrap().remove(&id) {
                debug!("Removed behaviour {} from entity instance {}", TOGGLE, id);
            }
        }
        if self.triggers.0.write().unwrap().contains_key(&id) {
            if let Some(_) = self.triggers.0.write().unwrap().remove(&id) {
                debug!("Removed behaviour {} from entity instance {}", TRIGGER, id);
            }
        }
        if self.if_then_else_behaviours.0.write().unwrap().contains_key(&id) {
            if let Some(_behaviour) = self.if_then_else_behaviours.0.write().unwrap().remove(&id) {
                // TODO: _behaviour.entity.remove_behaviour(IF_THEN_ELSE);
                debug!("Removed behaviour {} from entity instance {}", IF_THEN_ELSE, id);
            }
        }
    }
}

impl EntityBehaviourProvider for LogicalEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.create_logical_operation(entity_instance.clone());
        self.create_logical_gate(entity_instance.clone());
        match entity_instance.type_name.as_str() {
            TOGGLE => self.create_toggle(entity_instance),
            TRIGGER => self.create_trigger(entity_instance),
            IF_THEN_ELSE => self.create_if_then_else(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.remove_logical_operation(entity_instance.clone());
        self.remove_logical_gate(entity_instance.clone());
        match entity_instance.type_name.as_str() {
            TOGGLE => self.remove_toggle(entity_instance),
            TRIGGER => self.remove_trigger(entity_instance),
            IF_THEN_ELSE => self.remove_if_then_else(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
