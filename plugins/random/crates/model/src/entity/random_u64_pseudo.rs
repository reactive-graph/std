use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_result::ResultNumberU64;
use crate::model_runtime::Action;
use crate::PseudoNumberGenerator;
use crate::NAMESPACE_RANDOM;

entity_ty!(ENTITY_TYPE_RANDOM_U64_PSEUDO, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_U64_PSEUDO, "random_u64_pseudo");
behaviour_ty!(BEHAVIOUR_RANDOM_U64_PSEUDO, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_U64_PSEUDO, "random_u64_pseudo");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_U64_PSEUDO, ENTITY_TYPE_RANDOM_U64_PSEUDO, BEHAVIOUR_RANDOM_U64_PSEUDO);

entity_model!(RandomU64Pseudo);
impl Action for RandomU64Pseudo {}
impl PseudoNumberGenerator for RandomU64Pseudo {}
impl ResultNumberU64 for RandomU64Pseudo {}
