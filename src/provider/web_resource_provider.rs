use crate::di::*;
use async_trait::async_trait;
use http::header::CONTENT_TYPE;
use http::{Request, Response, Result, StatusCode};
use inexor_rgf_core_plugins::HttpBody;
use inexor_rgf_core_plugins::WebResourceProvider;
use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "./web/dist/bundle"]
struct GraphQlSchemaVisualizationWebResourceAsset;

#[async_trait]
pub trait GraphQlSchemaVisualizationWebResourceProvider: WebResourceProvider + Send + Sync {}

#[derive(Clone)]
pub struct GraphQlSchemaVisualizationWebResourceProviderImpl {}

interfaces!(GraphQlSchemaVisualizationWebResourceProviderImpl: dyn WebResourceProvider);

#[component]
impl GraphQlSchemaVisualizationWebResourceProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl GraphQlSchemaVisualizationWebResourceProvider
    for GraphQlSchemaVisualizationWebResourceProviderImpl
{
}

impl WebResourceProvider for GraphQlSchemaVisualizationWebResourceProviderImpl {
    fn get_base_path(&self) -> String {
        String::from("graphql-schema-visualization")
    }

    fn handle_web_resource(
        &self,
        path: String,
        _request: Request<HttpBody>,
    ) -> Result<Response<HttpBody>> {
        let path = match path.as_str() {
            "" => String::from("query.html"),
            "query" => format!("{}.html", path),
            "query/" => String::from("query.html"),
            "mutation" => format!("{}.html", path),
            "mutation/" => String::from("mutation.html"),
            "subscription" => format!("{}.html", path),
            "subscription/" => String::from("subscription.html"),
            _ => path,
        };
        let asset = GraphQlSchemaVisualizationWebResourceAsset::get(path.as_ref());
        match asset {
            Some(asset) => {
                // let x = asset.data;
                let body: HttpBody = match asset.data {
                    Cow::Borrowed(bytes) => HttpBody::Binary(bytes.to_vec()),
                    Cow::Owned(bytes) => HttpBody::Binary(bytes.to_vec()),
                };
                let mime_type = from_path(path.as_str()).first_or_octet_stream();
                Response::builder()
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, mime_type.to_string())
                    .body(body)
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(HttpBody::None),
        }
    }
}
