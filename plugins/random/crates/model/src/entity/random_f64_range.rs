use crate::NAMESPACE_RANDOM;
use crate::RangeF64;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;
use reactive_graph_std_result_model::ResultNumberF64;

entity_ty!(ENTITY_TYPE_RANDOM_F64_RANGE, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_F64_RANGE, "random_f64_range");
behaviour_ty!(BEHAVIOUR_RANDOM_F64_RANGE, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_F64_RANGE, "random_f64_range");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_F64_RANGE, ENTITY_TYPE_RANDOM_F64_RANGE, BEHAVIOUR_RANDOM_F64_RANGE);

entity_model!(RandomF64Range);
impl Action for RandomF64Range {}
impl ResultNumberF64 for RandomF64Range {}
impl RangeF64 for RandomF64Range {}
