use crate::di::*;
use async_trait::async_trait;
use log::error;
use rust_embed::RustEmbed;

use crate::model::Component;
use crate::plugins::ComponentProvider;

#[derive(RustEmbed)]
#[folder = "./assets/types/components"]
struct BinaryComponentAsset;

#[async_trait]
pub trait BinaryComponentProvider: ComponentProvider + Send + Sync {}

#[derive(Clone)]
pub struct BinaryComponentProviderImpl {}

interfaces!(BinaryComponentProviderImpl: dyn ComponentProvider);

#[component]
impl BinaryComponentProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl BinaryComponentProvider for BinaryComponentProviderImpl {}

impl ComponentProvider for BinaryComponentProviderImpl {
    fn get_components(&self) -> Vec<Component> {
        let mut components = Vec::new();
        for file in BinaryComponentAsset::iter() {
            let filename = file.as_ref();
            let asset = BinaryComponentAsset::get(filename).unwrap();
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
