use crate::behaviour_api::behaviour_ty;
use crate::behaviour_api::entity_behaviour_ty;
use crate::PseudoNumberGenerator;
use crate::NAMESPACE_RANDOM;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_result::ResultNumberU64;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

entity_ty!(ENTITY_TYPE_RANDOM_U64_PSEUDO, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_U64_PSEUDO, "random_u64_pseudo");
behaviour_ty!(BEHAVIOUR_RANDOM_U64_PSEUDO, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_U64_PSEUDO, "random_u64_pseudo");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_U64_PSEUDO, ENTITY_TYPE_RANDOM_U64_PSEUDO, BEHAVIOUR_RANDOM_U64_PSEUDO);

entity_model!(RandomU64Pseudo);
impl Action for RandomU64Pseudo {}
impl PseudoNumberGenerator for RandomU64Pseudo {}
impl ResultNumberU64 for RandomU64Pseudo {}
