use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_result::ResultObject;
use crate::model_trigger::Action;
use crate::ComponentHttp;
use crate::NAMESPACE_HTTP;

entity_ty!(ENTITY_TYPE_HTTP, NAMESPACE_HTTP, ENTITY_TYPE_NAME_HTTP, "http");

entity_model!(Http);
impl ComponentHttp for Http {}
impl Action for Http {}
impl ResultObject for Http {}
