use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_result::ResultNumberU64;
use crate::model_runtime::Action;
use crate::RangeU64;
use crate::NAMESPACE_RANDOM;

entity_ty!(ENTITY_TYPE_RANDOM_U64_RANGE, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_U64_RANGE, "random_u64_range");
behaviour_ty!(BEHAVIOUR_RANDOM_U64_RANGE, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_U64_RANGE, "random_u64_range");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_U64_RANGE, ENTITY_TYPE_RANDOM_U64_RANGE, BEHAVIOUR_RANDOM_U64_RANGE);

entity_model!(RandomU64Range);
impl Action for RandomU64Range {}
impl ResultNumberU64 for RandomU64Range {}
impl RangeU64 for RandomU64Range {}
