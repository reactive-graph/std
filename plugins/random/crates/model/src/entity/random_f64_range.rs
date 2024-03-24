use crate::RangeF64;
use crate::NAMESPACE_RANDOM;
use inexor_rgf_behaviour_model_api::behaviour_ty;
use inexor_rgf_behaviour_model_api::entity_behaviour_ty;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_result::ResultNumberF64;
use inexor_rgf_reactive_model_api::entity_model;
use inexor_rgf_runtime_model::Action;

entity_ty!(ENTITY_TYPE_RANDOM_F64_RANGE, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_F64_RANGE, "random_f64_range");
behaviour_ty!(BEHAVIOUR_RANDOM_F64_RANGE, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_F64_RANGE, "random_f64_range");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_F64_RANGE, ENTITY_TYPE_RANDOM_F64_RANGE, BEHAVIOUR_RANDOM_F64_RANGE);

entity_model!(RandomF64Range);
impl Action for RandomF64Range {}
impl ResultNumberF64 for RandomF64Range {}
impl RangeF64 for RandomF64Range {}
