use std::sync::Arc;
use std::sync::RwLock;

use crate::di::*;
use crate::plugins::component_provider;
use crate::plugins::entity_type_provider;
use crate::plugins::flow_type_provider;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::ComponentProvider;
use crate::plugins::ComponentProviderError;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::FlowTypeProvider;
use crate::plugins::FlowTypeProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::providers::FlowComponentProviderImpl;
use crate::providers::FlowEntityTypeProviderImpl;
use crate::providers::FlowFlowTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

pub trait FlowPlugin: Plugin + Send + Sync {}

#[module]
pub struct FlowPluginImpl {
    component_provider: Wrc<FlowComponentProviderImpl>,
    entity_type_provider: Wrc<FlowEntityTypeProviderImpl>,
    flow_type_provider: Wrc<FlowFlowTypeProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(FlowPluginImpl: dyn Plugin);

#[provides]
impl FlowPlugin for FlowPluginImpl {}

impl Plugin for FlowPluginImpl {
    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context);
        Ok(())
    }

    fn remove_context(&self) -> Result<(), PluginContextDeinitializationError> {
        let mut writer = self.context.0.write().unwrap();
        *writer = None;
        Ok(())
    }

    fn get_component_provider(&self) -> Result<Option<Arc<dyn ComponentProvider>>, ComponentProviderError> {
        component_provider!(self.component_provider)
    }

    fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
        entity_type_provider!(self.entity_type_provider)
    }

    fn get_flow_type_provider(&self) -> Result<Option<Arc<dyn FlowTypeProvider>>, FlowTypeProviderError> {
        flow_type_provider!(self.flow_type_provider)
    }
}
