use log::debug;
use serde_json::Value;

pub use functions::*;

use crate::model::*;
use crate::model_value::*;
use crate::reactive::*;

pub mod functions;

entity_behaviour!(
    ValueDebugger,
    ValueDebuggerFactory,
    ValueDebuggerFsm,
    ValueDebuggerBehaviourTransitions,
    ValueDebuggerValidator,
    f,
    ValueDebuggerFunction
);

behaviour_validator!(ValueDebuggerValidator, ReactiveEntityInstance, ValueProperties::VALUE.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for ValueDebuggerBehaviourTransitions {}

impl BehaviourConnect<ReactiveEntityInstance> for ValueDebuggerBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let f = self.f;
        self.property_observers
            .observe_with_handle(ValueProperties::VALUE.as_ref(), move |v: &Value| f(v.clone()));
        debug!("Starting debugging of {}[{}]", &self.property_observers.reactive_instance, ValueProperties::VALUE.as_ref());
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for ValueDebuggerBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for ValueDebuggerBehaviourTransitions {}
