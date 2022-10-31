use async_trait::async_trait;
use log::debug;
use log::error;
use rust_embed::RustEmbed;

use crate::di::*;
use crate::model::EntityType;
use crate::plugins::embedded_asset_provider_impl;
use crate::plugins::EntityTypeProvider;

#[derive(RustEmbed)]
#[folder = "./assets/types/entities"]
struct BaseEntityTypeAsset;

#[async_trait]
pub trait BaseEntityTypeProvider: EntityTypeProvider + Send + Sync {}

#[derive(Clone)]
pub struct BaseEntityTypeProviderImpl {}

interfaces!(BaseEntityTypeProviderImpl: dyn EntityTypeProvider);

#[component]
impl BaseEntityTypeProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl BaseEntityTypeProvider for BaseEntityTypeProviderImpl {}

impl EntityTypeProvider for BaseEntityTypeProviderImpl {
    fn get_entity_types(&self) -> Vec<EntityType> {
        embedded_asset_provider_impl!(BaseEntityTypeAsset, EntityType)
    }
}
