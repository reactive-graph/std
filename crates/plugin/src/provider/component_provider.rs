use async_trait::async_trait;
use log::debug;
use log::error;
use rust_embed::RustEmbed;

use crate::di::*;
use crate::model::Component;
use crate::plugins::embedded_asset_provider_impl;
use crate::plugins::ComponentProvider;

#[derive(RustEmbed)]
#[folder = "../../assets/types/components"]
struct BaseComponentAsset;

#[async_trait]
pub trait BaseComponentProvider: ComponentProvider + Send + Sync {}

#[derive(Clone)]
pub struct BaseComponentProviderImpl {}

interfaces!(BaseComponentProviderImpl: dyn ComponentProvider);

#[component]
impl BaseComponentProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl BaseComponentProvider for BaseComponentProviderImpl {}
impl ComponentProvider for BaseComponentProviderImpl {
    fn get_components(&self) -> Vec<Component> {
        embedded_asset_provider_impl!(BaseComponentAsset, Component)
    }
}
