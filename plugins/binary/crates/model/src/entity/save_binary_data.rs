use crate::BinaryData;
use crate::BinaryDataUrl;
use crate::NAMESPACE_BINARY;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_base::Named;
use inexor_rgf_model_file::File;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_SAVE_BINARY_DATA, NAMESPACE_BINARY, ENTITY_TYPE_NAME_SAVE_BINARY_DATA, "save_binary_data");

entity_model!(SaveBinaryData);
impl BinaryData for SaveBinaryData {}
impl BinaryDataUrl for SaveBinaryData {}
impl File for SaveBinaryData {}
impl Action for SaveBinaryData {}
impl Named for SaveBinaryData {}
