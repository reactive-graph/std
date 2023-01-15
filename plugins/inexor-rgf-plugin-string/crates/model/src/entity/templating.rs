use serde_json::json;

use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::NAMESPACE_STRING;

properties!(TemplatingProperties, (TEMPLATE, "template", ""), (CONTEXT, "context", json!({})), (RESULT, "result", ""));

entity_ty!(ENTITY_TYPE_TEMPLATING, NAMESPACE_STRING, ENTITY_TYPE_NAME_TEMPLATING, "templating");
behaviour_ty!(BEHAVIOUR_TEMPLATING, NAMESPACE_STRING, BEHAVIOUR_NAME_TEMPLATING, "templating");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_TEMPLATING, ENTITY_TYPE_TEMPLATING, BEHAVIOUR_TEMPLATING);

entity_model!(
    Templating,
    get result string,
    set context value,
    set template string
);
