use crate::factory::SystemEnvironmentReactiveEntityFactory;
use inexor_rgf_plugin_api::prelude::plugin::*;
use inexor_rgf_plugin_api::prelude::providers::*;

export_plugin!({
    "dependencies": [
        { "name": "inexor-rgf-plugin-base", "version": ">=0.10.0, <0.11.0" },
        { "name": "inexor-rgf-plugin-value", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[derive(Component)]
pub struct SystemEnvironmentPlugin {
    entity_types_provider: Arc<dyn TypeProvider<EntityTypes> + Send + Sync>,

    factory: Arc<SystemEnvironmentReactiveEntityFactory>,

    #[component(default = "entity_types_provider_registry")]
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for SystemEnvironmentPlugin {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;
        if let Err(e) = self.factory.create_entity_instances().await {
            return Err(PluginActivationError::ActivationFailed(format!(
                "Failed to create entities which represents the system environment variables: {e}"
            )));
        }
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        Ok(())
    }
}
