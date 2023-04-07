use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::info;

use crate::config::GraphQLServerConfig;
use crate::di::*;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::web_resource_provider;
use crate::plugins::Plugin;
use crate::plugins::PluginActivationError;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::plugins::WebResourceProvider;
use crate::plugins::WebResourceProviderError;
use crate::provider::GraphQlSchemaVisualizationWebResourceProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait GraphQlSchemaVisualizationPlugin: Plugin + Send + Sync {}

#[module]
pub struct GraphQlSchemaVisualizationPluginImpl {
    web_resource_provider: Wrc<GraphQlSchemaVisualizationWebResourceProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(GraphQlSchemaVisualizationPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl GraphQlSchemaVisualizationPlugin for GraphQlSchemaVisualizationPluginImpl {}

impl GraphQlSchemaVisualizationPluginImpl {
    fn get_graphql_server_config(&self) -> GraphQLServerConfig {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            return context.get_config_manager().get_graphql_server_config();
        }
        GraphQLServerConfig::default()
    }
}

#[async_trait]
impl Plugin for GraphQlSchemaVisualizationPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        let config = self.get_graphql_server_config();
        let context_path = self.web_resource_provider.get_context_path().clone();
        let url = config.url();
        info!(
            r"
    {url}/{context_path}/?rootType=Query&hideRoot=false&endpoint=/graphql
    {url}/{context_path}/?rootType=Mutation&hideRoot=false&endpoint=/graphql
    {url}/{context_path}/?rootType=Subscription&hideRoot=false&endpoint=/graphql
    {url}/{context_path}/?rootType=Query&hideRoot=false&endpoint=/dynamic_graph
    {url}/{context_path}/?rootType=Mutation&hideRoot=false&endpoint=/dynamic_graph
    {url}/{context_path}/?rootType=Subscription&hideRoot=false&endpoint=/dynamic_graph
        "
        );
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
