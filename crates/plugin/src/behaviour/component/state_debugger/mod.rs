use std::sync::Arc;

use log::debug;
use log::trace;
use serde_json::Value;

pub use functions::*;

use crate::model::*;
use crate::model_value::*;
use crate::reactive::*;

pub mod functions;

behaviour!(
    StateDebugger,
    StateDebuggerFactory,
    StateDebuggerFsm,
    StateDebuggerBehaviourTransitions,
    StateDebuggerValidator,
    ReactiveEntityInstance,
    f,
    StateDebuggerFunction
);

behaviour_validator!(StateDebuggerValidator, ReactiveEntityInstance, StateProperties::STATE.as_ref());

impl BehaviourTransitions<ReactiveEntityInstance> for StateDebuggerBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        let f = self.f;
        self.property_observers
            .observe_with_handle(StateProperties::STATE.as_ref(), move |v: &Value| f(v.clone(), reactive_instance.clone()));
        debug!("Starting debugging of {}[{}]", &self.property_observers.reactive_instance, StateProperties::STATE.as_ref());
        Ok(())
    }

    fn get_property_observers(&self) -> &PropertyObserverContainerImpl<ReactiveEntityInstance> {
        &self.property_observers
    }
}
