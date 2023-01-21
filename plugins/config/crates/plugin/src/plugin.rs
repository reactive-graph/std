use std::sync::Arc;
use std::sync::RwLock;

use crate::behaviour::component::config_file::ConfigFileFactory;
use async_trait::async_trait;

use crate::di::*;
use crate::model_config::BEHAVIOUR_CONFIG_FILE;
use crate::model_config::COMPONENT_BEHAVIOUR_CONFIG_FILE;
use crate::plugins::component_provider;
use crate::plugins::entity_type_provider;
use crate::plugins::flow_instance_provider;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::ComponentProvider;
use crate::plugins::ComponentProviderError;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::FlowInstanceProvider;
use crate::plugins::FlowInstanceProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginActivationError;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::plugins::PluginDeactivationError;
use crate::providers::ConfigComponentProviderImpl;
use crate::providers::ConfigEntityTypeProviderImpl;
use crate::providers::ConfigFlowInstanceProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait ConfigPlugin: Plugin + Send + Sync {}

#[module]
pub struct ConfigPluginImpl {
    component_provider: Wrc<ConfigComponentProviderImpl>,
    entity_type_provider: Wrc<ConfigEntityTypeProviderImpl>,
    flow_instance_provider: Wrc<ConfigFlowInstanceProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(ConfigPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl ConfigPlugin for ConfigPluginImpl {}

impl Plugin for ConfigPluginImpl {
    fn activate(&self) -> Result<(), PluginActivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_component_behaviour_registry = context.get_entity_component_behaviour_registry();

            // Component Behaviour ConfigFile
            let factory = Arc::new(ConfigFileFactory::new(BEHAVIOUR_CONFIG_FILE.clone()));
            entity_component_behaviour_registry.register(COMPONENT_BEHAVIOUR_CONFIG_FILE.clone(), factory);
        }
        Ok(())
    }
    fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_component_behaviour_registry = context.get_entity_component_behaviour_registry();
            entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_CONFIG_FILE);
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

    fn get_flow_instance_provider(&self) -> Result<Option<Arc<dyn FlowInstanceProvider>>, FlowInstanceProviderError> {
        flow_instance_provider!(self.flow_instance_provider)
    }
}
