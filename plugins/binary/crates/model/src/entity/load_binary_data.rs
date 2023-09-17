use crate::BinaryData;
use crate::BinaryDataUrl;
use crate::NAMESPACE_BINARY;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_base::Named;
use inexor_rgf_model_file::File;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_LOAD_BINARY_DATA, NAMESPACE_BINARY, ENTITY_TYPE_NAME_LOAD_BINARY_DATA, "load_binary_data");

entity_model!(LoadBinaryData);
impl BinaryData for LoadBinaryData {}
impl BinaryDataUrl for LoadBinaryData {}
impl File for LoadBinaryData {}
impl Action for LoadBinaryData {}
impl Named for LoadBinaryData {}
