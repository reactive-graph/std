use inexor_rgf_core_model::BehaviourTypeId;
use inexor_rgf_core_reactive::Behaviour;
use std::sync::Arc;

use log::debug;
use serde_json::Value;
use uuid::Uuid;

use crate::behaviour::component::value_debugger::ValueDebuggerFunction;
use crate::behaviour::component::ValueProperties;
use crate::model::ReactiveEntityInstance;
use crate::reactive::BehaviourCreationError;

pub struct ValueDebugger {
    pub entity: Arc<ReactiveEntityInstance>,

    pub ty: BehaviourTypeId,

    pub f: ValueDebuggerFunction,

    pub handle_id: u128,
}

impl ValueDebugger {
    pub fn new(e: Arc<ReactiveEntityInstance>, ty: BehaviourTypeId, f: ValueDebuggerFunction) -> Result<ValueDebugger, BehaviourCreationError> {
        let property_value = e.properties.get(ValueProperties::VALUE.as_ref()).ok_or(BehaviourCreationError)?;
        let handle_id = Uuid::new_v4().as_u128();
        property_value
            .stream
            .read()
            .unwrap()
            .observe_with_handle(move |v: &Value| f(v.clone()), handle_id);
        debug!("Starting debugging of entity {} property {}", e.id, ValueProperties::VALUE.as_ref());
        let entity = e.clone();
        Ok(ValueDebugger { entity, ty, f, handle_id })
    }
}

impl Behaviour for ValueDebugger {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(ValueProperties::VALUE.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
            debug!("Stopped debugging of entity {} property {}", self.entity.id, ValueProperties::VALUE.as_ref());
        }
    }

    fn ty(&self) -> BehaviourTypeId {
        self.ty.clone()
    }
}

impl Drop for ValueDebugger {
    fn drop(&mut self) {
        self.disconnect();
    }
}
