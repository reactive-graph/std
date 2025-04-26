use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;

use reactive_graph_std_result_model::ResultBoolean;

use crate::NAMESPACE_RANDOM;

entity_ty!(ENTITY_TYPE_RANDOM_BOOL, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_BOOL, "random_bool");
behaviour_ty!(BEHAVIOUR_RANDOM_BOOL, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_BOOL, "random_bool");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_BOOL, ENTITY_TYPE_RANDOM_BOOL, BEHAVIOUR_RANDOM_BOOL);

entity_model!(RandomBool);
impl Action for RandomBool {}
impl ResultBoolean for RandomBool {}
