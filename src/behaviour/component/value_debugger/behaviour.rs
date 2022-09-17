use std::sync::Arc;

use crate::behaviour::component::value_debugger::ValueDebuggerFunction;
use log::debug;
use serde_json::Value;

use crate::behaviour::component::ValueProperties;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub struct ValueDebugger {
    pub f: ValueDebuggerFunction,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl ValueDebugger {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>, f: ValueDebuggerFunction) -> Result<ValueDebugger, BehaviourCreationError> {
        if !e.properties.contains_key(ValueProperties::VALUE.as_ref()) {
            return Err(BehaviourCreationError);
        }
        let handle_id = e.properties.get(ValueProperties::VALUE.as_ref()).unwrap().id.as_u128();
        e.properties
            .get(ValueProperties::VALUE.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(move |v: &Value| f(v.clone()), handle_id);
        debug!("Starting debugging of entity {} property {}", e.id, ValueProperties::VALUE.as_ref());
        Ok(ValueDebugger { f, entity: e, handle_id })
    }
}

impl Disconnectable for ValueDebugger {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(ValueProperties::VALUE.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
            debug!("Stopped debugging of entity {} property {}", self.entity.id, ValueProperties::VALUE.as_ref());
        }
    }
}

impl Drop for ValueDebugger {
    fn drop(&mut self) {
        self.disconnect();
    }
}
