use crate::NAMESPACE_RANDOM;
use crate::RangeI64;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;
use reactive_graph_std_result_model::ResultNumberI64;

entity_ty!(ENTITY_TYPE_RANDOM_I64_RANGE, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_I64_RANGE, "random_i64_range");
behaviour_ty!(BEHAVIOUR_RANDOM_I64_RANGE, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_I64_RANGE, "random_i64_range");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_I64_RANGE, ENTITY_TYPE_RANDOM_I64_RANGE, BEHAVIOUR_RANDOM_I64_RANGE);

entity_model!(RandomI64Range);
impl Action for RandomI64Range {}
impl ResultNumberI64 for RandomI64Range {}
impl RangeI64 for RandomI64Range {}
