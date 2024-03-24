use crate::NAMESPACE_HTTP;
use inexor_rgf_behaviour_model_api::behaviour_ty;
use inexor_rgf_behaviour_model_api::component_behaviour_ty;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(
    JsonRpcProperties,
    (METHOD, "method", "POST"),
    (JSON_RPC_VERSION, "json_rpc_version", "2.0"),
    (PARAMS, "params", {}),
    (ERROR, "error", {})
);

component_ty!(COMPONENT_JSON_RPC, NAMESPACE_HTTP, COMPONENT_NAME_JSON_RPC, "json_rpc");
behaviour_ty!(BEHAVIOUR_JSON_RPC, NAMESPACE_HTTP, BEHAVIOUR_NAME_JSON_RPC, "json_rpc");
component_behaviour_ty!(COMPONENT_BEHAVIOUR_JSON_RPC, COMPONENT_JSON_RPC, BEHAVIOUR_JSON_RPC);

component_model!(
    ComponentJsonRpc,
    set method string,
    set json_rpc_version string,
    set params object,
    get error object
);
