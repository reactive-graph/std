use crate::NAMESPACE_LOGICAL;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;
use reactive_graph_std_result_model::ResultAny;

properties!(TriggerProperties, (PAYLOAD, "payload", 0));

entity_ty!(ENTITY_TYPE_TRIGGER, NAMESPACE_LOGICAL, ENTITY_TYPE_NAME_TRIGGER, "trigger");
behaviour_ty!(BEHAVIOUR_TRIGGER, NAMESPACE_LOGICAL, BEHAVIOUR_NAME_TRIGGER, "trigger");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_TRIGGER, ENTITY_TYPE_TRIGGER, BEHAVIOUR_TRIGGER);

entity_model!(
    Trigger,
    set payload value
);
impl Action for Trigger {}
impl ResultAny for Trigger {}
