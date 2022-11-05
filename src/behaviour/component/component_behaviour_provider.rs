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
use crate::model::ComponentTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::ReactiveBehaviourContainer;
use crate::model::ReactiveEntityInstance;
use crate::plugins::ComponentBehaviourProvider;
use crate::reactive::BehaviourType;

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
    fn create_state(&self, entity_instance: Arc<ReactiveEntityInstance>, component_ty: &ComponentTypeId);

    fn create_value_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, component_ty: &ComponentTypeId);

    fn create_state_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, component_ty: &ComponentTypeId);

    fn remove_state(&self, entity_instance: Arc<ReactiveEntityInstance>, component_ty: &ComponentTypeId);

    fn remove_value_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, component_ty: &ComponentTypeId);

    fn remove_state_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, component_ty: &ComponentTypeId);

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
    fn create_state(&self, entity_instance: Arc<ReactiveEntityInstance>, component_ty: &ComponentTypeId) {
        let id = entity_instance.id;
        if STATE_COMPONENTS.contains(&component_ty) {
            if let Ok(behaviour) = State::new(entity_instance.clone(), component_ty.clone()) {
                let type_name = behaviour.type_name();
                self.states.0.write().unwrap().insert(id, Arc::new(behaviour));
                entity_instance.add_behaviour(type_name.clone());
                debug!("Added behaviour {} to entity instance {}", type_name, id);
            }
        }
    }

    fn create_value_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, component_ty: &ComponentTypeId) {
        let id = entity_instance.id;
        if let Some(behaviour) = VALUE_DEBUGGERS
            .get(component_ty)
            .and_then(|function| ValueDebugger::new(entity_instance.clone(), component_ty.clone(), *function).ok())
        {
            let type_name = behaviour.type_name();
            self.value_debuggers.0.write().unwrap().insert(id, Arc::new(behaviour));
            entity_instance.add_behaviour(type_name.clone());
            debug!("Added behaviour {} to entity instance {}", type_name, id);
        }
    }

    fn create_state_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, component_ty: &ComponentTypeId) {
        let id = entity_instance.id;
        if let Some(behaviour) = STATE_DEBUGGERS
            .get(component_ty)
            .and_then(|function| StateDebugger::new(entity_instance.clone(), component_ty.clone(), *function).ok())
        {
            let type_name = behaviour.type_name();
            self.state_debuggers.0.write().unwrap().insert(id, Arc::new(behaviour));
            entity_instance.add_behaviour(type_name.clone());
            debug!("Added behaviour {} to entity instance {}", type_name, id);
        }
    }

    fn remove_state(&self, entity_instance: Arc<ReactiveEntityInstance>, component_ty: &ComponentTypeId) {
        if let Some(behaviour) = self.states.0.write().unwrap().remove(&entity_instance.id) {
            entity_instance.remove_behaviour(behaviour.type_name());
            debug!("Removed behaviour {} from entity instance {}", behaviour.type_name(), entity_instance.id);
        }
    }

    fn remove_value_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, component_ty: &ComponentTypeId) {
        if let Some(behaviour) = self.value_debuggers.0.write().unwrap().remove(&entity_instance.id) {
            entity_instance.remove_behaviour(behaviour.type_name());
            debug!("Removed behaviour {} from entity instance {}", behaviour.type_name(), entity_instance.id);
        }
    }

    fn remove_state_debugger(&self, entity_instance: Arc<ReactiveEntityInstance>, component_ty: &ComponentTypeId) {
        if let Some(behaviour) = self.state_debuggers.0.write().unwrap().remove(&entity_instance.id) {
            entity_instance.remove_behaviour(behaviour.type_name());
            debug!("Removed behaviour {} from entity instance {}", behaviour.type_name(), entity_instance.id);
        }
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
        for type_name in STATE_COMPONENTS.iter().map(|k| k.to_string()) {
            let component_ty = ComponentTypeId::new_from_type("state", &type_name);
            if entity_instance.is_a(&component_ty) {
                self.create_state(entity_instance.clone(), &component_ty);
            }
        }
        for type_name in VALUE_DEBUGGERS.keys().map(|k| k.to_string()) {
            let component_ty = ComponentTypeId::new_from_type("value", &type_name);
            if entity_instance.is_a(&component_ty) {
                self.create_value_debugger(entity_instance.clone(), &component_ty);
            }
        }
        for type_name in STATE_DEBUGGERS.keys().map(|k| k.to_string()) {
            let component_ty = ComponentTypeId::new_from_type("state", &type_name);
            if entity_instance.is_a(&component_ty) {
                self.create_state_debugger(entity_instance.clone(), &component_ty);
            }
        }
    }

    fn add_behaviours_to_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        if STATE_COMPONENTS.contains(&component.ty) {
            self.create_state(entity_instance.clone(), &component.ty);
        }
        if VALUE_DEBUGGERS.contains_key(&component.ty) {
            self.create_value_debugger(entity_instance.clone(), &component.ty);
        }
        if STATE_DEBUGGERS.contains_key(&component.ty) {
            self.create_state_debugger(entity_instance, &component.ty);
        }
    }

    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        for component_ty in STATE_COMPONENTS.iter() {
            if entity_instance.behaves_as(&component_ty.type_name()) {
                self.remove_state(entity_instance.clone(), component_ty);
            }
        }
        for component_ty in VALUE_DEBUGGERS.keys() {
            if entity_instance.behaves_as(&component_ty.type_name()) {
                self.remove_value_debugger(entity_instance.clone(), component_ty);
            }
        }
        for component_ty in STATE_DEBUGGERS.keys() {
            if entity_instance.behaves_as(&component_ty.type_name()) {
                self.remove_state_debugger(entity_instance.clone(), component_ty);
            }
        }
    }

    fn remove_behaviours_from_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        if STATE_COMPONENTS.contains(&component.ty) {
            self.remove_state(entity_instance.clone(), &component.ty);
        }
        if VALUE_DEBUGGERS.contains_key(&component.ty) {
            self.remove_value_debugger(entity_instance.clone(), &component.ty);
        }
        if STATE_DEBUGGERS.contains_key(&component.ty) {
            self.remove_state_debugger(entity_instance, &component.ty);
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
