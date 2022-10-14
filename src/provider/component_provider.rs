use log::debug;
use log::error;
use rust_embed::RustEmbed;

use crate::di::*;
use crate::model::Component;
use crate::plugins::ComponentProvider;

#[derive(RustEmbed)]
#[folder = "./assets/types/components"]
struct ArithmeticComponentAsset;

pub trait ArithmeticComponentProvider: ComponentProvider + Send + Sync {}

pub struct ArithmeticComponentProviderImpl {}

interfaces!(ArithmeticComponentProviderImpl: dyn ComponentProvider);

#[component]
impl ArithmeticComponentProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[provides]
impl ArithmeticComponentProvider for ArithmeticComponentProviderImpl {}

impl ComponentProvider for ArithmeticComponentProviderImpl {
    fn get_components(&self) -> Vec<Component> {
        let mut components = Vec::new();
        for file in ArithmeticComponentAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading component from resource {}", filename);
            let asset = ArithmeticComponentAsset::get(filename).unwrap();
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
