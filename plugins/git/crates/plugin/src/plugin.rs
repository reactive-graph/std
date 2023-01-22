use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::behaviour::component::repository::RepositoryFactory;
use crate::di::*;
use crate::model_git::BEHAVIOUR_REPOSITORY;
use crate::model_git::COMPONENT_BEHAVIOUR_REPOSITORY;
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
use crate::providers::GitComponentProviderImpl;
use crate::providers::GitEntityTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<std::sync::Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait GitPlugin: Plugin + Send + Sync {}

#[module]
pub struct GitPluginImpl {
    component_provider: Wrc<GitComponentProviderImpl>,
    entity_type_provider: Wrc<GitEntityTypeProviderImpl>,

    context: PluginContextContainer,
}

impl GitPluginImpl {}

interfaces!(GitPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl GitPlugin for GitPluginImpl {}

impl Plugin for GitPluginImpl {
    fn activate(&self) -> Result<(), PluginActivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_component_behaviour_registry = context.get_entity_component_behaviour_registry();
            // Git Repository
            let factory = Arc::new(RepositoryFactory::new(BEHAVIOUR_REPOSITORY.clone()));
            entity_component_behaviour_registry.register(COMPONENT_BEHAVIOUR_REPOSITORY.clone(), factory);
        }
        Ok(())
    }

    fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_component_behaviour_registry = context.get_entity_component_behaviour_registry();
            entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_REPOSITORY);
        }
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context.clone());
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
