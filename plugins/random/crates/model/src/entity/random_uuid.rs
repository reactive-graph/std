use crate::NAMESPACE_RANDOM;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;
use reactive_graph_std_result_model::ResultString;

entity_ty!(ENTITY_TYPE_RANDOM_UUID, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_UUID, "random_uuid");
behaviour_ty!(BEHAVIOUR_RANDOM_UUID, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_UUID, "random_uuid");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_UUID, ENTITY_TYPE_RANDOM_UUID, BEHAVIOUR_RANDOM_UUID);

entity_model!(RandomUuid);
impl Action for RandomUuid {}
impl ResultString for RandomUuid {}
