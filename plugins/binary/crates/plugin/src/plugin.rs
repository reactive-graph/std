use inexor_rgf_plugin_api::prelude::plugin::*;
use inexor_rgf_plugin_api::prelude::providers::*;
use inexor_rgf_plugin_api::EntityComponentBehaviourRegistry;
use inexor_rgf_plugin_api::WebResourceManager;

use crate::behaviour::component::load_binary_data::LoadBinaryDataFactory;
use crate::behaviour::component::save_binary_data::SaveBinaryDataFactory;
use inexor_rgf_model_binary::BEHAVIOUR_LOAD_BINARY_DATA;
use inexor_rgf_model_binary::BEHAVIOUR_SAVE_BINARY_DATA;
use inexor_rgf_model_binary::COMPONENT_BEHAVIOUR_LOAD_BINARY_DATA;
use inexor_rgf_model_binary::COMPONENT_BEHAVIOUR_SAVE_BINARY_DATA;

export_plugin!({
    "dependencies": [
        { "name": "inexor-rgf-plugin-base", "version": ">=0.10.0, <0.11.0" },
        { "name": "inexor-rgf-plugin-file", "version": ">=0.10.0, <0.11.0" },
        { "name": "inexor-rgf-plugin-trigger", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait BinaryPlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct BinaryPluginImpl {
    component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,

    #[component(default = "component_provider_registry")]
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,

    entity_types_provider: Arc<dyn TypeProvider<EntityTypes> + Send + Sync>,

    #[component(default = "entity_types_provider_registry")]
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,

    #[component(default = "entity_component_behaviour_registry")]
    entity_component_behaviour_registry: Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>,

    web_resource_provider: Arc<dyn WebResourceProvider + Send + Sync>,

    #[component(default = "web_resource_manager")]
    web_resource_manager: Arc<dyn WebResourceManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for BinaryPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;

        // Load Binary Data
        let factory = Arc::new(LoadBinaryDataFactory::new(BEHAVIOUR_LOAD_BINARY_DATA.clone()));
        self.entity_component_behaviour_registry
            .register(COMPONENT_BEHAVIOUR_LOAD_BINARY_DATA.clone(), factory)
            .await;

        // Save Binary Data
        let factory = Arc::new(SaveBinaryDataFactory::new(BEHAVIOUR_SAVE_BINARY_DATA.clone()));
        self.entity_component_behaviour_registry
            .register(COMPONENT_BEHAVIOUR_SAVE_BINARY_DATA.clone(), factory)
            .await;
        self.web_resource_manager.register_provider(self.web_resource_provider.clone()).await;

        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.web_resource_manager.unregister_provider(self.web_resource_provider.id()).await;
        self.entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_SAVE_BINARY_DATA).await;
        self.entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_LOAD_BINARY_DATA).await;
        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}
