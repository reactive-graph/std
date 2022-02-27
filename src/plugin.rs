use std::sync::{Arc, RwLock};

use crate::behaviour::component::component_behaviour_provider::ValueComponentBehaviourProviderImpl;
use crate::di::*;
use async_trait::async_trait;
use log::debug;

use crate::plugins::plugin::PluginMetadata;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::{
    ComponentBehaviourProvider, ComponentProvider, EntityBehaviourProvider, EntityTypeProvider,
    FlowProvider, Plugin, PluginError, RelationBehaviourProvider, RelationTypeProvider,
    WebResourceProvider,
};
use crate::provider::{ValueComponentProviderImpl, ValueEntityTypeProviderImpl};

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<std::sync::Arc<dyn PluginContext>>>);

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
    component_behaviour_provider: Wrc<ValueComponentBehaviourProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(ValuePluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl ValuePlugin for ValuePluginImpl {}

impl Plugin for ValuePluginImpl {
    fn metadata(&self) -> Result<PluginMetadata, PluginError> {
        Ok(PluginMetadata {
            name: env!("CARGO_PKG_NAME").into(),
            description: env!("CARGO_PKG_DESCRIPTION").into(),
            version: env!("CARGO_PKG_VERSION").into(),
        })
    }

    fn init(&self) -> Result<(), PluginError> {
        debug!("ValuePluginModuleImpl::init()");
        Ok(())
    }

    fn post_init(&self) -> Result<(), PluginError> {
        debug!("ValuePluginModuleImpl::post_init()");
        Ok(())
    }

    fn pre_shutdown(&self) -> Result<(), PluginError> {
        debug!("ValuePluginModuleImpl::pre_shutdown()");
        Ok(())
    }

    fn shutdown(&self) -> Result<(), PluginError> {
        debug!("ValuePluginModuleImpl::shutdown()");
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginError> {
        self.context.0.write().unwrap().replace(context);
        Ok(())
    }

    fn get_component_provider(&self) -> Result<Arc<dyn ComponentProvider>, PluginError> {
        let component_provider = self.component_provider.clone();
        let component_provider: Result<Arc<dyn ComponentProvider>, _> =
            <dyn query_interface::Object>::query_arc(component_provider);
        if component_provider.is_err() {
            return Err(PluginError::NoComponentProvider);
        }
        Ok(component_provider.unwrap())
    }

    fn get_entity_type_provider(&self) -> Result<Arc<dyn EntityTypeProvider>, PluginError> {
        let entity_type_provider = self.entity_type_provider.clone();
        let entity_type_provider: Result<Arc<dyn EntityTypeProvider>, _> =
            <dyn query_interface::Object>::query_arc(entity_type_provider);
        if entity_type_provider.is_err() {
            return Err(PluginError::NoEntityTypeProvider);
        }
        Ok(entity_type_provider.unwrap())
    }

    fn get_relation_type_provider(&self) -> Result<Arc<dyn RelationTypeProvider>, PluginError> {
        Err(PluginError::NoRelationTypeProvider)
    }

    fn get_component_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn ComponentBehaviourProvider>, PluginError> {
        let component_behaviour_provider = self.component_behaviour_provider.clone();
        let component_behaviour_provider: Result<Arc<dyn ComponentBehaviourProvider>, _> =
            <dyn query_interface::Object>::query_arc(component_behaviour_provider);
        if component_behaviour_provider.is_err() {
            return Err(PluginError::NoComponentBehaviourProvider);
        }
        Ok(component_behaviour_provider.unwrap())
    }

    fn get_entity_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn EntityBehaviourProvider>, PluginError> {
        Err(PluginError::NoEntityBehaviourProvider)
    }

    fn get_relation_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn RelationBehaviourProvider>, PluginError> {
        Err(PluginError::NoRelationBehaviourProvider)
    }

    fn get_flow_provider(&self) -> Result<Arc<dyn FlowProvider>, PluginError> {
        Err(PluginError::NoFlowProvider)
    }

    fn get_web_resource_provider(&self) -> Result<Arc<dyn WebResourceProvider>, PluginError> {
        Err(PluginError::NoWebResourceProvider)
    }
}
