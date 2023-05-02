use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_result::ResultNumberF64;
use crate::model_runtime::Action;
use crate::NAMESPACE_RANDOM;

entity_ty!(ENTITY_TYPE_RANDOM_F64, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_F64, "random_f64");
behaviour_ty!(BEHAVIOUR_RANDOM_F64, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_F64, "random_f64");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_F64, ENTITY_TYPE_RANDOM_F64, BEHAVIOUR_RANDOM_F64);

entity_model!(RandomF64);
impl Action for RandomF64 {}
impl ResultNumberF64 for RandomF64 {}
