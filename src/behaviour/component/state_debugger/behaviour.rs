use std::sync::Arc;

use log::debug;
use serde_json::Value;

use crate::behaviour::component::state_debugger::StateDebuggerFunction;
use crate::behaviour::component::StateProperties;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub struct StateDebugger {
    pub f: StateDebuggerFunction,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl StateDebugger {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>, f: StateDebuggerFunction) -> Result<StateDebugger, BehaviourCreationError> {
        if !e.properties.contains_key(StateProperties::STATE.as_ref()) {
            return Err(BehaviourCreationError);
        }
        let handle_id = e.properties.get(StateProperties::STATE.as_ref()).unwrap().id.as_u128();
        let entity_instance = e.clone();
        e.properties
            .get(StateProperties::STATE.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(move |v: &Value| f(v.clone(), entity_instance.clone()), handle_id);
        debug!("Starting debugging of entity {} property {}", e.id, StateProperties::STATE.as_ref());
        Ok(StateDebugger { f, entity: e, handle_id })
    }
}

impl Disconnectable for StateDebugger {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(StateProperties::STATE.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
            debug!("Stopped debugging of entity {} property {}", self.entity.id, StateProperties::STATE.as_ref());
        }
    }
}

impl Drop for StateDebugger {
    fn drop(&mut self) {
        self.disconnect();
    }
}
