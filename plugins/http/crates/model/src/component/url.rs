use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_HTTP;

properties!(UrlProperties, (URL, "url", ""));

component_ty!(COMPONENT_URL, NAMESPACE_HTTP, COMPONENT_NAME_URL, "url");

component_model!(
    Url,
    set url string,
);
