use crate::NAMESPACE_RANDOM;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_model_result::ResultNumberI64;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;

entity_ty!(ENTITY_TYPE_RANDOM_I64, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_I64, "random_i64");
behaviour_ty!(BEHAVIOUR_RANDOM_I64, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_I64, "random_i64");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_I64, ENTITY_TYPE_RANDOM_I64, BEHAVIOUR_RANDOM_I64);

entity_model!(RandomI64);
impl Action for RandomI64 {}
impl ResultNumberI64 for RandomI64 {}
