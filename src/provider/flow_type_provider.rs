use async_trait::async_trait;
use log::debug;
use log::error;
use rust_embed::RustEmbed;

use crate::di::*;
use crate::model::FlowType;
use crate::model::FlowTypeDao;
use crate::plugins::embedded_asset_provider_impl;
use crate::plugins::FlowTypeProvider;

#[derive(RustEmbed)]
#[folder = "./assets/types/flows"]
struct BaseFlowTypeAsset;

#[async_trait]
pub trait BaseFlowTypeProvider: FlowTypeProvider + Send + Sync {}

#[derive(Clone)]
pub struct BaseFlowTypeProviderImpl {}

interfaces!(BaseFlowTypeProviderImpl: dyn FlowTypeProvider);

#[component]
impl BaseFlowTypeProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl BaseFlowTypeProvider for BaseFlowTypeProviderImpl {}

impl FlowTypeProvider for BaseFlowTypeProviderImpl {
    fn get_flow_types(&self) -> Vec<FlowType> {
        embedded_asset_provider_impl!(BaseFlowTypeAsset, FlowTypeDao, FlowType)
    }
}
