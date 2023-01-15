use serde_json::json;

use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveRelationInstance;
use crate::model_connector::ConnectorProperties::OUTBOUND_PROPERTY_NAME;
use crate::model_connector::PropagationCounterProperties::PROPAGATION_COUNT;
use crate::reactive::*;

relation_behaviour!(
    PropagationCounter,
    PropagationCounterFactory,
    PropagationCounterFsm,
    PropagationCounterBehaviourTransitions,
    PropagationCounterValidator
);

behaviour_validator!(
    PropagationCounterValidator,
    ReactiveRelationInstance,
    OUTBOUND_PROPERTY_NAME.as_ref(),
    PROPAGATION_COUNT.as_ref()
);

impl PropagationCounterBehaviourTransitions {
    fn get_outbound_property_name(&self) -> Option<String> {
        self.reactive_instance.as_string(OUTBOUND_PROPERTY_NAME.as_ref())
    }
}

impl BehaviourInit<ReactiveRelationInstance> for PropagationCounterBehaviourTransitions {}

impl BehaviourShutdown<ReactiveRelationInstance> for PropagationCounterBehaviourTransitions {}

impl BehaviourConnect<ReactiveRelationInstance> for PropagationCounterBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let outbound_property_name = self.get_outbound_property_name().ok_or(BehaviourConnectFailed {})?;
        let reactive_instance = self.reactive_instance.clone();
        self.outbound_property_observers.observe_with_handle(outbound_property_name.as_str(), move |_| {
            if let Some(mut count) = reactive_instance.as_u64(PROPAGATION_COUNT) {
                count += 1;
                reactive_instance.set(PROPAGATION_COUNT.as_ref(), json!(count));
            }
        });
        Ok(())
    }
}

impl BehaviourTransitions<ReactiveRelationInstance> for PropagationCounterBehaviourTransitions {}
