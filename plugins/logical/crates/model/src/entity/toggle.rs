use crate::behaviour_api::behaviour_ty;
use crate::behaviour_api::entity_behaviour_ty;
use crate::NAMESPACE_LOGICAL;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_result::ResultBoolean;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_TOGGLE, NAMESPACE_LOGICAL, ENTITY_TYPE_NAME_TOGGLE, "toggle");
behaviour_ty!(BEHAVIOUR_TOGGLE, NAMESPACE_LOGICAL, BEHAVIOUR_NAME_TOGGLE, "toggle");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_TOGGLE, ENTITY_TYPE_TOGGLE, BEHAVIOUR_TOGGLE);

entity_model!(Toggle);
impl Action for Toggle {}
impl ResultBoolean for Toggle {}
