use async_trait::async_trait;
use log::debug;
use log::error;
use rust_embed::RustEmbed;

use crate::di::*;
use crate::model::EntityType;
use crate::plugins::embedded_asset_provider_impl;
use crate::plugins::EntityTypeProvider;

#[derive(RustEmbed)]
#[folder = "../../assets/types/entities"]
struct ValueEntityTypeAsset;

#[async_trait]
pub trait ValueEntityTypeProvider: EntityTypeProvider + Send + Sync {}

#[derive(Clone)]
pub struct ValueEntityTypeProviderImpl {}

interfaces!(ValueEntityTypeProviderImpl: dyn EntityTypeProvider);

#[component]
impl ValueEntityTypeProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl ValueEntityTypeProvider for ValueEntityTypeProviderImpl {}

impl EntityTypeProvider for ValueEntityTypeProviderImpl {
    fn get_entity_types(&self) -> Vec<EntityType> {
        embedded_asset_provider_impl!(ValueEntityTypeAsset, EntityType)
    }
}
