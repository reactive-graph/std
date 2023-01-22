use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_result::ResultObject;
use crate::model_trigger::Action;
use crate::Request;
use crate::Response;
use crate::Url;
use crate::NAMESPACE_HTTP;

entity_ty!(ENTITY_TYPE_HTTP, NAMESPACE_HTTP, ENTITY_TYPE_NAME_HTTP, "http");

entity_model!(Http);
impl Request for Http {}
impl Response for Http {}
impl Url for Http {}
impl Action for Http {}
impl ResultObject for Http {}
