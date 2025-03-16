use crate::ComponentJsonRpc;
use crate::NAMESPACE_HTTP;
use crate::ParsedUrl;
use crate::Url;
use reactive_graph_graph::entity_ty;
use reactive_graph_model_result::ResultObject;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;

entity_ty!(ENTITY_TYPE_JSON_RPC, NAMESPACE_HTTP, ENTITY_TYPE_NAME_JSON_RPC, "json_rpc");

entity_model!(JsonRpc);
impl ComponentJsonRpc for JsonRpc {}
impl Url for JsonRpc {}
impl ParsedUrl for JsonRpc {}
impl Action for JsonRpc {}
impl ResultObject for JsonRpc {}
