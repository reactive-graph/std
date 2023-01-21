use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_result::ResultNumberF64;
use crate::model_trigger::Action;
use crate::PseudoNumberGenerator;
use crate::NAMESPACE_RANDOM;

entity_ty!(ENTITY_TYPE_RANDOM_F64_PSEUDO, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_F64_PSEUDO, "random_f64_pseudo");
behaviour_ty!(BEHAVIOUR_RANDOM_F64_PSEUDO, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_F64_PSEUDO, "random_f64_pseudo");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_F64_PSEUDO, ENTITY_TYPE_RANDOM_F64_PSEUDO, BEHAVIOUR_RANDOM_F64_PSEUDO);

entity_model!(RandomF64Pseudo);
impl Action for RandomF64Pseudo {}
impl PseudoNumberGenerator for RandomF64Pseudo {}
impl ResultNumberF64 for RandomF64Pseudo {}
