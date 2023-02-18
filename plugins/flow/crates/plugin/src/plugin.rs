use std::sync::Arc;
use std::sync::RwLock;

use crate::di::*;
use crate::plugins::entity_type_provider;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::providers::FlowEntityTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

pub trait FlowPlugin: Plugin + Send + Sync {}

#[module]
pub struct FlowPluginImpl {
    entity_type_provider: Wrc<FlowEntityTypeProviderImpl>,

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

    fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
        entity_type_provider!(self.entity_type_provider)
    }
}
