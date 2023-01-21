use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_result::ResultNumberI64;
use crate::model_trigger::Action;
use crate::RangeI64;
use crate::NAMESPACE_RANDOM;

entity_ty!(ENTITY_TYPE_RANDOM_I64_RANGE, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_I64_RANGE, "random_i64_range");
behaviour_ty!(BEHAVIOUR_RANDOM_I64_RANGE, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_I64_RANGE, "random_i64_range");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_I64_RANGE, ENTITY_TYPE_RANDOM_I64_RANGE, BEHAVIOUR_RANDOM_I64_RANGE);

entity_model!(RandomI64Range);
impl Action for RandomI64Range {}
impl ResultNumberI64 for RandomI64Range {}
impl RangeI64 for RandomI64Range {}
