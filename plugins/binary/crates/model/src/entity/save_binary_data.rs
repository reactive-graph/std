use crate::BinaryData;
use crate::BinaryDataUrl;
use crate::NAMESPACE_BINARY;
use reactive_graph_graph::entity_ty;
use reactive_graph_model_base::Named;
use reactive_graph_model_file::File;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_SAVE_BINARY_DATA, NAMESPACE_BINARY, ENTITY_TYPE_NAME_SAVE_BINARY_DATA, "save_binary_data");

entity_model!(SaveBinaryData);
impl BinaryData for SaveBinaryData {}
impl BinaryDataUrl for SaveBinaryData {}
impl File for SaveBinaryData {}
impl Action for SaveBinaryData {}
impl Named for SaveBinaryData {}
