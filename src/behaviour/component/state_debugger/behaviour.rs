use std::sync::Arc;

use log::debug;
use serde_json::Value;
use uuid::Uuid;

use crate::behaviour::component::state_debugger::StateDebuggerFunction;
use crate::behaviour::component::StateProperties;
use crate::model::BehaviourTypeId;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactivePropertyContainer;
use crate::reactive::Behaviour;
use crate::reactive::BehaviourConnectFailed;
use crate::reactive::BehaviourCreationError;
use crate::reactive::BehaviourInitializer;
use crate::reactive::BehaviourPropertyInvalid;
use crate::reactive::BehaviourPropertyValidator;
use crate::reactive::BehaviourReactiveInstanceContainer;
use crate::reactive::BehaviourValidator;

pub struct StateDebugger {
    pub entity: Arc<ReactiveEntityInstance>,

    pub ty: BehaviourTypeId,

    pub f: StateDebuggerFunction,

    pub handle_id: u128,
}

impl StateDebugger {
    pub fn new(entity: Arc<ReactiveEntityInstance>, ty: BehaviourTypeId, f: StateDebuggerFunction) -> Result<StateDebugger, BehaviourCreationError> {
        let state_debugger = StateDebugger {
            entity,
            ty,
            f,
            handle_id: Uuid::new_v4().as_u128(),
        };
        state_debugger.connect().map_err(|e| BehaviourCreationError::BehaviourConnectFailed(e))?;
        Ok(state_debugger)
    }
}

impl Behaviour<ReactiveEntityInstance> for StateDebugger {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        // Validation Guard
        self.validate().map_err(|e| BehaviourConnectFailed::BehaviourInvalid(e))?;
        // Observer with function
        let entity = self.entity.clone();
        let f = self.f.clone();
        self.entity
            .observe_with_handle(StateProperties::STATE.as_ref(), move |v: &Value| f(v.clone(), entity.clone()), self.handle_id);
        debug!("Starting debugging of {}[{}]", self.entity, StateProperties::STATE.as_ref());
        Ok(())
    }

    fn disconnect(&self) {
        self.entity.remove_observer(StateProperties::STATE.as_ref(), self.handle_id);
        debug!("Stopped debugging of {}[{}]", self.entity, StateProperties::STATE.as_ref());
    }

    fn ty(&self) -> BehaviourTypeId {
        self.ty.clone()
    }
}

impl BehaviourReactiveInstanceContainer<ReactiveEntityInstance> for StateDebugger {
    fn get_reactive_instance(&self) -> &Arc<ReactiveEntityInstance> {
        &self.entity
    }
}

impl BehaviourInitializer for StateDebugger {}

impl BehaviourValidator<ReactiveEntityInstance> for StateDebugger {}

impl BehaviourPropertyValidator<ReactiveEntityInstance> for StateDebugger {
    fn validate_properties(&self) -> Result<(), BehaviourPropertyInvalid> {
        self.validate_property(StateProperties::STATE.as_ref())
    }
}

impl Drop for StateDebugger {
    fn drop(&mut self) {
        self.disconnect();
    }
}
