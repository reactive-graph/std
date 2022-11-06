use std::sync::Arc;

use serde_json::Value;
use uuid::Uuid;

use crate::behaviour::component::StateProperties;
use crate::behaviour::component::ValueProperties;
use crate::model::BehaviourTypeId;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactivePropertyContainer;
use crate::reactive::Behaviour;
use crate::reactive::BehaviourConnectFailed;
use crate::reactive::BehaviourCreationError;
use crate::reactive::BehaviourInitializationFailed;
use crate::reactive::BehaviourInitializer;
use crate::reactive::BehaviourPropertyInvalid;
use crate::reactive::BehaviourPropertyValidator;
use crate::reactive::BehaviourReactiveInstanceContainer;
use crate::reactive::BehaviourValidator;

pub struct State {
    pub entity: Arc<ReactiveEntityInstance>,
    pub ty: BehaviourTypeId,
    pub handle_id_set_state: u128,
    pub handle_id_value: u128,
}

impl State {
    pub fn new(entity: Arc<ReactiveEntityInstance>, ty: BehaviourTypeId) -> Result<State, BehaviourCreationError> {
        let state = State {
            entity,
            ty,
            handle_id_set_state: Uuid::new_v4().as_u128(),
            handle_id_value: Uuid::new_v4().as_u128(),
        };
        state.connect().map_err(|e| BehaviourCreationError::BehaviourConnectFailed(e))?;
        Ok(state)
    }
}

impl Behaviour<ReactiveEntityInstance> for State {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        // Validation Guard
        self.validate().map_err(|e| BehaviourConnectFailed::BehaviourInvalid(e))?;
        // Initialize the behaviour
        self.init().map_err(|_| BehaviourConnectFailed::BehaviourInitializationFailed)?;
        // Debounce set_state -> state
        let entity = self.entity.clone();
        self.entity.observe_with_handle(
            StateProperties::SET_STATE.as_ref(),
            move |new_value: &Value| {
                if let Some(old_value) = entity.get(StateProperties::STATE.as_ref()) {
                    let new_value = new_value.clone();
                    if old_value != new_value {
                        entity.set(StateProperties::STATE.as_ref(), new_value);
                    }
                }
            },
            self.handle_id_set_state,
        );
        // Propagate state -> value
        let entity = self.entity.clone();
        self.entity.observe_with_handle(
            StateProperties::STATE.as_ref(),
            move |v: &Value| {
                entity.set(ValueProperties::VALUE.as_ref(), v.clone());
            },
            self.handle_id_value,
        );
        Ok(())
    }

    fn disconnect(&self) {
        self.entity.remove_observer(StateProperties::SET_STATE.as_ref(), self.handle_id_set_state);
        self.entity.remove_observer(ValueProperties::VALUE.as_ref(), self.handle_id_value);
    }

    fn ty(&self) -> BehaviourTypeId {
        self.ty.clone()
    }
}

impl BehaviourReactiveInstanceContainer<ReactiveEntityInstance> for State {
    fn get_reactive_instance(&self) -> &Arc<ReactiveEntityInstance> {
        &self.entity
    }
}

impl BehaviourInitializer for State {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        // If value and state are not equal propagate the state, initially
        let state = self.entity.get(StateProperties::STATE.as_ref()).ok_or(BehaviourInitializationFailed {})?;
        let value = self.entity.get(ValueProperties::VALUE.as_ref()).ok_or(BehaviourInitializationFailed {})?;
        if state != value {
            self.entity.set(ValueProperties::VALUE.as_ref(), state);
        }
        Ok(())
    }
}

impl BehaviourValidator<ReactiveEntityInstance> for State {}

impl BehaviourPropertyValidator<ReactiveEntityInstance> for State {
    fn validate_properties(&self) -> Result<(), BehaviourPropertyInvalid> {
        self.validate_property(StateProperties::STATE.as_ref())?;
        self.validate_property(StateProperties::SET_STATE.as_ref())?;
        self.validate_property(ValueProperties::VALUE.as_ref())?;
        Ok(())
    }
}

impl Drop for State {
    fn drop(&mut self) {
        self.disconnect();
    }
}
