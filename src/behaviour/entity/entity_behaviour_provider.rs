use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::entity::comparison::StringComparison;
use crate::behaviour::entity::comparison::STRING_COMPARISONS;
use crate::behaviour::entity::gate::StringGate;
use crate::behaviour::entity::gate::STRING_GATES;
use crate::behaviour::entity::operation::StringOperation;
use crate::behaviour::entity::operation::STRING_OPERATIONS;
use crate::behaviour::entity::str_bool_operation::StrBoolOperation;
use crate::behaviour::entity::str_bool_operation::STR_BOOL_FUNCTIONS;
use crate::behaviour::entity::str_num_operation::StrNumOperation;
use crate::behaviour::entity::str_num_operation::STR_NUM_FUNCTIONS;
use crate::behaviour::entity::str_str_num_gate::StrStrNumGate;
use crate::behaviour::entity::str_str_num_gate::STR_STR_NUM_FUNCTIONS;
use crate::behaviour::entity::templating::behaviour::Templating;
use crate::behaviour::entity::templating::behaviour::TEMPLATING;
use crate::di::*;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct StringOperationStorage(RwLock<HashMap<Uuid, Arc<StringOperation<'static>>>>);

#[wrapper]
pub struct StringGateStorage(RwLock<HashMap<Uuid, Arc<StringGate<'static>>>>);

#[wrapper]
pub struct StringComparisonStorage(RwLock<HashMap<Uuid, Arc<StringComparison<'static>>>>);

#[wrapper]
pub struct StrNumOperationStorage(RwLock<HashMap<Uuid, Arc<StrNumOperation<'static>>>>);

#[wrapper]
pub struct StrStrNumGateStorage(RwLock<HashMap<Uuid, Arc<StrStrNumGate<'static>>>>);

#[wrapper]
pub struct StrBoolOperationStorage(RwLock<HashMap<Uuid, Arc<StrBoolOperation<'static>>>>);

#[wrapper]
pub struct TemplatingStorage(RwLock<HashMap<Uuid, Arc<Templating<'static>>>>);

#[provides]
fn create_string_operation_storage() -> StringOperationStorage {
    StringOperationStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_string_gate_storage() -> StringGateStorage {
    StringGateStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_string_comparison_storage() -> StringComparisonStorage {
    StringComparisonStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_str_num_operation_storage() -> StrNumOperationStorage {
    StrNumOperationStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_str_str_num_gate_storage() -> StrStrNumGateStorage {
    StrStrNumGateStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_str_bool_operation_storage() -> StrBoolOperationStorage {
    StrBoolOperationStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_templating_storage() -> TemplatingStorage {
    TemplatingStorage(RwLock::new(HashMap::new()))
}

#[async_trait]
pub trait StringEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_string_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_string_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_string_comparison(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_str_num_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_str_str_num_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_str_bool_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_templating(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_string_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_string_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_string_comparison(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_str_num_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_str_str_num_gate(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_str_bool_operation(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_templating(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct StringEntityBehaviourProviderImpl {
    string_operations: StringOperationStorage,
    string_gates: StringGateStorage,
    string_comparisons: StringComparisonStorage,
    str_num_operations: StrNumOperationStorage,
    str_str_num_gates: StrStrNumGateStorage,
    str_bool_operations: StrBoolOperationStorage,
    templating_gates: TemplatingStorage,
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
            str_num_operations: create_str_num_operation_storage(),
            str_str_num_gates: create_str_str_num_gate_storage(),
            str_bool_operations: create_str_bool_operation_storage(),
            templating_gates: create_templating_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl StringEntityBehaviourProvider for StringEntityBehaviourProviderImpl {
    fn create_string_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let type_name = entity_instance.type_name.as_str();
        let id = entity_instance.id;
        if let Some(behaviour) = STRING_OPERATIONS
            .get(type_name)
            .map(|function| Arc::new(StringOperation::new(entity_instance.clone(), *function)))
        {
            self.string_operations.0.write().unwrap().insert(id, behaviour);
            entity_instance.add_behaviour(type_name);
            debug!("Added behaviour {} to entity instance {}", type_name, id);
        }
    }

    fn create_string_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let type_name = entity_instance.type_name.as_str();
        let id = entity_instance.id;
        if let Some(behaviour) = STRING_GATES
            .get(type_name)
            .map(|function| Arc::new(StringGate::new(entity_instance.clone(), *function)))
        {
            self.string_gates.0.write().unwrap().insert(id, behaviour);
            entity_instance.add_behaviour(type_name);
            debug!("Added behaviour {} to entity instance {}", type_name, id);
        }
    }

    fn create_string_comparison(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let type_name = entity_instance.type_name.as_str();
        let id = entity_instance.id;
        if let Some(behaviour) = STRING_COMPARISONS
            .get(type_name)
            .map(|function| Arc::new(StringComparison::new(entity_instance.clone(), *function)))
        {
            self.string_comparisons.0.write().unwrap().insert(id, behaviour);
            entity_instance.add_behaviour(type_name);
            debug!("Added behaviour {} to entity instance {}", type_name, id);
        }
    }

    fn create_str_num_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let type_name = entity_instance.type_name.as_str();
        let id = entity_instance.id;
        if let Some(behaviour) = STR_NUM_FUNCTIONS
            .get(type_name)
            .map(|function| Arc::new(StrNumOperation::new(entity_instance.clone(), *function)))
        {
            self.str_num_operations.0.write().unwrap().insert(id, behaviour);
            entity_instance.add_behaviour(type_name);
            debug!("Added behaviour {} to entity instance {}", type_name, id);
        }
    }

    fn create_str_str_num_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let type_name = entity_instance.type_name.as_str();
        let id = entity_instance.id;
        if let Some(behaviour) = STR_STR_NUM_FUNCTIONS
            .get(type_name)
            .map(|function| Arc::new(StrStrNumGate::new(entity_instance.clone(), *function)))
        {
            self.str_str_num_gates.0.write().unwrap().insert(id, behaviour);
            entity_instance.add_behaviour(type_name);
            debug!("Added behaviour {} to entity instance {}", type_name, id);
        }
    }

    fn create_str_bool_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let type_name = entity_instance.type_name.as_str();
        let id = entity_instance.id;
        if let Some(behaviour) = STR_BOOL_FUNCTIONS
            .get(type_name)
            .map(|function| Arc::new(StrBoolOperation::new(entity_instance.clone(), *function)))
        {
            self.str_bool_operations.0.write().unwrap().insert(id, behaviour);
            entity_instance.add_behaviour(type_name);
            debug!("Added behaviour {} to entity instance {}", type_name, id);
        }
    }

    fn create_templating(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let type_name = entity_instance.type_name.as_str();
        let id = entity_instance.id;
        let behaviour = Arc::new(Templating::new(entity_instance.clone()));
        self.templating_gates.0.write().unwrap().insert(id, behaviour);
        entity_instance.add_behaviour(type_name);
        debug!("Added behaviour {} to entity instance {}", type_name, id);
    }

    fn remove_string_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if self.string_operations.0.write().unwrap().remove(&entity_instance.id).is_some() {
            let type_name = entity_instance.type_name.as_str();
            entity_instance.remove_behaviour(type_name);
            debug!("Removed behaviour {} from entity instance {}", type_name, entity_instance.id);
        }
    }

    fn remove_string_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if self.string_gates.0.write().unwrap().remove(&entity_instance.id).is_some() {
            let type_name = entity_instance.type_name.as_str();
            entity_instance.remove_behaviour(type_name);
            debug!("Removed behaviour {} from entity instance {}", type_name, entity_instance.id);
        }
    }

    fn remove_string_comparison(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if self.string_comparisons.0.write().unwrap().remove(&entity_instance.id).is_some() {
            let type_name = entity_instance.type_name.as_str();
            entity_instance.remove_behaviour(type_name);
            debug!("Removed behaviour {} from entity instance {}", type_name, entity_instance.id);
        }
    }

    fn remove_str_num_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if self.str_num_operations.0.write().unwrap().remove(&entity_instance.id).is_some() {
            let type_name = entity_instance.type_name.as_str();
            entity_instance.remove_behaviour(type_name);
            debug!("Removed behaviour {} from entity instance {}", type_name, entity_instance.id);
        }
    }

    fn remove_str_str_num_gate(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if self.str_str_num_gates.0.write().unwrap().remove(&entity_instance.id).is_some() {
            let type_name = entity_instance.type_name.as_str();
            entity_instance.remove_behaviour(type_name);
            debug!("Removed behaviour {} from entity instance {}", type_name, entity_instance.id);
        }
    }

    fn remove_str_bool_operation(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if self.str_bool_operations.0.write().unwrap().remove(&entity_instance.id).is_some() {
            let type_name = entity_instance.type_name.as_str();
            entity_instance.remove_behaviour(type_name);
            debug!("Removed behaviour {} from entity instance {}", type_name, entity_instance.id);
        }
    }

    fn remove_templating(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if self.templating_gates.0.write().unwrap().remove(&entity_instance.id).is_some() {
            let type_name = entity_instance.type_name.as_str();
            entity_instance.remove_behaviour(type_name);
            debug!("Removed behaviour {} from entity instance {}", type_name, entity_instance.id);
        }
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.string_operations.0.write().unwrap().contains_key(&id) && self.string_operations.0.write().unwrap().remove(&id).is_some() {
            debug!("Removed behaviour string_operation from entity instance {}", id);
        }
        if self.string_gates.0.write().unwrap().contains_key(&id) && self.string_gates.0.write().unwrap().remove(&id).is_some() {
            debug!("Removed behaviour string_gate from entity instance {}", id);
        }
        if self.string_comparisons.0.write().unwrap().contains_key(&id) && self.string_comparisons.0.write().unwrap().remove(&id).is_some() {
            debug!("Removed behaviour string_comparisons from entity instance {}", id);
        }
        if self.str_num_operations.0.write().unwrap().contains_key(&id) && self.str_num_operations.0.write().unwrap().remove(&id).is_some() {
            debug!("Removed behaviour str_num_operation from entity instance {}", id);
        }
        if self.str_str_num_gates.0.write().unwrap().contains_key(&id) && self.str_str_num_gates.0.write().unwrap().remove(&id).is_some() {
            debug!("Removed behaviour str_str_num_gate from entity instance {}", id);
        }
        if self.str_bool_operations.0.write().unwrap().contains_key(&id) && self.str_bool_operations.0.write().unwrap().remove(&id).is_some() {
            debug!("Removed behaviour str_bool_operation from entity instance {}", id);
        }
        if self.templating_gates.0.write().unwrap().contains_key(&id) && self.templating_gates.0.write().unwrap().remove(&id).is_some() {
            debug!("Removed behaviour templating from entity instance {}", id);
        }
    }
}

impl EntityBehaviourProvider for StringEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.create_string_operation(entity_instance.clone());
        self.create_string_gate(entity_instance.clone());
        self.create_string_comparison(entity_instance.clone());
        self.create_str_num_operation(entity_instance.clone());
        self.create_str_str_num_gate(entity_instance.clone());
        self.create_str_bool_operation(entity_instance.clone());
        match entity_instance.type_name.as_str() {
            TEMPLATING => self.create_templating(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.remove_string_operation(entity_instance.clone());
        self.remove_string_gate(entity_instance.clone());
        self.remove_string_comparison(entity_instance.clone());
        self.remove_str_num_operation(entity_instance.clone());
        self.remove_str_str_num_gate(entity_instance.clone());
        self.remove_str_bool_operation(entity_instance.clone());
        match entity_instance.type_name.as_str() {
            TEMPLATING => self.remove_templating(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
