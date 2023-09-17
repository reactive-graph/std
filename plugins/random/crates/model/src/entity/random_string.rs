use crate::behaviour_api::behaviour_ty;
use crate::behaviour_api::entity_behaviour_ty;
use crate::NAMESPACE_RANDOM;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_model_result::ResultString;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

properties!(RandomStringProperties, (LENGTH, "length", 10));

entity_ty!(ENTITY_TYPE_RANDOM_STRING, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_STRING, "random_string");
behaviour_ty!(BEHAVIOUR_RANDOM_STRING, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_STRING, "random_string");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_STRING, ENTITY_TYPE_RANDOM_STRING, BEHAVIOUR_RANDOM_STRING);

entity_model!(
    RandomString,
    set length u64
);
impl Action for RandomString {}
impl ResultString for RandomString {}
