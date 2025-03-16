use std::borrow::Cow;
use std::sync::LazyLock;

use async_trait::async_trait;
use http::Response;
use http::StatusCode;
use http::header::CONTENT_TYPE;
use http::request::Request;
use mime_guess::from_path;
use reactive_graph_plugin_api::HttpBody;
use reactive_graph_plugin_api::WebResourceProvider;
use reactive_graph_plugin_api::prelude::plugin::*;
use uuid::Uuid;

const CONTEXT_PATH: &str = "graphql-schema-visualization";

static ID: LazyLock<Uuid> = LazyLock::new(Uuid::new_v4);

mod embedded_resources {
    use rust_embed::RustEmbed;

    #[derive(RustEmbed)]
    #[folder = "./web/dist"]
    pub(crate) struct GraphQlSchemaVisualizationWebResourceAsset;
}

#[derive(Component)]
pub struct GraphQlSchemaVisualizationWebResourceProvider {}

#[async_trait]
#[component_alias]
impl WebResourceProvider for GraphQlSchemaVisualizationWebResourceProvider {
    fn id(&self) -> Uuid {
        ID.clone()
    }

    fn get_context_path(&self) -> String {
        CONTEXT_PATH.to_string()
    }

    async fn handle_web_resource(&self, path: String, _request: Request<HttpBody>) -> http::Result<Response<HttpBody>> {
        let path = match path.as_str() {
            "" | "index.html" => String::from("index.html"),
            _ => path,
        };
        let asset = embedded_resources::GraphQlSchemaVisualizationWebResourceAsset::get(path.as_ref());
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
