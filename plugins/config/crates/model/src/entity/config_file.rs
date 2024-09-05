use crate::NAMESPACE_CONFIG;
use reactive_graph_graph::entity_ty;
use reactive_graph_model_file::File;
use reactive_graph_model_result::ResultObject;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_CONFIG_FILE, NAMESPACE_CONFIG, ENTITY_TYPE_NAME_CONFIG_FILE, "config_file");

entity_model!(ConfigFile);
impl Action for ConfigFile {}
impl File for ConfigFile {}
impl ResultObject for ConfigFile {}
