use crate::behaviour_api::behaviour_ty;
use crate::behaviour_api::entity_behaviour_ty;
use crate::NAMESPACE_RANDOM;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_result::ResultBoolean;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

entity_ty!(ENTITY_TYPE_RANDOM_BOOL, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_BOOL, "random_bool");
behaviour_ty!(BEHAVIOUR_RANDOM_BOOL, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_BOOL, "random_bool");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_BOOL, ENTITY_TYPE_RANDOM_BOOL, BEHAVIOUR_RANDOM_BOOL);

entity_model!(RandomBool);
impl Action for RandomBool {}
impl ResultBoolean for RandomBool {}
