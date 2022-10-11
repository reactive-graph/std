use std::sync::Arc;

use serde_json::Value;
use uuid::Uuid;

use crate::behaviour::component::StateProperties;
use crate::behaviour::component::ValueProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub struct State {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id_set_state: u128,

    pub handle_id_value: u128,
}

impl State {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<State, BehaviourCreationError> {
        if !e.properties.contains_key(StateProperties::STATE.as_ref()) {
            return Err(BehaviourCreationError);
        }
        if !e.properties.contains_key(StateProperties::SET_STATE.as_ref()) {
            return Err(BehaviourCreationError);
        }
        if !e.properties.contains_key(ValueProperties::VALUE.as_ref()) {
            return Err(BehaviourCreationError);
        }
        // Debounce set_state -> state
        let handle_id_set_state = Uuid::new_v4().as_u128();
        let entity_instance = e.clone();
        e.properties
            .get(StateProperties::SET_STATE.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |new_value: &Value| {
                    if let Some(old_value) = entity_instance.get(StateProperties::STATE.as_ref()) {
                        let new_value = new_value.clone();
                        if old_value != new_value {
                            entity_instance.set(StateProperties::STATE.as_ref(), new_value);
                        }
                    }
                },
                handle_id_set_state,
            );
        // Propagate state -> value
        let handle_id_value = Uuid::new_v4().as_u128();
        let entity_instance = e.clone();
        e.properties
            .get(StateProperties::STATE.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |v: &Value| {
                    entity_instance.set(ValueProperties::VALUE.as_ref(), v.clone());
                },
                handle_id_set_state,
            );
        Ok(State {
            entity: e,
            handle_id_set_state,
            handle_id_value,
        })
    }
}

impl Disconnectable for State {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(StateProperties::SET_STATE.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id_set_state);
        }
        if let Some(property) = self.entity.properties.get(ValueProperties::VALUE.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id_value);
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        self.disconnect();
    }
}
