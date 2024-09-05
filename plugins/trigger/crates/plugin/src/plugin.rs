use reactive_graph_plugin_api::prelude::plugin::*;
use reactive_graph_plugin_api::ComponentProviderRegistry;

export_plugin!({
  "dependencies": [
    {
      "name": "reactive-graph-plugin-base",
      "version": ">=0.10.0, <0.11.0"
    }
  ]
});

#[injectable]
pub trait TriggerPlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct TriggerPluginImpl {
    component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,

    #[component(default = "component_provider_registry")]
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for TriggerPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}
