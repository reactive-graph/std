use crate::NAMESPACE_LOGICAL;
use reactive_graph_graph::entity_ty;
use reactive_graph_graph::flow_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_std_result_model::ResultBoolean;

properties!(And3Properties, (INPUT_1, "input1", false), (INPUT_2, "input2", false), (INPUT_3, "input3", false));

entity_ty!(ENTITY_TYPE_AND3, NAMESPACE_LOGICAL, ENTITY_TYPE_NAME_AND3, "and3");
flow_ty!(FLOW_TYPE_AND3, NAMESPACE_LOGICAL, FLOW_TYPE_NAME_AND3, "and3");

entity_model!(
    And3,
    set input1 bool,
    set input2 bool,
    set input3 bool
);
impl ResultBoolean for And3 {}
