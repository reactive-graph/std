use crate::di::*;
use async_trait::async_trait;
use log::{debug, error};
use rust_embed::RustEmbed;

use crate::plugins::ComponentProvider;

use crate::model::component::Component;

#[derive(RustEmbed)]
#[folder = "./assets/types/components"]
struct ValueComponentAsset;

#[async_trait]
pub trait ValueComponentProvider: ComponentProvider + Send + Sync {}

#[derive(Clone)]
pub struct ValueComponentProviderImpl {}

interfaces!(ValueComponentProviderImpl: dyn ComponentProvider);

#[component]
impl ValueComponentProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl ValueComponentProvider for ValueComponentProviderImpl {}

impl ComponentProvider for ValueComponentProviderImpl {
    fn get_components(&self) -> Vec<Component> {
        let mut components = Vec::new();
        for file in ValueComponentAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading component from resource {}", filename);
            let asset = ValueComponentAsset::get(filename).unwrap();
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
