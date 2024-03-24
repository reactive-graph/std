use crate::NAMESPACE_RANDOM;
use inexor_rgf_behaviour_model_api::behaviour_ty;
use inexor_rgf_behaviour_model_api::entity_behaviour_ty;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_result::ResultNumberF64;
use inexor_rgf_reactive_model_api::entity_model;
use inexor_rgf_runtime_model::Action;

entity_ty!(ENTITY_TYPE_RANDOM_F64, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_F64, "random_f64");
behaviour_ty!(BEHAVIOUR_RANDOM_F64, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_F64, "random_f64");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_F64, ENTITY_TYPE_RANDOM_F64, BEHAVIOUR_RANDOM_F64);

entity_model!(RandomF64);
impl Action for RandomF64 {}
impl ResultNumberF64 for RandomF64 {}
