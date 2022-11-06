use async_trait::async_trait;
use log::debug;
use log::error;
use rust_embed::RustEmbed;

use crate::di::*;
use crate::model::component::Component;
use crate::plugins::embedded_asset_provider_impl;
use crate::plugins::ComponentProvider;

#[derive(RustEmbed)]
#[folder = "./assets/types/components"]
struct TaxonomyComponentAsset;

#[async_trait]
pub trait TaxonomyComponentProvider: ComponentProvider + Send + Sync {}

#[derive(Clone)]
pub struct TaxonomyComponentProviderImpl {}

interfaces!(TaxonomyComponentProviderImpl: dyn ComponentProvider);

#[component]
impl TaxonomyComponentProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl TaxonomyComponentProvider for TaxonomyComponentProviderImpl {}

impl ComponentProvider for TaxonomyComponentProviderImpl {
    fn get_components(&self) -> Vec<Component> {
        embedded_asset_provider_impl!(TaxonomyComponentAsset, Component)
    }
}
