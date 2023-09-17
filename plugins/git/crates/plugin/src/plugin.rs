use inexor_rgf_plugin_api::prelude::plugin::*;
use inexor_rgf_plugin_api::prelude::providers::*;
use inexor_rgf_plugin_api::EntityComponentBehaviourRegistry;

use crate::behaviour::component::repository::RepositoryFactory;
use inexor_rgf_model_git::BEHAVIOUR_REPOSITORY;
use inexor_rgf_model_git::COMPONENT_BEHAVIOUR_REPOSITORY;

export_plugin!({
    "dependencies": [
        { "name": "inexor-rgf-plugin-base", "version": ">=0.10.0, <0.11.0" },
        { "name": "inexor-rgf-plugin-file", "version": ">=0.10.0, <0.11.0" },
        { "name": "inexor-rgf-plugin-http", "version": ">=0.10.0, <0.11.0" },
        { "name": "inexor-rgf-plugin-trigger", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait ValuePlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct ValuePluginImpl {
    component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,

    #[component(default = "component_provider_registry")]
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,

    entity_types_provider: Arc<dyn TypeProvider<EntityTypes> + Send + Sync>,

    #[component(default = "entity_types_provider_registry")]
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,

    #[component(default = "entity_component_behaviour_registry")]
    entity_component_behaviour_registry: Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for ValuePluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;

        // Git Repository
        let factory = Arc::new(RepositoryFactory::new(BEHAVIOUR_REPOSITORY.clone()));
        self.entity_component_behaviour_registry
            .register(COMPONENT_BEHAVIOUR_REPOSITORY.clone(), factory)
            .await;

        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_REPOSITORY).await;
        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}
