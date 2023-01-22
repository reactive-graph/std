use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_HTTP;

properties!(UrlProperties, (URL, "url", ""));

component_ty!(COMPONENT_URL, NAMESPACE_HTTP, COMPONENT_NAME_URL, "url");

component_model!(
    Url,
    data url string,
);

pub trait ParsedUrl: Url {
    fn parse_url(&self) -> Option<url::Url> {
        self.get_url().and_then(|url| url::Url::parse(&url).ok())
    }
}
