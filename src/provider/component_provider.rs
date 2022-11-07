use async_trait::async_trait;
use log::debug;
use log::error;
use rust_embed::RustEmbed;

use crate::di::*;
use crate::model::Component;
use crate::plugins::embedded_asset_provider_impl;
use crate::plugins::ComponentProvider;

#[derive(RustEmbed)]
#[folder = "./assets/types/components"]
struct MetaDataComponentAsset;

#[async_trait]
pub trait MetaDataComponentProvider: ComponentProvider + Send + Sync {}

#[derive(Clone)]
pub struct MetaDataComponentProviderImpl {}

interfaces!(MetaDataComponentProviderImpl: dyn ComponentProvider);

#[component]
impl MetaDataComponentProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl MetaDataComponentProvider for MetaDataComponentProviderImpl {}

impl ComponentProvider for MetaDataComponentProviderImpl {
    fn get_components(&self) -> Vec<Component> {
        embedded_asset_provider_impl!(MetaDataComponentAsset, Component)
    }
}
