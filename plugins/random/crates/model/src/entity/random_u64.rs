use crate::behaviour_api::behaviour_ty;
use crate::behaviour_api::entity_behaviour_ty;
use crate::NAMESPACE_RANDOM;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_result::ResultNumberU64;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

entity_ty!(ENTITY_TYPE_RANDOM_U64, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_U64, "random_u64");
behaviour_ty!(BEHAVIOUR_RANDOM_U64, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_U64, "random_u64");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_U64, ENTITY_TYPE_RANDOM_U64, BEHAVIOUR_RANDOM_U64);

entity_model!(RandomU64);
impl Action for RandomU64 {}
impl ResultNumberU64 for RandomU64 {}
