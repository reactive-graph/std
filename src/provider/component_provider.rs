use async_trait::async_trait;
use log::{debug, error};
use rust_embed::RustEmbed;
use waiter_di::*;

use crate::plugins::ComponentProvider;

use crate::model::component::Component;

#[derive(RustEmbed)]
#[folder = "./assets/types/components"]
struct MetaDataComponentAsset;

#[async_trait]
pub trait MetaDataComponentProvider: ComponentProvider + Send + Sync {}

#[derive(Clone)]
pub struct MetaDataComponentProviderImpl {}

interfaces!(MetaDataComponentProviderImpl: dyn ComponentProvider);

#[component]
impl MetaDataComponentProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl MetaDataComponentProvider for MetaDataComponentProviderImpl {}

impl ComponentProvider for MetaDataComponentProviderImpl {
    fn get_components(&self) -> Vec<Component> {
        let mut components = Vec::new();
        for file in MetaDataComponentAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading component from resource {}", filename);
            let asset = MetaDataComponentAsset::get(filename).unwrap();
            let json_str = std::str::from_utf8(asset.as_ref());
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
