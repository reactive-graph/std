use std::sync::Arc;

use serde_json::Value;
use uuid::Uuid;

use crate::behaviour::component::StateProperties;
use crate::behaviour::component::ValueProperties;
use crate::model::BehaviourTypeId;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::Behaviour;
use crate::reactive::BehaviourCreationError;

pub struct State {
    pub entity: Arc<ReactiveEntityInstance>,

    pub ty: BehaviourTypeId,

    pub handle_id_set_state: u128,

    pub handle_id_value: u128,
}

impl State {
    pub fn new(e: Arc<ReactiveEntityInstance>, ty: BehaviourTypeId) -> Result<State, BehaviourCreationError> {
        let property_state = e.properties.get(StateProperties::STATE.as_ref()).ok_or(BehaviourCreationError)?;
        let property_set_state = e.properties.get(StateProperties::SET_STATE.as_ref()).ok_or(BehaviourCreationError)?;
        if !e.properties.contains_key(ValueProperties::VALUE.as_ref()) {
            return Err(BehaviourCreationError);
        }
        // Debounce set_state -> state
        let handle_id_set_state = Uuid::new_v4().as_u128();
        let entity = e.clone();
        property_set_state.stream.read().unwrap().observe_with_handle(
            move |new_value: &Value| {
                if let Some(old_value) = entity.get(StateProperties::STATE.as_ref()) {
                    let new_value = new_value.clone();
                    if old_value != new_value {
                        entity.set(StateProperties::STATE.as_ref(), new_value);
                    }
                }
            },
            handle_id_set_state,
        );
        // Propagate state -> value
        let handle_id_value = Uuid::new_v4().as_u128();
        let entity = e.clone();
        property_state.stream.read().unwrap().observe_with_handle(
            move |v: &Value| {
                entity.set(ValueProperties::VALUE.as_ref(), v.clone());
            },
            handle_id_value,
        );
        let entity = e.clone();
        Ok(State {
            entity,
            ty,
            handle_id_set_state,
            handle_id_value,
        })
    }
}

impl Behaviour for State {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(StateProperties::SET_STATE.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id_set_state);
        }
        if let Some(property) = self.entity.properties.get(ValueProperties::VALUE.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id_value);
        }
    }

    fn ty(&self) -> BehaviourTypeId {
        self.ty.clone()
    }
}

impl Drop for State {
    fn drop(&mut self) {
        self.disconnect();
    }
}
