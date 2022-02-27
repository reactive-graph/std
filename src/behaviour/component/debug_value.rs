use std::sync::Arc;

use crate::reactive::BehaviourCreationError;
use log::{debug, trace};
use serde_json::Value;

use crate::behaviour::component::DebugValueProperties;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;

pub const DEBUG_VALUE: &'static str = "debug_value";

pub struct DebugValue {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl DebugValue {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<DebugValue, BehaviourCreationError> {
        if !e
            .properties
            .contains_key(DebugValueProperties::VALUE.as_ref())
        {
            return Err(BehaviourCreationError);
        }
        let handle_id = e
            .properties
            .get(DebugValueProperties::VALUE.as_ref())
            .unwrap()
            .id
            .as_u128();
        e.properties
            .get(DebugValueProperties::VALUE.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(move |v: &Value| debug!("{}", v), handle_id);
        debug!(
            "Enabled debugging of entity {} property {}",
            e.id, DEBUG_VALUE
        );
        Ok(DebugValue {
            entity: e.clone(),
            handle_id,
        })
    }
}

impl Disconnectable for DebugValue {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", DEBUG_VALUE, self.entity.id);
        if let Some(property) = self
            .entity
            .properties
            .get(DebugValueProperties::VALUE.as_ref())
        {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Drop for DebugValue {
    fn drop(&mut self) {
        self.disconnect();
    }
}
