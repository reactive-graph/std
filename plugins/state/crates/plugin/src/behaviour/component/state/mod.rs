use std::sync::Arc;
use std::sync::LazyLock;

use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity::EntityBehaviourFactories;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_api::ReactiveInstanceContainer;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_state::NAMESPACE_STATE;

use crate::model_state::StateProperties;
use crate::model_value::*;

pub static STATE_BEHAVIOURS: LazyLock<BehaviourTypeIds> = LazyLock::new(|| {
    BehaviourTypeIds::with_namespace(NAMESPACE_STATE)
        .ty("state_array")
        .ty("state_boolean")
        .ty("state_number")
        .ty("state_object")
        .ty("state_string")
        .into()
});

pub static STATE_FACTORIES: LazyLock<EntityBehaviourFactories> = LazyLock::new(|| {
    STATE_BEHAVIOURS.iter().fold(EntityBehaviourFactories::new(), |factories, behaviour_ty| {
        factories.factory(Arc::new(StateFactory::new(behaviour_ty.clone())))
    })
});

entity_behaviour!(State, StateFactory, StateFsm, StateBehaviourTransitions, StateValidator);

behaviour_validator!(
    StateValidator,
    Uuid,
    ReactiveEntity,
    StateProperties::STATE.as_ref(),
    StateProperties::SET_STATE.as_ref(),
    ValueProperties::VALUE.as_ref()
);

impl BehaviourInit<Uuid, ReactiveEntity> for StateBehaviourTransitions {
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

impl BehaviourShutdown<Uuid, ReactiveEntity> for StateBehaviourTransitions {}

impl BehaviourConnect<Uuid, ReactiveEntity> for StateBehaviourTransitions {
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

impl BehaviourTransitions<Uuid, ReactiveEntity> for StateBehaviourTransitions {}
