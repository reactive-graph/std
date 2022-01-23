use async_trait::async_trait;
use log::{debug, error};
use rust_embed::RustEmbed;
use waiter_di::*;

use crate::model::EntityType;
use crate::plugins::EntityTypeProvider;

#[derive(RustEmbed)]
#[folder = "./assets/types/entities"]
struct ArithmeticEntityTypeAsset;

#[async_trait]
pub trait ArithmeticEntityTypeProvider: EntityTypeProvider + Send + Sync {}

#[derive(Clone)]
pub struct ArithmeticEntityTypeProviderImpl {}

interfaces!(ArithmeticEntityTypeProviderImpl: dyn EntityTypeProvider);

#[component]
impl ArithmeticEntityTypeProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl ArithmeticEntityTypeProvider for ArithmeticEntityTypeProviderImpl {}

impl EntityTypeProvider for ArithmeticEntityTypeProviderImpl {
    fn get_entity_types(&self) -> Vec<EntityType> {
        let mut entity_types = Vec::new();
        for file in ArithmeticEntityTypeAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading entity_type from resource {}", filename);
            let asset = ArithmeticEntityTypeAsset::get(filename).unwrap();
            let json_str = std::str::from_utf8(asset.data.as_ref());
            if json_str.is_err() {
                error!("Could not decode UTF-8 {}", filename);
                continue;
            }
            let entity_type: EntityType = match serde_json::from_str(json_str.unwrap()) {
                Result::Ok(entity_type) => entity_type,
                Result::Err(err) => {
                    error!("Error in parsing JSON file {}: {}", filename, err);
                    continue;
                }
            };
            entity_types.push(entity_type);
        }
        entity_types
    }
}
