use std::convert::AsRef;
use std::sync::Arc;

use inexor_rgf_core_model::PropertyInstanceSetter;
use log::error;
use serde_json::Value;

use crate::behaviour::entity::if_then_else_properties::IfThenElseProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const IF_THEN_ELSE: &str = "if_then_else";

pub struct IfThenElse {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl IfThenElse {
    pub fn new(e: Arc<ReactiveEntityInstance>) -> Result<IfThenElse, BehaviourCreationError> {
        let condition = e.properties.get(IfThenElseProperties::CONDITION.as_ref());
        if condition.is_none() {
            error!("Missing property condition");
            return Err(BehaviourCreationError);
        }
        let result = e.properties.get(IfThenElseProperties::RESULT.as_ref());
        if result.is_none() {
            error!("Missing property result");
            return Err(BehaviourCreationError);
        }

        let entity_instance = e.clone();
        let handle_id = e.properties.get(IfThenElseProperties::CONDITION.as_ref()).unwrap().id.as_u128();
        e.properties
            .get(IfThenElseProperties::CONDITION.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |v: &Value| {
                    if !v.is_boolean() {
                        // Invalid type
                        return;
                    }
                    let payload_property = if v.as_bool().unwrap() {
                        IfThenElseProperties::THEN_PAYLOAD
                    } else {
                        IfThenElseProperties::ELSE_PAYLOAD
                    };
                    if let Some(payload) = entity_instance.get(payload_property) {
                        entity_instance.set(IfThenElseProperties::RESULT, payload);
                    }
                },
                handle_id,
            );

        Ok(IfThenElse { entity: e.clone(), handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for IfThenElse {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(IfThenElseProperties::CONDITION.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for IfThenElse {
    fn drop(&mut self) {
        self.disconnect();
    }
}
