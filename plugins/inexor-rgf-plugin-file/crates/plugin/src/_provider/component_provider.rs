use crate::di::*;
use async_trait::async_trait;
use log::debug;
use log::error;
use rust_embed::RustEmbed;

use crate::plugins::ComponentProvider;

use crate::model::component::Component;

#[derive(RustEmbed)]
#[folder = "./assets/types/components"]
struct FileComponentAsset;

#[async_trait]
pub trait FileComponentProvider: ComponentProvider + Send + Sync {}

#[derive(Clone)]
pub struct FileComponentProviderImpl {}

interfaces!(FileComponentProviderImpl: dyn ComponentProvider);

#[component]
impl FileComponentProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl FileComponentProvider for FileComponentProviderImpl {}

impl ComponentProvider for FileComponentProviderImpl {
    fn get_components(&self) -> Vec<Component> {
        let mut components = Vec::new();
        for file in FileComponentAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading component from resource {}", filename);
            let asset = FileComponentAsset::get(filename).unwrap();
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
