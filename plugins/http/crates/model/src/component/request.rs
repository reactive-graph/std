use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_HTTP;

properties!(
    RequestProperties,
    (METHOD, "method", "GET"),
    (REQUEST_HEADERS, "request_headers", {}),
    (PAYLOAD, "payload", {})
);

component_ty!(COMPONENT_REQUEST, NAMESPACE_HTTP, COMPONENT_NAME_REQUEST, "http");

component_model!(
    Request,
    set method string,
    set request_headers object,
    set payload value
);
