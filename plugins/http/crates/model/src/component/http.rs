use crate::model::behaviour_ty;
use crate::model::component_behaviour_ty;
use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_HTTP;

properties!(
    HttpProperties,
    (METHOD, "method", "GET"),
    (URL, "url", ""),
    (REQUEST_HEADERS, "request_headers", {}),
    (PAYLOAD, "payload", {}),
    (RESPONSE_HEADERS, "response_headers", {}),
    (RESULT, "result", {}),
    (STATUS, "status", 200)
);

component_ty!(COMPONENT_HTTP, NAMESPACE_HTTP, COMPONENT_NAME_HTTP, "http");
behaviour_ty!(BEHAVIOUR_HTTP, NAMESPACE_HTTP, BEHAVIOUR_NAME_HTTP, "http");
component_behaviour_ty!(COMPONENT_BEHAVIOUR_HTTP, COMPONENT_HTTP, BEHAVIOUR_HTTP);

component_model!(
    ComponentHttp,
    set method string,
    set url string,
    set request_headers object,
    set payload value,
    get response_headers object,
    get status u64
);
