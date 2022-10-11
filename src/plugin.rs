use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::behaviour::component::component_behaviour_provider::ValueComponentBehaviourProviderImpl;
use crate::di::*;
use crate::plugins::component_behaviour_provider;
use crate::plugins::component_provider;
use crate::plugins::entity_type_provider;
use crate::plugins::plugin::PluginMetadata;
use crate::plugins::plugin::PluginMetadataError;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::plugin_metadata;
use crate::plugins::ComponentBehaviourProvider;
use crate::plugins::ComponentBehaviourProviderError;
use crate::plugins::ComponentProvider;
use crate::plugins::ComponentProviderError;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginContextInitializationError;
use crate::provider::ValueComponentProviderImpl;
use crate::provider::ValueEntityTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait ValuePlugin: Plugin + Send + Sync {}

#[module]
pub struct ValuePluginImpl {
    component_provider: Wrc<ValueComponentProviderImpl>,
    entity_type_provider: Wrc<ValueEntityTypeProviderImpl>,
    component_behaviour_provider: Wrc<ValueComponentBehaviourProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(ValuePluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl ValuePlugin for ValuePluginImpl {}

impl Plugin for ValuePluginImpl {
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

    fn get_component_behaviour_provider(&self) -> Result<Option<Arc<dyn ComponentBehaviourProvider>>, ComponentBehaviourProviderError> {
        component_behaviour_provider!(self.component_behaviour_provider)
    }
}
