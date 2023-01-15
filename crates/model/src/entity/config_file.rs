use crate::model::entity_model;
use crate::model::entity_ty;
use crate::NAMESPACE_CONFIG;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_CONFIG_FILE, NAMESPACE_CONFIG, ENTITY_TYPE_NAME_CONFIG_FILE, "config_file");

entity_model!(
    ConfigFile,
    get result value,
    set trigger bool,
    set filename string
);
