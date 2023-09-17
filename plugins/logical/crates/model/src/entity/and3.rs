use crate::NAMESPACE_LOGICAL;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_graph::flow_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_model_result::ResultBoolean;
use inexor_rgf_reactive_api::entity_model;

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
