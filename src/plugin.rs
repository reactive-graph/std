use std::env;
use std::sync::{Arc, RwLock};

use async_trait::async_trait;

use crate::behaviour::entity::entity_behaviour_provider::StringEntityBehaviourProviderImpl;
use crate::di::*;
use crate::plugins::component_provider;
use crate::plugins::entity_behaviour_provider;
use crate::plugins::entity_type_provider;
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
use crate::plugins::Plugin;
use crate::plugins::PluginContextInitializationError;
use crate::provider::{StringComponentProviderImpl, StringEntityTypeProviderImpl};

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<std::sync::Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait StringPlugin: Plugin + Send + Sync {}

#[module]
pub struct StringPluginImpl {
    component_provider: Wrc<StringComponentProviderImpl>,
    entity_type_provider: Wrc<StringEntityTypeProviderImpl>,
    entity_behaviour_provider: Wrc<StringEntityBehaviourProviderImpl>,

    context: PluginContextContainer,
}

impl StringPluginImpl {}

interfaces!(StringPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl StringPlugin for StringPluginImpl {}

impl Plugin for StringPluginImpl {
    fn metadata(&self) -> Result<PluginMetadata, PluginMetadataError> {
        plugin_metadata!()
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context.clone());
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
}
