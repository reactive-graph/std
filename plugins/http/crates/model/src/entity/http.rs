use crate::NAMESPACE_HTTP;
use crate::ParsedUrl;
use crate::Request;
use crate::Response;
use crate::Url;
use reactive_graph_graph::entity_ty;
use reactive_graph_model_result::ResultObject;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;

entity_ty!(ENTITY_TYPE_HTTP, NAMESPACE_HTTP, ENTITY_TYPE_NAME_HTTP, "http");

entity_model!(Http);
impl Request for Http {}
impl Response for Http {}
impl Url for Http {}
impl ParsedUrl for Http {}
impl Action for Http {}
impl ResultObject for Http {}
