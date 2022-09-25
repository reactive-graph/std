use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::di::*;
use crate::plugins::component_provider;
use crate::plugins::plugin::PluginMetadata;
use crate::plugins::plugin::PluginMetadataError;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::plugin_metadata;
use crate::plugins::ComponentProvider;
use crate::plugins::ComponentProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginContextInitializationError;
use crate::provider::MetaDataComponentProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<std::sync::Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait MetaDataPlugin: Plugin + Send + Sync {}

#[module]
pub struct MetaDataPluginImpl {
    component_provider: Wrc<MetaDataComponentProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(MetaDataPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl MetaDataPlugin for MetaDataPluginImpl {}

impl Plugin for MetaDataPluginImpl {
    fn metadata(&self) -> Result<PluginMetadata, PluginMetadataError> {
        plugin_metadata!("inexor-rgf-plugin-base")
    }

    fn set_context(
        &self,
        context: Arc<dyn PluginContext>,
    ) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context);
        Ok(())
    }

    fn get_component_provider(
        &self,
    ) -> Result<Option<Arc<dyn ComponentProvider>>, ComponentProviderError> {
        component_provider!(self.component_provider)
    }
}
