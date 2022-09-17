use std::sync::Arc;

use serde_json::json;
use uuid::Uuid;

use crate::behaviour::component::properties::PropagationCountProperties;
use crate::behaviour::relation::properties::ConnectorProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveRelationInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const PROPAGATION_COUNTER: &str = "propagation_counter";

pub struct PropagationCounter {
    pub relation: Arc<ReactiveRelationInstance>,

    pub handle_id: u128,
}

impl PropagationCounter {
    pub fn new<'a>(r: Arc<ReactiveRelationInstance>) -> Result<PropagationCounter, BehaviourCreationError> {
        if !r.properties.contains_key(PropagationCountProperties::PROPAGATION_COUNT.as_ref()) {
            return Err(BehaviourCreationError);
        }
        if !r.properties.contains_key(ConnectorProperties::OUTBOUND_PROPERTY_NAME.as_ref()) {
            return Err(BehaviourCreationError);
        }
        let outbound_property_name = match r.as_string(ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string()) {
            None => {
                return Err(BehaviourCreationError);
            }
            Some(outbound_property_name) => outbound_property_name,
        };
        let handle_id = Uuid::new_v4().as_u128();
        let relation_instance = r.clone();
        r.outbound
            .properties
            .get(outbound_property_name.as_str())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |_| {
                    if let Some(mut count) = relation_instance
                        .get(PropagationCountProperties::PROPAGATION_COUNT.as_ref())
                        .and_then(|v| v.as_u64())
                    {
                        count += 1;
                        relation_instance.set(PropagationCountProperties::PROPAGATION_COUNT.as_ref(), json!(count));
                    }
                },
                handle_id,
            );
        Ok(PropagationCounter { relation: r, handle_id })
    }
}

impl Disconnectable for PropagationCounter {
    fn disconnect(&self) {
        if let Some(outbound_property_name) = self.relation.as_string(ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string()) {
            if let Some(outbound_property) = self.relation.outbound.properties.get(&outbound_property_name) {
                outbound_property.stream.read().unwrap().remove(self.handle_id);
            }
        }
    }
}

impl Drop for PropagationCounter {
    fn drop(&mut self) {
        self.disconnect();
    }
}
