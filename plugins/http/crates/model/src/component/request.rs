use crate::NAMESPACE_HTTP;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

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
