use crate::NAMESPACE_HTTP;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(ResponseProperties, (RESPONSE_HEADERS, "response_headers", {}), (STATUS, "status", 200));

component_ty!(COMPONENT_RESPONSE, NAMESPACE_HTTP, COMPONENT_NAME_RESPONSE, "response");

component_model!(
    Response,
    get response_headers object,
    get status u64
);
