use std::env;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::behaviour::entity::entity_behaviour_provider::RandomEntityBehaviourProviderImpl;
use crate::di::*;
use crate::plugins::entity_behaviour_provider;
use crate::plugins::entity_type_provider;
use crate::plugins::plugin::PluginMetadata;
use crate::plugins::plugin::PluginMetadataError;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::plugin_metadata;
use crate::plugins::EntityBehaviourProvider;
use crate::plugins::EntityBehaviourProviderError;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginContextInitializationError;
use crate::provider::RandomEntityTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait RandomPlugin: Plugin + Send + Sync {}

#[module]
pub struct RandomPluginImpl {
    entity_type_provider: Wrc<RandomEntityTypeProviderImpl>,
    entity_behaviour_provider: Wrc<RandomEntityBehaviourProviderImpl>,

    context: PluginContextContainer,
}

impl RandomPluginImpl {}

interfaces!(RandomPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl RandomPlugin for RandomPluginImpl {}

impl Plugin for RandomPluginImpl {
    fn metadata(&self) -> Result<PluginMetadata, PluginMetadataError> {
        plugin_metadata!()
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context.clone());
        Ok(())
    }

    fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
        entity_type_provider!(self.entity_type_provider)
    }

    fn get_entity_behaviour_provider(&self) -> Result<Option<Arc<dyn EntityBehaviourProvider>>, EntityBehaviourProviderError> {
        entity_behaviour_provider!(self.entity_behaviour_provider)
    }
}
