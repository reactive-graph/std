use crate::behaviour_api::behaviour_ty;
use crate::behaviour_api::entity_behaviour_ty;
use crate::NAMESPACE_RANDOM;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_result::ResultString;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

entity_ty!(ENTITY_TYPE_RANDOM_UUID, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_UUID, "random_uuid");
behaviour_ty!(BEHAVIOUR_RANDOM_UUID, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_UUID, "random_uuid");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_UUID, ENTITY_TYPE_RANDOM_UUID, BEHAVIOUR_RANDOM_UUID);

entity_model!(RandomUuid);
impl Action for RandomUuid {}
impl ResultString for RandomUuid {}
