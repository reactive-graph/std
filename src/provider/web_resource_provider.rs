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
struct GraphQlClientWebResourceAsset;

#[async_trait]
pub trait GraphQlClientWebResourceProvider: WebResourceProvider + Send + Sync {}

#[derive(Clone)]
pub struct GraphQlClientWebResourceProviderImpl {}

interfaces!(GraphQlClientWebResourceProviderImpl: dyn WebResourceProvider);

#[component]
impl GraphQlClientWebResourceProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl GraphQlClientWebResourceProvider for GraphQlClientWebResourceProviderImpl {}

impl WebResourceProvider for GraphQlClientWebResourceProviderImpl {
    fn get_base_path(&self) -> String {
        String::from("graphql-client")
    }

    fn handle_web_resource(&self, path: String, _request: Request<HttpBody>) -> Result<Response<HttpBody>> {
        let path = match path.as_str() {
            "" | "index.html" | "graph.html" | "graph" => String::from("graph.html"),
            "dynamic-graph.html" | "dynamic-graph" => String::from("dynamic-graph.html"),
            _ => path,
        };
        let asset = GraphQlClientWebResourceAsset::get(path.as_ref());
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
            None => Response::builder().status(StatusCode::NOT_FOUND).body(HttpBody::None),
        }
    }
}
