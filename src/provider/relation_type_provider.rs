use async_trait::async_trait;
use log::debug;
use log::error;
use rust_embed::RustEmbed;

use crate::di::*;
use crate::model::RelationType;
use crate::plugins::embedded_asset_provider_impl;
use crate::plugins::RelationTypeProvider;

#[derive(RustEmbed)]
#[folder = "./assets/types/relations"]
struct TaxonomyRelationTypeAsset;

#[async_trait]
pub trait TaxonomyRelationTypeProvider: RelationTypeProvider + Send + Sync {}

#[derive(Clone)]
pub struct TaxonomyRelationTypeProviderImpl {}

interfaces!(TaxonomyRelationTypeProviderImpl: dyn RelationTypeProvider);

#[component]
impl TaxonomyRelationTypeProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl TaxonomyRelationTypeProvider for TaxonomyRelationTypeProviderImpl {}

impl RelationTypeProvider for TaxonomyRelationTypeProviderImpl {
    fn get_relation_types(&self) -> Vec<RelationType> {
        embedded_asset_provider_impl!(TaxonomyRelationTypeAsset, RelationType)
    }
}
