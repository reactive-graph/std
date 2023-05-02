use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_result::ResultBoolean;
use crate::model_runtime::Action;
use crate::NAMESPACE_RANDOM;

entity_ty!(ENTITY_TYPE_RANDOM_BOOL, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_BOOL, "random_bool");
behaviour_ty!(BEHAVIOUR_RANDOM_BOOL, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_BOOL, "random_bool");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_BOOL, ENTITY_TYPE_RANDOM_BOOL, BEHAVIOUR_RANDOM_BOOL);

entity_model!(RandomBool);
impl Action for RandomBool {}
impl ResultBoolean for RandomBool {}
