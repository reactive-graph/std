use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_result::ResultNumberF64;
use crate::model_runtime::Action;
use crate::RangeF64;
use crate::NAMESPACE_RANDOM;

entity_ty!(ENTITY_TYPE_RANDOM_F64_RANGE, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_F64_RANGE, "random_f64_range");
behaviour_ty!(BEHAVIOUR_RANDOM_F64_RANGE, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_F64_RANGE, "random_f64_range");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_F64_RANGE, ENTITY_TYPE_RANDOM_F64_RANGE, BEHAVIOUR_RANDOM_F64_RANGE);

entity_model!(RandomF64Range);
impl Action for RandomF64Range {}
impl ResultNumberF64 for RandomF64Range {}
impl RangeF64 for RandomF64Range {}
