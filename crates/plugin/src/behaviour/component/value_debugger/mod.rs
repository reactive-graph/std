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
    ValueDebugger,
    ValueDebuggerFactory,
    ValueDebuggerFsm,
    ValueDebuggerBehaviourTransitions,
    ValueDebuggerValidator,
    ReactiveEntityInstance,
    f,
    ValueDebuggerFunction
);

behaviour_validator!(ValueDebuggerValidator, ReactiveEntityInstance, ValueProperties::VALUE.as_ref());

impl BehaviourTransitions<ReactiveEntityInstance> for ValueDebuggerBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let f = self.f;
        self.property_observers
            .observe_with_handle(ValueProperties::VALUE.as_ref(), move |v: &Value| f(v.clone()));
        debug!("Starting debugging of {}[{}]", &self.property_observers.reactive_instance, ValueProperties::VALUE.as_ref());
        Ok(())
    }

    fn get_property_observers(&self) -> &PropertyObserverContainerImpl<ReactiveEntityInstance> {
        &self.property_observers
    }
}
