use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_result::ResultObject;
use crate::model_runtime::Action;
use crate::ComponentJsonRpc;
use crate::ParsedUrl;
use crate::Url;
use crate::NAMESPACE_HTTP;

entity_ty!(ENTITY_TYPE_JSON_RPC, NAMESPACE_HTTP, ENTITY_TYPE_NAME_JSON_RPC, "json_rpc");

entity_model!(JsonRpc);
impl ComponentJsonRpc for JsonRpc {}
impl Url for JsonRpc {}
impl ParsedUrl for JsonRpc {}
impl Action for JsonRpc {}
impl ResultObject for JsonRpc {}
