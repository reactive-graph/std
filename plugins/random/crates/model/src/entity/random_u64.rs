use crate::NAMESPACE_RANDOM;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;
use reactive_graph_std_result_model::ResultNumberU64;

entity_ty!(ENTITY_TYPE_RANDOM_U64, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_U64, "random_u64");
behaviour_ty!(BEHAVIOUR_RANDOM_U64, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_U64, "random_u64");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_U64, ENTITY_TYPE_RANDOM_U64, BEHAVIOUR_RANDOM_U64);

entity_model!(RandomU64);
impl Action for RandomU64 {}
impl ResultNumberU64 for RandomU64 {}
