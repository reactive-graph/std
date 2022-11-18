use inexor_rgf_core_reactive::BehaviourDisconnectFailed;
use std::sync::Arc;

use log::debug;
use serde_json::Value;
use uuid::Uuid;

use crate::behaviour::component::value_debugger::ValueDebuggerFunction;
use crate::model::BehaviourTypeId;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactivePropertyContainer;
use crate::model_value::ValueProperties;
use crate::reactive::Behaviour;
use crate::reactive::BehaviourConnectFailed;
use crate::reactive::BehaviourCreationError;
use crate::reactive::BehaviourPropertyInvalid;
use crate::reactive::BehaviourPropertyValidator;
use crate::reactive::BehaviourReactiveInstanceContainer;
use crate::reactive::BehaviourValidator;

pub struct ValueDebugger {
    pub entity: Arc<ReactiveEntityInstance>,

    pub ty: BehaviourTypeId,

    pub f: ValueDebuggerFunction,

    pub handle_id: u128,
}

impl ValueDebugger {
    pub fn new(entity: Arc<ReactiveEntityInstance>, ty: BehaviourTypeId, f: ValueDebuggerFunction) -> Result<ValueDebugger, BehaviourCreationError> {
        let value_debugger = ValueDebugger {
            entity,
            ty,
            f,
            handle_id: Uuid::new_v4().as_u128(),
        };
        value_debugger.connect().map_err(|e| BehaviourCreationError::BehaviourConnectFailed(e))?;
        Ok(value_debugger)
    }
}

impl Behaviour<ReactiveEntityInstance> for ValueDebugger {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        // Validation Guard
        self.validate().map_err(|e| BehaviourConnectFailed::BehaviourInvalid(e))?;
        // Observer with function
        let f = self.f.clone();
        self.entity
            .observe_with_handle(ValueProperties::VALUE.as_ref(), move |v: &Value| f(v.clone()), self.handle_id);
        debug!("Starting debugging of {}[{}]", self.entity, ValueProperties::VALUE.as_ref());
        Ok(())
    }

    fn disconnect(&self) -> Result<(), BehaviourDisconnectFailed> {
        self.entity.remove_observer(ValueProperties::VALUE.as_ref(), self.handle_id);
        debug!("Stopped debugging of {}[{}]", self.entity, ValueProperties::VALUE.as_ref());
        Ok(())
    }

    fn ty(&self) -> BehaviourTypeId {
        self.ty.clone()
    }
}

impl BehaviourReactiveInstanceContainer<ReactiveEntityInstance> for ValueDebugger {
    fn get_reactive_instance(&self) -> &Arc<ReactiveEntityInstance> {
        &self.entity
    }
}

impl BehaviourValidator<ReactiveEntityInstance> for ValueDebugger {}

impl BehaviourPropertyValidator<ReactiveEntityInstance> for ValueDebugger {
    fn validate_properties(&self) -> Result<(), BehaviourPropertyInvalid> {
        self.validate_property(ValueProperties::VALUE.as_ref())
    }
}

impl Drop for ValueDebugger {
    fn drop(&mut self) {
        self.disconnect();
    }
}
