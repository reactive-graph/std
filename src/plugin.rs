use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::info;

use crate::di::*;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::web_resource_provider;
use crate::plugins::Plugin;
use crate::plugins::PluginActivationError;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::plugins::WebResourceProvider;
use crate::plugins::WebResourceProviderError;
use crate::provider::GraphQlClientWebResourceProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait GraphQlClientPlugin: Plugin + Send + Sync {}

#[module]
pub struct GraphQlClientPluginImpl {
    web_resource_provider: Wrc<GraphQlClientWebResourceProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(GraphQlClientPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl GraphQlClientPlugin for GraphQlClientPluginImpl {}

impl Plugin for GraphQlClientPluginImpl {
    fn activate(&self) -> Result<(), PluginActivationError> {
        let base_path = self.web_resource_provider.get_base_path().clone();
        info!("\n    http://localhost:31415/{base_path}/graph\n    http://localhost:31415/{base_path}/dynamic-graph");
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context);
        Ok(())
    }

    fn remove_context(&self) -> Result<(), PluginContextDeinitializationError> {
        let mut writer = self.context.0.write().unwrap();
        *writer = None;
        Ok(())
    }

    fn get_web_resource_provider(&self) -> Result<Option<Arc<dyn WebResourceProvider>>, WebResourceProviderError> {
        web_resource_provider!(self.web_resource_provider)
    }
}
