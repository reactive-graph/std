use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::behaviour::entity::gate::behaviour::ComparisonGateFactory;
use crate::behaviour::entity::gate::COMPARISON_GATES;
use crate::di::*;
use crate::model::EntityBehaviourTypeId;
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
use crate::providers::ComparisonComponentProviderImpl;
use crate::providers::ComparisonEntityTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait ComparisonPlugin: Plugin + Send + Sync {}

#[module]
pub struct ComparisonPluginImpl {
    component_provider: Wrc<ComparisonComponentProviderImpl>,
    entity_type_provider: Wrc<ComparisonEntityTypeProviderImpl>,
    // entity_behaviour_provider: Wrc<ComparisonEntityBehaviourProviderImpl>,
    context: PluginContextContainer,
}

interfaces!(ComparisonPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl ComparisonPlugin for ComparisonPluginImpl {}

impl Plugin for ComparisonPluginImpl {
    fn activate(&self) -> Result<(), PluginActivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();
            for (behaviour_ty, f) in COMPARISON_GATES.iter() {
                entity_behaviour_registry.register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(ComparisonGateFactory::new(behaviour_ty.clone(), *f)));
            }
        }
        Ok(())
    }

    fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();
            for behaviour_ty in COMPARISON_GATES.keys() {
                entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
            }
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
