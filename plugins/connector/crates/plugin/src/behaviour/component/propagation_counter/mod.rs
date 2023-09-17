use inexor_rgf_behaviour::relation_behaviour;
use inexor_rgf_behaviour::PropertyObserverContainer;
use inexor_rgf_behaviour_api::behaviour_validator;
use inexor_rgf_behaviour_api::prelude::*;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive::ReactiveRelation;
use serde_json::json;

use inexor_rgf_model_connector::ConnectorProperties::OUTBOUND_PROPERTY_NAME;
use inexor_rgf_model_connector::PropagationCounterProperties::PROPAGATION_COUNT;

relation_behaviour!(
    PropagationCounter,
    PropagationCounterFactory,
    PropagationCounterFsm,
    PropagationCounterBehaviourTransitions,
    PropagationCounterValidator
);

behaviour_validator!(
    PropagationCounterValidator,
    RelationInstanceId,
    ReactiveRelation,
    OUTBOUND_PROPERTY_NAME.as_ref(),
    PROPAGATION_COUNT.as_ref()
);

impl PropagationCounterBehaviourTransitions {
    fn get_outbound_property_name(&self) -> Option<String> {
        self.reactive_instance.as_string(OUTBOUND_PROPERTY_NAME.as_ref())
    }
}

impl BehaviourInit<RelationInstanceId, ReactiveRelation> for PropagationCounterBehaviourTransitions {}

impl BehaviourShutdown<RelationInstanceId, ReactiveRelation> for PropagationCounterBehaviourTransitions {}

impl BehaviourConnect<RelationInstanceId, ReactiveRelation> for PropagationCounterBehaviourTransitions {
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

impl BehaviourTransitions<RelationInstanceId, ReactiveRelation> for PropagationCounterBehaviourTransitions {}
