use crate::NAMESPACE_RANDOM;
use inexor_rgf_behaviour_model_api::behaviour_ty;
use inexor_rgf_behaviour_model_api::entity_behaviour_ty;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_result::ResultNumberI64;
use inexor_rgf_reactive_model_api::entity_model;
use inexor_rgf_runtime_model::Action;

entity_ty!(ENTITY_TYPE_RANDOM_I64, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_I64, "random_i64");
behaviour_ty!(BEHAVIOUR_RANDOM_I64, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_I64, "random_i64");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_I64, ENTITY_TYPE_RANDOM_I64, BEHAVIOUR_RANDOM_I64);

entity_model!(RandomI64);
impl Action for RandomI64 {}
impl ResultNumberI64 for RandomI64 {}
