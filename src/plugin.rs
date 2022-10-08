use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::behaviour::entity::entity_behaviour_provider::LogicalEntityBehaviourProviderImpl;
use crate::di::*;
use crate::plugins::component_provider;
use crate::plugins::entity_behaviour_provider;
use crate::plugins::entity_type_provider;
use crate::plugins::flow_type_provider;
use crate::plugins::plugin::PluginMetadata;
use crate::plugins::plugin::PluginMetadataError;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::plugin_metadata;
use crate::plugins::ComponentProvider;
use crate::plugins::ComponentProviderError;
use crate::plugins::EntityBehaviourProvider;
use crate::plugins::EntityBehaviourProviderError;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::FlowTypeProvider;
use crate::plugins::FlowTypeProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginContextInitializationError;
use crate::provider::LogicalComponentProviderImpl;
use crate::provider::LogicalEntityTypeProviderImpl;
use crate::provider::LogicalFlowTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait LogicalPlugin: Plugin + Send + Sync {}

#[module]
pub struct LogicalPluginImpl {
    component_provider: Wrc<LogicalComponentProviderImpl>,
    entity_type_provider: Wrc<LogicalEntityTypeProviderImpl>,
    entity_behaviour_provider: Wrc<LogicalEntityBehaviourProviderImpl>,
    flow_type_provider: Wrc<LogicalFlowTypeProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(LogicalPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl LogicalPlugin for LogicalPluginImpl {}

impl Plugin for LogicalPluginImpl {
    fn metadata(&self) -> Result<PluginMetadata, PluginMetadataError> {
        plugin_metadata!("inexor-rgf-plugin-base")
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context);
        Ok(())
    }

    fn get_component_provider(&self) -> Result<Option<Arc<dyn ComponentProvider>>, ComponentProviderError> {
        component_provider!(self.component_provider)
    }

    fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
        entity_type_provider!(self.entity_type_provider)
    }

    fn get_entity_behaviour_provider(&self) -> Result<Option<Arc<dyn EntityBehaviourProvider>>, EntityBehaviourProviderError> {
        entity_behaviour_provider!(self.entity_behaviour_provider)
    }

    fn get_flow_type_provider(&self) -> Result<Option<Arc<dyn FlowTypeProvider>>, FlowTypeProviderError> {
        flow_type_provider!(self.flow_type_provider)
    }
}
