use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::behaviour::component::load_binary_data::LoadBinaryDataFactory;
use crate::behaviour::component::save_binary_data::SaveBinaryDataFactory;
use crate::di::*;
use crate::model_binary::BEHAVIOUR_LOAD_BINARY_DATA;
use crate::model_binary::BEHAVIOUR_SAVE_BINARY_DATA;
use crate::model_binary::COMPONENT_BEHAVIOUR_LOAD_BINARY_DATA;
use crate::model_binary::COMPONENT_BEHAVIOUR_SAVE_BINARY_DATA;
use crate::plugins::component_provider;
use crate::plugins::entity_type_provider;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::web_resource_provider;
use crate::plugins::ComponentProvider;
use crate::plugins::ComponentProviderError;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginActivationError;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::plugins::PluginDeactivationError;
use crate::plugins::WebResourceProvider;
use crate::plugins::WebResourceProviderError;
use crate::providers::BinaryComponentProviderImpl;
use crate::providers::BinaryEntityTypeProviderImpl;
use crate::providers::BinaryWebResourceProvider;
use crate::providers::BinaryWebResourceProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait BinaryPlugin: Plugin + Send + Sync {}

#[module]
pub struct BinaryPluginImpl {
    component_provider: Wrc<BinaryComponentProviderImpl>,
    entity_type_provider: Wrc<BinaryEntityTypeProviderImpl>,
    web_resource_provider: Wrc<BinaryWebResourceProviderImpl>,

    context: PluginContextContainer,
}

impl BinaryPluginImpl {}

interfaces!(BinaryPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl BinaryPlugin for BinaryPluginImpl {}

#[async_trait]
impl Plugin for BinaryPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_component_behaviour_registry = context.get_entity_component_behaviour_registry();

            // Load Binary Data
            let factory = Arc::new(LoadBinaryDataFactory::new(BEHAVIOUR_LOAD_BINARY_DATA.clone()));
            entity_component_behaviour_registry.register(COMPONENT_BEHAVIOUR_LOAD_BINARY_DATA.clone(), factory);

            // Save Binary Data
            let factory = Arc::new(SaveBinaryDataFactory::new(BEHAVIOUR_SAVE_BINARY_DATA.clone()));
            entity_component_behaviour_registry.register(COMPONENT_BEHAVIOUR_SAVE_BINARY_DATA.clone(), factory);
        }
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_component_behaviour_registry = context.get_entity_component_behaviour_registry();
            entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_LOAD_BINARY_DATA);
            entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_SAVE_BINARY_DATA);
        }
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context.clone());
        self.web_resource_provider.set_context(context.clone());
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

    fn get_web_resource_provider(&self) -> Result<Option<Arc<dyn WebResourceProvider>>, WebResourceProviderError> {
        web_resource_provider!(self.web_resource_provider)
    }
}
