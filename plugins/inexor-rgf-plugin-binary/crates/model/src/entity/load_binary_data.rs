use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_file::File;
use crate::model_logical::Action;
use crate::BinaryData;
use crate::BinaryDataUrl;
use crate::NAMESPACE_BINARY;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_LOAD_BINARY_DATA, NAMESPACE_BINARY, ENTITY_TYPE_NAME_LOAD_BINARY_DATA, "load_binary_data");

entity_model!(LoadBinaryData);
impl BinaryData for LoadBinaryData {}
impl BinaryDataUrl for LoadBinaryData {}
impl File for LoadBinaryData {}
impl Action for LoadBinaryData {}
// impl model_base::Named for LoadBinaryData {}
