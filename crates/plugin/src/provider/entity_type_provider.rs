use async_trait::async_trait;
use log::debug;
use log::error;
use rust_embed::RustEmbed;

use crate::di::*;
use crate::model::entity_type::EntityType;
use crate::plugins::embedded_asset_provider_impl;
use crate::plugins::EntityTypeProvider;

#[derive(RustEmbed)]
#[folder = "../../assets/types/entities"]
struct TaxonomyEntityTypeAsset;

#[async_trait]
pub trait TaxonomyEntityTypeProvider: EntityTypeProvider + Send + Sync {}

#[derive(Clone)]
pub struct TaxonomyEntityTypeProviderImpl {}

interfaces!(TaxonomyEntityTypeProviderImpl: dyn EntityTypeProvider);

#[component]
impl TaxonomyEntityTypeProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl TaxonomyEntityTypeProvider for TaxonomyEntityTypeProviderImpl {}

impl EntityTypeProvider for TaxonomyEntityTypeProviderImpl {
    fn get_entity_types(&self) -> Vec<EntityType> {
        embedded_asset_provider_impl!(TaxonomyEntityTypeAsset, EntityType)
    }
}
