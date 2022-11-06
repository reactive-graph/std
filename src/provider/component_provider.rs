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
struct ValueComponentAsset;

#[async_trait]
pub trait ValueComponentProvider: ComponentProvider + Send + Sync {}

#[derive(Clone)]
pub struct ValueComponentProviderImpl {}

interfaces!(ValueComponentProviderImpl: dyn ComponentProvider);

#[component]
impl ValueComponentProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl ValueComponentProvider for ValueComponentProviderImpl {}

impl ComponentProvider for ValueComponentProviderImpl {
    fn get_components(&self) -> Vec<Component> {
        embedded_asset_provider_impl!(ValueComponentAsset, Component)
    }
}
