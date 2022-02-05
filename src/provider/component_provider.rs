use crate::di::*;
use async_trait::async_trait;
use log::{debug, error};
use rust_embed::RustEmbed;

use crate::model::component::Component;
use crate::plugins::ComponentProvider;

#[derive(RustEmbed)]
#[folder = "./assets/types/components"]
struct ConnectorComponentAsset;

#[async_trait]
pub trait ConnectorComponentProvider: ComponentProvider + Send + Sync {}

#[derive(Clone)]
pub struct ConnectorComponentProviderImpl {}

interfaces!(ConnectorComponentProviderImpl: dyn ComponentProvider);

#[component]
impl ConnectorComponentProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl ConnectorComponentProvider for ConnectorComponentProviderImpl {}

impl ComponentProvider for ConnectorComponentProviderImpl {
    fn get_components(&self) -> Vec<Component> {
        let mut components = Vec::new();
        for file in ConnectorComponentAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading component from resource {}", filename);
            let asset = ConnectorComponentAsset::get(filename).unwrap();
            let json_str = std::str::from_utf8(asset.data.as_ref());
            if json_str.is_err() {
                error!("Could not decode UTF-8 {}", filename);
                continue;
            }
            let component: Component = match serde_json::from_str(json_str.unwrap()) {
                Result::Ok(component) => component,
                Result::Err(err) => {
                    error!("Error in parsing JSON file {}: {}", filename, err);
                    continue;
                }
            };
            components.push(component);
        }
        components
    }
}
