use inexor_rgf_core_reactive::BehaviourDisconnectFailed;
use inexor_rgf_core_reactive::BehaviourState;
use std::sync::Arc;

use log::debug;
use serde_json::Value;
use uuid::Uuid;

use crate::behaviour::component::state_debugger::StateDebuggerFunction;
use crate::model::BehaviourTypeId;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactivePropertyContainer;
use crate::model_value::StateProperties;
use crate::reactive::Behaviour;
use crate::reactive::BehaviourConnectFailed;
use crate::reactive::BehaviourCreationError;
use crate::reactive::BehaviourPropertyInvalid;
use crate::reactive::BehaviourPropertyValidator;
use crate::reactive::BehaviourReactiveInstanceContainer;
use crate::reactive::BehaviourValidator;

pub struct StateDebugger {
    pub behaviour: StatefulBehaviour<ReactiveEntityInstance>,
    // pub entity: Arc<ReactiveEntityInstance>,
    // pub ty: BehaviourTypeId,
    // pub state: BehaviourState,
    pub f: StateDebuggerFunction,
    pub handle_id: u128,
}

impl StateDebugger {
    pub fn new(entity: Arc<ReactiveEntityInstance>, ty: BehaviourTypeId, f: StateDebuggerFunction) -> Result<StateDebugger, BehaviourCreationError> {
        Ok(StateDebugger {
            behaviour: StatefulBehaviour::new(entity, ty),
            f,
            handle_id: Uuid::new_v4().as_u128(),
        })
        // state_debugger.connect().map_err(|e| BehaviourCreationError::BehaviourConnectFailed(e))?;
    }
}

impl Behaviour<ReactiveEntityInstance> for StateDebugger {
    fn ty(&self) -> BehaviourTypeId {
        self.ty.clone()
    }

    fn get_state(&self) -> BehaviourState {
        self.state
    }

    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        // Validation Guard
        self.validate().map_err(|e| BehaviourConnectFailed::BehaviourInvalid(e))?;
        // Observer with function
        let entity = self.entity.clone();
        let f = self.f.clone();
        self.behaviour
            .reactive_instance
            .observe_with_handle(StateProperties::STATE.as_ref(), move |v: &Value| f(v.clone(), entity.clone()), self.handle_id);
        debug!("Starting debugging of {}[{}]", self.entity, StateProperties::STATE.as_ref());
        Ok(())
    }

    fn disconnect(&self) -> Result<(), BehaviourDisconnectFailed> {
        self.behaviour
            .reactive_instance
            .remove_observer(StateProperties::STATE.as_ref(), self.handle_id);
        debug!("Stopped debugging of {}[{}]", self.entity, StateProperties::STATE.as_ref());
        Ok(())
    }
}

impl BehaviourReactiveInstanceContainer<ReactiveEntityInstance> for StateDebugger {
    fn get_reactive_instance(&self) -> &Arc<ReactiveEntityInstance> {
        &self.entity
    }
}

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
