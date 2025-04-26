use crate::NAMESPACE_RANDOM;
use crate::PseudoNumberGenerator;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;
use reactive_graph_std_result_model::ResultNumberI64;

entity_ty!(ENTITY_TYPE_RANDOM_I64_PSEUDO, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_I64_PSEUDO, "random_i64_pseudo");
behaviour_ty!(BEHAVIOUR_RANDOM_I64_PSEUDO, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_I64_PSEUDO, "random_i64_pseudo");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_I64_PSEUDO, ENTITY_TYPE_RANDOM_I64_PSEUDO, BEHAVIOUR_RANDOM_I64_PSEUDO);

entity_model!(RandomI64Pseudo);
impl Action for RandomI64Pseudo {}
impl PseudoNumberGenerator for RandomI64Pseudo {}
impl ResultNumberI64 for RandomI64Pseudo {}
