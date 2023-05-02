use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_result::ResultNumberU64;
use crate::model_runtime::Action;
use crate::NAMESPACE_RANDOM;

entity_ty!(ENTITY_TYPE_RANDOM_U64, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_U64, "random_u64");
behaviour_ty!(BEHAVIOUR_RANDOM_U64, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_U64, "random_u64");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_U64, ENTITY_TYPE_RANDOM_U64, BEHAVIOUR_RANDOM_U64);

entity_model!(RandomU64);
impl Action for RandomU64 {}
impl ResultNumberU64 for RandomU64 {}
