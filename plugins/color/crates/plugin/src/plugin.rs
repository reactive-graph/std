use inexor_rgf_plugin_api::prelude::plugin::*;
use inexor_rgf_plugin_api::prelude::providers::*;
use inexor_rgf_plugin_api::EntityBehaviourRegistry;

use crate::behaviour::entity::hsv_to_rgb::HsvToRgbFactory;
use crate::behaviour::entity::rgb_to_hsv::RgbToHsvFactory;
use inexor_rgf_model_color::BEHAVIOUR_HSV_TO_RGB;
use inexor_rgf_model_color::BEHAVIOUR_RGB_TO_HSV;
use inexor_rgf_model_color::ENTITY_BEHAVIOUR_HSV_TO_RGB;
use inexor_rgf_model_color::ENTITY_BEHAVIOUR_RGB_TO_HSV;

export_plugin!({
    "dependencies": [
        { "name": "inexor-rgf-plugin-base", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait ColorPlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct ColorPluginImpl {
    component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,

    #[component(default = "component_provider_registry")]
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,

    entity_types_provider: Arc<dyn TypeProvider<EntityTypes> + Send + Sync>,

    #[component(default = "entity_types_provider_registry")]
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,

    #[component(default = "entity_behaviour_registry")]
    entity_behaviour_registry: Arc<dyn EntityBehaviourRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for ColorPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;
        let factory = Arc::new(RgbToHsvFactory::new(BEHAVIOUR_RGB_TO_HSV.clone()));
        self.entity_behaviour_registry.register(ENTITY_BEHAVIOUR_RGB_TO_HSV.clone(), factory).await;

        let factory = Arc::new(HsvToRgbFactory::new(BEHAVIOUR_HSV_TO_RGB.clone()));
        self.entity_behaviour_registry.register(ENTITY_BEHAVIOUR_HSV_TO_RGB.clone(), factory).await;
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_HSV_TO_RGB).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RGB_TO_HSV).await;
        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}
