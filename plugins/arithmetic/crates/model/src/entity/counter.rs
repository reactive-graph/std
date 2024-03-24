use inexor_rgf_behaviour_model_api::behaviour_ty;
use inexor_rgf_behaviour_model_api::entity_behaviour_ty;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_reactive_model_api::entity_model;
use inexor_rgf_reactive_service_api::reactive_entity;
use inexor_rgf_runtime_model::Action;

use inexor_rgf_model_result::ResultNumberU64;

use crate::NAMESPACE_ARITHMETIC_U64;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_COUNTER, NAMESPACE_ARITHMETIC_U64, ENTITY_TYPE_NAME_COUNTER, "counter");
behaviour_ty!(BEHAVIOUR_COUNTER, NAMESPACE_ARITHMETIC_U64, BEHAVIOUR_NAME_COUNTER, "counter");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_COUNTER, ENTITY_TYPE_COUNTER, BEHAVIOUR_COUNTER);

#[reactive_entity(namespace = "arithmetic_u64", type_name = "counter")]
pub struct Counter1 {
    pub trigger: bool,
    pub result: u64,
}

entity_model!(Counter);
impl Action for Counter {}
impl ResultNumberU64 for Counter {}
