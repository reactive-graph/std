use inexor_rgf_plugin_api::prelude::plugin::*;
use inexor_rgf_plugin_api::prelude::providers::*;

export_plugin!({
    "dependencies": [
        { "name": "inexor-rgf-plugin-base", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait TaxonomyPlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct TaxonomyPluginImpl {
    component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,

    #[component(default = "component_provider_registry")]
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,

    entity_types_provider: Arc<dyn TypeProvider<EntityTypes> + Send + Sync>,

    #[component(default = "entity_types_provider_registry")]
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,

    relation_types_provider: Arc<dyn TypeProvider<RelationTypes> + Send + Sync>,

    #[component(default = "relation_types_provider_registry")]
    relation_type_provider_registry: Arc<dyn RelationTypeProviderRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for TaxonomyPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;
        self.relation_type_provider_registry
            .register_provider(self.relation_types_provider.clone())
            .await;
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.relation_type_provider_registry
            .unregister_provider(self.relation_types_provider.id())
            .await;
        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}
