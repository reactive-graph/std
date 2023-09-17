use crate::NAMESPACE_CONFIG;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_file::File;
use inexor_rgf_model_result::ResultObject;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_CONFIG_FILE, NAMESPACE_CONFIG, ENTITY_TYPE_NAME_CONFIG_FILE, "config_file");

entity_model!(ConfigFile);
impl Action for ConfigFile {}
impl File for ConfigFile {}
impl ResultObject for ConfigFile {}
