use crate::NAMESPACE_HTTP;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

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
