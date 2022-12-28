use async_trait::async_trait;
use inexor_rgf_core_model::FlowInstance;
use log::debug;
use log::error;
use rust_embed::RustEmbed;

use crate::di::*;
// use crate::model::flow::Flow;
use crate::plugins::FlowInstanceProvider;

#[derive(RustEmbed)]
#[folder = "./assets/flows"]
struct ConfigFlowInstancesAsset;

#[async_trait]
pub trait ConfigFlowInstanceProvider: FlowInstanceProvider + Send + Sync {}

#[derive(Clone)]
pub struct ConfigFlowInstanceProviderImpl {}

interfaces!(ConfigFlowInstanceProviderImpl: dyn FlowInstanceProvider);

#[component]
impl ConfigFlowInstanceProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl ConfigFlowInstanceProvider for ConfigFlowInstanceProviderImpl {}

impl FlowInstanceProvider for ConfigFlowInstanceProviderImpl {
    fn get_flow_instances(&self) -> Vec<FlowInstance> {
        let mut flow_instances = Vec::new();
        for file in ConfigFlowInstancesAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading flow instances from resource {}", filename);
            let asset = ConfigFlowInstancesAsset::get(filename).unwrap();
            let json_str = std::str::from_utf8(asset.data.as_ref());
            if json_str.is_err() {
                error!("Could not decode UTF-8 {}", filename);
                continue;
            }
            let flow_instance: FlowInstance = match serde_json::from_str(json_str.unwrap()) {
                Ok(flow) => flow,
                Err(err) => {
                    error!("Error in parsing JSON file {}: {}", filename, err);
                    continue;
                }
            };
            flow_instances.push(flow_instance);
        }
        flow_instances
    }
}
