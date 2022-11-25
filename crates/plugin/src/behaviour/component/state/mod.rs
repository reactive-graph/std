use std::sync::Arc;

use lazy_static::lazy_static;
use log::trace;
use serde_json::Value;

use crate::model::BehaviourTypeId;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveBehaviourContainer;
use crate::model::ReactiveEntityInstance;
use crate::model_value::*;
use crate::reactive::*;

behaviour_types!(
    STATE_BEHAVIOURS,
    NAMESPACE_STATE,
    "state_array",
    "state_boolean",
    "state_number",
    "state_object",
    "state_string"
);

entity_behaviour!(State, StateFactory, StateFsm, StateBehaviourTransitions, StateValidator);

behaviour_validator!(
    StateValidator,
    ReactiveEntityInstance,
    StateProperties::STATE.as_ref(),
    StateProperties::SET_STATE.as_ref(),
    ValueProperties::VALUE.as_ref()
);

impl BehaviourInit<ReactiveEntityInstance> for StateBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        // If value and state are not equal propagate the state, initially
        let state = self.get(StateProperties::STATE.as_ref()).ok_or(BehaviourInitializationFailed {})?;
        let value = self.get(ValueProperties::VALUE.as_ref()).ok_or(BehaviourInitializationFailed {})?;
        if state != value {
            self.set(ValueProperties::VALUE.as_ref(), state);
        }
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for StateBehaviourTransitions {}

impl BehaviourConnect<ReactiveEntityInstance> for StateBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        self.property_observers
            .observe_with_handle(StateProperties::SET_STATE.as_ref(), move |new_value: &Value| {
                if let Some(old_value) = reactive_instance.get(StateProperties::STATE.as_ref()) {
                    let new_value = new_value.clone();
                    if old_value != new_value {
                        reactive_instance.set(StateProperties::STATE.as_ref(), new_value);
                    }
                }
            });
        // Propagate state -> value
        let reactive_instance = self.property_observers.reactive_instance.clone();
        self.property_observers.observe_with_handle(StateProperties::STATE.as_ref(), move |v: &Value| {
            reactive_instance.set(ValueProperties::VALUE.as_ref(), v.clone());
        });
        Ok(())
    }
}

impl BehaviourTransitions<ReactiveEntityInstance> for StateBehaviourTransitions {}
