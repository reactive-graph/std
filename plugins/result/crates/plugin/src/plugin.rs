use std::sync::Arc;
use std::sync::RwLock;

use crate::di::*;
use crate::plugins::component_provider;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::ComponentProvider;
use crate::plugins::ComponentProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::providers::ResultComponentProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

pub trait ResultPlugin: Plugin + Send + Sync {}

#[module]
pub struct ResultPluginImpl {
    component_provider: Wrc<ResultComponentProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(ResultPluginImpl: dyn Plugin);

#[provides]
impl ResultPlugin for ResultPluginImpl {}

impl Plugin for ResultPluginImpl {
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
}
