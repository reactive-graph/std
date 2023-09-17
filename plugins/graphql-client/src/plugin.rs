use inexor_rgf_plugin_api::prelude::plugin::*;
use inexor_rgf_plugin_api::ConfigManager;
use inexor_rgf_plugin_api::WebResourceManager;
use log::info;

export_plugin!();

#[derive(Component)]
pub struct GraphQlClientPlugin {
    web_resource_provider: Arc<dyn WebResourceProvider + Send + Sync>,

    #[component(default = "web_resource_manager")]
    web_resource_manager: Arc<dyn WebResourceManager + Send + Sync>,

    #[component(default = "config_manager")]
    config_manager: Arc<dyn ConfigManager + Send + Sync>,
}

impl GraphQlClientPlugin {
    fn log_urls(&self) {
        let config = self.config_manager.get_graphql_server_config();
        let context_path = self.web_resource_provider.get_context_path().clone();
        let url = config.url();
        info!("\n    {url}/{context_path}/graph\n    {url}/{context_path}/dynamic-graph",);
    }
}

#[async_trait]
#[component_alias]
impl Plugin for GraphQlClientPlugin {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.web_resource_manager.register_provider(self.web_resource_provider.clone()).await;
        self.log_urls();
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.web_resource_manager.unregister_provider(self.web_resource_provider.id()).await;
        Ok(())
    }
}
