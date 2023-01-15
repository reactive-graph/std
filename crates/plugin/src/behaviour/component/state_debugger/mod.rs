use log::debug;
use serde_json::Value;

pub use functions::*;

use crate::model::*;
use crate::model_value::*;
use crate::reactive::behaviour_validator;
use crate::reactive::entity_behaviour;
use crate::reactive::BehaviourConnect;
use crate::reactive::BehaviourConnectFailed;
use crate::reactive::BehaviourDisconnect;
use crate::reactive::BehaviourFsm;
use crate::reactive::BehaviourInit;
use crate::reactive::BehaviourShutdown;
use crate::reactive::BehaviourTransitions;
use crate::reactive::PropertyObserverContainer;

pub mod functions;

entity_behaviour!(
    StateDebugger,
    StateDebuggerFactory,
    StateDebuggerFsm,
    StateDebuggerBehaviourTransitions,
    StateDebuggerValidator,
    f,
    StateDebuggerFunction
);

behaviour_validator!(StateDebuggerValidator, ReactiveEntityInstance, StateProperties::STATE.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for StateDebuggerBehaviourTransitions {}

impl BehaviourConnect<ReactiveEntityInstance> for StateDebuggerBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        let f = self.f;
        self.property_observers
            .observe_with_handle(StateProperties::STATE.as_ref(), move |v: &Value| f(v.clone(), reactive_instance.clone()));
        debug!("Starting debugging of {}[{}]", &self.property_observers.reactive_instance, StateProperties::STATE.as_ref());
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for StateDebuggerBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for StateDebuggerBehaviourTransitions {}
