use crate::model::entity_model;
use crate::model::entity_ty;
use crate::NAMESPACE_SYSTEM_ENVIRONMENT;

entity_ty!(ENTITY_TYPE_SYSTEM_ENV, NAMESPACE_SYSTEM_ENVIRONMENT, ENTITY_TYPE_NAME_SYSTEM_ENV, "system_env");

entity_model!(
    SystemEnv,
    get value string,
    get label string
);
