use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_HTTP;

properties!(ResponseProperties, (RESPONSE_HEADERS, "response_headers", {}), (STATUS, "status", 200));

component_ty!(COMPONENT_RESPONSE, NAMESPACE_HTTP, COMPONENT_NAME_RESPONSE, "response");

component_model!(
    Response,
    get response_headers object,
    get status u64
);
