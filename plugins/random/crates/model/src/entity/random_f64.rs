use crate::NAMESPACE_RANDOM;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;
use reactive_graph_std_result_model::ResultNumberF64;

entity_ty!(ENTITY_TYPE_RANDOM_F64, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_F64, "random_f64");
behaviour_ty!(BEHAVIOUR_RANDOM_F64, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_F64, "random_f64");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_F64, ENTITY_TYPE_RANDOM_F64, BEHAVIOUR_RANDOM_F64);

entity_model!(RandomF64);
impl Action for RandomF64 {}
impl ResultNumberF64 for RandomF64 {}
