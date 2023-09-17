use crate::behaviour_api::behaviour_ty;
use crate::behaviour_api::entity_behaviour_ty;
use crate::RangeU64;
use crate::NAMESPACE_RANDOM;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_result::ResultNumberU64;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

entity_ty!(ENTITY_TYPE_RANDOM_U64_RANGE, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_U64_RANGE, "random_u64_range");
behaviour_ty!(BEHAVIOUR_RANDOM_U64_RANGE, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_U64_RANGE, "random_u64_range");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_U64_RANGE, ENTITY_TYPE_RANDOM_U64_RANGE, BEHAVIOUR_RANDOM_U64_RANGE);

entity_model!(RandomU64Range);
impl Action for RandomU64Range {}
impl ResultNumberU64 for RandomU64Range {}
impl RangeU64 for RandomU64Range {}
