use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::behaviour::entity::hsv_to_rgb::HsvToRgbFactory;
use crate::behaviour::entity::rgb_to_hsv::RgbToHsvFactory;
use crate::di::*;
use crate::model_color::BEHAVIOUR_HSV_TO_RGB;
use crate::model_color::BEHAVIOUR_RGB_TO_HSV;
use crate::model_color::ENTITY_BEHAVIOUR_HSV_TO_RGB;
use crate::model_color::ENTITY_BEHAVIOUR_RGB_TO_HSV;
use crate::plugins::component_provider;
use crate::plugins::entity_type_provider;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::ComponentProvider;
use crate::plugins::ComponentProviderError;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginActivationError;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::plugins::PluginDeactivationError;
use crate::providers::ColorComponentProviderImpl;
use crate::providers::ColorEntityTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

pub trait ColorPlugin: Plugin + Send + Sync {}

#[module]
pub struct ColorPluginImpl {
    component_provider: Wrc<ColorComponentProviderImpl>,
    entity_type_provider: Wrc<ColorEntityTypeProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(ColorPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl ColorPlugin for ColorPluginImpl {}

#[async_trait]
impl Plugin for ColorPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();

            let factory = Arc::new(RgbToHsvFactory::new(BEHAVIOUR_RGB_TO_HSV.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_RGB_TO_HSV.clone(), factory);

            let factory = Arc::new(HsvToRgbFactory::new(BEHAVIOUR_HSV_TO_RGB.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_HSV_TO_RGB.clone(), factory);
        }
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_HSV_TO_RGB);
        }
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

    fn get_component_provider(&self) -> Result<Option<Arc<dyn ComponentProvider>>, ComponentProviderError> {
        component_provider!(self.component_provider)
    }

    fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
        entity_type_provider!(self.entity_type_provider)
    }
}
