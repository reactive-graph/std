use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::relation_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use serde_json::json;

use reactive_graph_model_connector::ConnectorProperties::OUTBOUND_PROPERTY_NAME;
use reactive_graph_model_connector::PropagationCounterProperties::PROPAGATION_COUNT;

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
