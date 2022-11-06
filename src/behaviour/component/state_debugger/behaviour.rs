use inexor_rgf_core_model::BehaviourTypeId;
use inexor_rgf_core_reactive::Behaviour;
use std::sync::Arc;

use log::debug;
use serde_json::Value;
use uuid::Uuid;

use crate::behaviour::component::state_debugger::StateDebuggerFunction;
use crate::behaviour::component::StateProperties;
use crate::model::ReactiveEntityInstance;
use crate::reactive::BehaviourCreationError;

pub struct StateDebugger {
    pub entity: Arc<ReactiveEntityInstance>,

    pub ty: BehaviourTypeId,

    pub f: StateDebuggerFunction,

    pub handle_id: u128,
}

impl StateDebugger {
    pub fn new(e: Arc<ReactiveEntityInstance>, ty: BehaviourTypeId, f: StateDebuggerFunction) -> Result<StateDebugger, BehaviourCreationError> {
        let property_value = e.properties.get(StateProperties::STATE.as_ref()).ok_or(BehaviourCreationError)?;
        let handle_id = Uuid::new_v4().as_u128();
        let entity = e.clone();
        property_value
            .stream
            .read()
            .unwrap()
            .observe_with_handle(move |v: &Value| f(v.clone(), entity.clone()), handle_id);
        debug!("Starting debugging of entity {} property {}", e.id, StateProperties::STATE.as_ref());
        let entity = e.clone();
        Ok(StateDebugger { entity, ty, f, handle_id })
    }
}

impl Behaviour for StateDebugger {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(StateProperties::STATE.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
            debug!("Stopped debugging of entity {} property {}", self.entity.id, StateProperties::STATE.as_ref());
        }
    }

    fn ty(&self) -> BehaviourTypeId {
        self.ty.clone()
    }
}

impl Drop for StateDebugger {
    fn drop(&mut self) {
        self.disconnect();
    }
}
