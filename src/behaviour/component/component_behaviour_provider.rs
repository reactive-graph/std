use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::component::State;
use crate::behaviour::component::StateDebugger;
use crate::behaviour::component::ValueDebugger;
use crate::behaviour::component::STATE_COMPONENTS;
use crate::behaviour::component::STATE_DEBUGGERS;
use crate::behaviour::component::VALUE_DEBUGGERS;
use crate::di::*;
use crate::model::ComponentContainer;
use crate::model::ReactiveBehaviourContainer;
use crate::model::ReactiveEntityInstance;
use crate::plugins::ComponentBehaviourProvider;

#[wrapper]
pub struct StateStorage(RwLock<HashMap<Uuid, Arc<State>>>);

#[wrapper]
pub struct ValueDebuggerStorage(RwLock<HashMap<Uuid, Arc<ValueDebugger>>>);

#[wrapper]
pub struct StateDebuggerStorage(RwLock<HashMap<Uuid, Arc<StateDebugger>>>);

#[provides]
fn create_state_storage() -> StateStorage {
    StateStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_value_debugger_storage() -> ValueDebuggerStorage {
    ValueDebuggerStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_state_debugger_storage() -> StateDebuggerStorage {
    StateDebuggerStorage(RwLock::new(HashMap::new()))
}

#[async_trait]
pub trait ValueComponentBehaviourProvider: ComponentBehaviourProvider + Send + Sync {
    fn create_state(&self, entity_instance: Arc<ReactiveEntityInstance>, component_name: &str);

    fn create_value_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, component_name: &str);

    fn create_state_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, component_name: &str);

    fn remove_state(&self, entity_instance: Arc<ReactiveEntityInstance>, component_name: &str);

    fn remove_value_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, component_name: &str);

    fn remove_state_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, component_name: &str);

    fn remove_by_id(&self, id: Uuid);
}

pub struct ValueComponentBehaviourProviderImpl {
    states: StateStorage,
    value_debuggers: ValueDebuggerStorage,
    state_debuggers: StateDebuggerStorage,
}

interfaces!(ValueComponentBehaviourProviderImpl: dyn ComponentBehaviourProvider);

#[component]
impl ValueComponentBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            states: create_state_storage(),
            value_debuggers: create_value_debugger_storage(),
            state_debuggers: create_state_debugger_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl ValueComponentBehaviourProvider for ValueComponentBehaviourProviderImpl {
    fn create_state(&self, entity_instance: Arc<ReactiveEntityInstance>, component_name: &str) {
        let id = entity_instance.id;
        if STATE_COMPONENTS.contains(&component_name) {
            if let Ok(behaviour) = State::new(entity_instance.clone()) {
                self.states.0.write().unwrap().insert(id, Arc::new(behaviour));
                entity_instance.add_behaviour(component_name);
                debug!("Added behaviour {} to entity instance {}", component_name, id);
            }
        }
    }

    fn create_value_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, component_name: &str) {
        let id = entity_instance.id;
        if let Some(behaviour) = VALUE_DEBUGGERS
            .get(component_name)
            .and_then(|function| ValueDebugger::new(entity_instance.clone(), *function).ok())
        {
            self.value_debuggers.0.write().unwrap().insert(id, Arc::new(behaviour));
            entity_instance.add_behaviour(component_name);
            debug!("Added behaviour {} to entity instance {}", component_name, id);
        }
    }

    fn create_state_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, component_name: &str) {
        let id = entity_instance.id;
        if let Some(behaviour) = STATE_DEBUGGERS
            .get(component_name)
            .and_then(|function| StateDebugger::new(entity_instance.clone(), *function).ok())
        {
            self.state_debuggers.0.write().unwrap().insert(id, Arc::new(behaviour));
            entity_instance.add_behaviour(component_name);
            debug!("Added behaviour {} to entity instance {}", component_name, id);
        }
    }

    fn remove_state(&self, entity_instance: Arc<ReactiveEntityInstance>, component_name: &str) {
        self.states.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(component_name);
        debug!("Removed behaviour {} from entity instance {}", component_name, entity_instance.id);
    }

    fn remove_value_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, component_name: &str) {
        self.value_debuggers.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(component_name);
        debug!("Removed behaviour {} from entity instance {}", component_name, entity_instance.id);
    }

    fn remove_state_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, component_name: &str) {
        self.state_debuggers.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(component_name);
        debug!("Removed behaviour {} from entity instance {}", component_name, entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.states.0.write().unwrap().contains_key(&id) {
            self.states.0.write().unwrap().remove(&id);
            debug!("Removed a state behaviour from entity instance {}", id);
        }
        if self.value_debuggers.0.write().unwrap().contains_key(&id) {
            self.value_debuggers.0.write().unwrap().remove(&id);
            debug!("Removed a value_debugger behaviour from entity instance {}", id);
        }
        if self.state_debuggers.0.write().unwrap().contains_key(&id) {
            self.state_debuggers.0.write().unwrap().remove(&id);
            debug!("Removed a state_debugger behaviour from entity instance {}", id);
        }
    }
}

impl ComponentBehaviourProvider for ValueComponentBehaviourProviderImpl {
    fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        for component_name in STATE_COMPONENTS.iter().map(|k| k.to_string()) {
            if entity_instance.is_a(&component_name) {
                self.create_state(entity_instance.clone(), &component_name);
            }
        }
        for component_name in VALUE_DEBUGGERS.keys().map(|k| k.to_string()) {
            if entity_instance.is_a(&component_name) {
                self.create_value_debugger(entity_instance.clone(), &component_name);
            }
        }
        for component_name in STATE_DEBUGGERS.keys().map(|k| k.to_string()) {
            if entity_instance.is_a(&component_name) {
                self.create_state_debugger(entity_instance.clone(), &component_name);
            }
        }
    }

    fn add_behaviours_to_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        if STATE_COMPONENTS.contains(&component.name.as_str()) {
            self.create_state(entity_instance.clone(), &component.name);
        }
        if VALUE_DEBUGGERS.contains_key(component.name.as_str()) {
            self.create_value_debugger(entity_instance.clone(), &component.name);
        }
        if STATE_DEBUGGERS.contains_key(component.name.as_str()) {
            self.create_state_debugger(entity_instance, &component.name);
        }
    }

    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        for component_name in STATE_COMPONENTS.iter().map(|k| k.to_string()) {
            if entity_instance.behaves_as(&component_name) {
                self.remove_state(entity_instance.clone(), &component_name);
            }
        }
        for component_name in VALUE_DEBUGGERS.keys().map(|k| k.to_string()) {
            if entity_instance.behaves_as(&component_name) {
                self.remove_value_debugger(entity_instance.clone(), &component_name);
            }
        }
        for component_name in STATE_DEBUGGERS.keys().map(|k| k.to_string()) {
            if entity_instance.behaves_as(&component_name) {
                self.remove_state_debugger(entity_instance.clone(), &component_name);
            }
        }
    }

    fn remove_behaviours_from_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        if STATE_COMPONENTS.contains(&component.name.as_str()) {
            self.remove_state(entity_instance.clone(), &component.name);
        }
        if VALUE_DEBUGGERS.contains_key(component.name.as_str()) {
            self.remove_value_debugger(entity_instance.clone(), &component.name);
        }
        if STATE_DEBUGGERS.contains_key(component.name.as_str()) {
            self.remove_state_debugger(entity_instance, &component.name);
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
