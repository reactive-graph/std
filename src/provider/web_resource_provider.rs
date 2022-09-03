use crate::di::*;
use crate::plugins::HttpBody;
use crate::plugins::WebResourceProvider;
use async_trait::async_trait;
use http::header::CONTENT_TYPE;
use http::Request;
use http::Response;
use http::Result;
use http::StatusCode;
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
            ""
            | "index.html"
            | "graph/query"
            | "graph/query/"
            | "graph/query.html"
            | "graph/query/index.html" => String::from("graph/query.html"),
            "graph/mutation"
            | "graph/mutation/"
            | "graph/mutation.html"
            | "graph/mutation/index.html" => String::from("graph/mutation.html"),
            "graph/subscription"
            | "graph/subscription/"
            | "graph/subscription.html"
            | "graph/subscription/index.html" => String::from("graph/subscription.html"),
            "dynamic-graph/query"
            | "dynamic-graph/query/"
            | "dynamic-graph/query.html"
            | "dynamic-graph/query/index.html" => String::from("dynamic-graph/query.html"),
            "dynamic-graph/mutation"
            | "dynamic-graph/mutation/"
            | "dynamic-graph/mutation.html"
            | "dynamic-graph/mutation/index.html" => String::from("dynamic-graph/mutation.html"),
            "dynamic-graph/subscription"
            | "dynamic-graph/subscription/"
            | "dynamic-graph/subscription.html"
            | "dynamic-graph/subscription/index.html" => {
                String::from("dynamic-graph/subscription.html")
            }
            _ => path,
        };
        let asset = GraphQlSchemaVisualizationWebResourceAsset::get(path.as_ref());
        match asset {
            Some(asset) => {
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
