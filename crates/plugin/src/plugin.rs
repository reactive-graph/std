use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::info;

use crate::behaviour::component::StateDebuggerFactory;
use crate::behaviour::component::StateFactory;
use crate::behaviour::component::ValueDebuggerFactory;
use crate::behaviour::component::STATE_BEHAVIOURS;
use crate::behaviour::component::STATE_DEBUGGER_BEHAVIOURS;
use crate::behaviour::component::VALUE_DEBUGGER_BEHAVIOURS;
use crate::di::module;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::ComponentBehaviourTypeId;
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
use crate::provider::ValueComponentProviderImpl;
use crate::provider::ValueEntityTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait ValuePlugin: Plugin + Send + Sync {}

#[module]
pub struct ValuePluginImpl {
    component_provider: Wrc<ValueComponentProviderImpl>,
    entity_type_provider: Wrc<ValueEntityTypeProviderImpl>,
    // component_behaviour_provider: Wrc<ValueComponentBehaviourProviderImpl>,
    context: PluginContextContainer,
}

interfaces!(ValuePluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl ValuePlugin for ValuePluginImpl {}

impl Plugin for ValuePluginImpl {
    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context);
        Ok(())
    }

    fn remove_context(&self) -> Result<(), PluginContextDeinitializationError> {
        let mut writer = self.context.0.write().unwrap();
        *writer = None;
        Ok(())
    }

    fn activate(&self) -> Result<(), PluginActivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_component_behaviour_registry = context.get_entity_component_behaviour_registry();
            info!("State Behaviours");
            for behaviour_ty in STATE_BEHAVIOURS.iter() {
                info!("Create and register factory for {}", &behaviour_ty);
                entity_component_behaviour_registry.register(ComponentBehaviourTypeId::from(behaviour_ty), Arc::new(StateFactory::new(behaviour_ty.clone())));
            }
            info!("State Debugger Behaviours");
            for (behaviour_ty, f) in STATE_DEBUGGER_BEHAVIOURS.iter() {
                entity_component_behaviour_registry
                    .register(ComponentBehaviourTypeId::from(behaviour_ty), Arc::new(StateDebuggerFactory::new(behaviour_ty.clone(), *f)));
            }
            info!("Value Debugger Behaviours");
            for (behaviour_ty, f) in VALUE_DEBUGGER_BEHAVIOURS.iter() {
                entity_component_behaviour_registry
                    .register(ComponentBehaviourTypeId::from(behaviour_ty), Arc::new(ValueDebuggerFactory::new(behaviour_ty.clone(), *f)));
            }
        }
        Ok(())
    }

    fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        info!("Deactivate");
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_component_behaviour_registry = context.get_entity_component_behaviour_registry();
            info!("State Behaviours");
            for behaviour_ty in STATE_BEHAVIOURS.iter() {
                entity_component_behaviour_registry.unregister(&ComponentBehaviourTypeId::from(behaviour_ty));
            }
            info!("State Debugger Behaviours");
            for behaviour_ty in STATE_DEBUGGER_BEHAVIOURS.keys() {
                entity_component_behaviour_registry.unregister(&ComponentBehaviourTypeId::from(behaviour_ty));
            }
            info!("Value Debugger Behaviours: TODO");
            for behaviour_ty in VALUE_DEBUGGER_BEHAVIOURS.keys() {
                entity_component_behaviour_registry.unregister(&ComponentBehaviourTypeId::from(behaviour_ty));
            }
        }
        Ok(())
    }

    fn get_component_provider(&self) -> Result<Option<Arc<dyn ComponentProvider>>, ComponentProviderError> {
        component_provider!(self.component_provider)
    }

    fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
        entity_type_provider!(self.entity_type_provider)
    }

    // fn get_component_behaviour_provider(&self) -> Result<Option<Arc<dyn ComponentBehaviourProvider>>, ComponentBehaviourProviderError> {
    //     component_behaviour_provider!(self.component_behaviour_provider)
    // }
}
