use crate::NAMESPACE_SYSTEM_ENVIRONMENT;
use reactive_graph_graph::entity_ty;
use reactive_graph_reactive_model_api::entity_model;

entity_ty!(ENTITY_TYPE_SYSTEM_ENV_VAR, NAMESPACE_SYSTEM_ENVIRONMENT, ENTITY_TYPE_NAME_SYSTEM_ENV, "env_var");

entity_model!(
    EnvVar,
    get value string,
    get label string
);
