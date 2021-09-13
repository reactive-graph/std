use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use waiter_di::*;

use crate::behaviour::entity::entity_behaviour_provider::LogicalEntityBehaviourProviderImpl;
use crate::plugins::{
    ComponentProvider, EntityBehaviourProvider, EntityTypeProvider, FlowProvider, Plugin,
    PluginError, RelationBehaviourProvider, RelationTypeProvider, WebResourceProvider,
};
use crate::provider::{LogicalComponentProviderImpl, LogicalEntityTypeProviderImpl};

#[async_trait]
pub trait LogicalPlugin: Plugin + Send + Sync {}

#[module]
pub struct LogicalPluginImpl {
    component_provider: Wrc<LogicalComponentProviderImpl>,
    entity_type_provider: Wrc<LogicalEntityTypeProviderImpl>,
    entity_behaviour_provider: Wrc<LogicalEntityBehaviourProviderImpl>,
}

interfaces!(LogicalPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl LogicalPlugin for LogicalPluginImpl {}

impl Plugin for LogicalPluginImpl {
    fn init(&self) -> Result<(), PluginError> {
        debug!("LogicalPluginModuleImpl::init()");
        Ok(())
    }

    fn post_init(&self) -> Result<(), PluginError> {
        debug!("LogicalPluginModuleImpl::post_init()");
        Ok(())
    }

    fn pre_shutdown(&self) -> Result<(), PluginError> {
        debug!("LogicalPluginModuleImpl::pre_shutdown()");
        Ok(())
    }

    fn shutdown(&self) -> Result<(), PluginError> {
        debug!("LogicalPluginModuleImpl::shutdown()");
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

    fn get_entity_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn EntityBehaviourProvider>, PluginError> {
        let entity_behaviour_provider = self.entity_behaviour_provider.clone();
        let entity_behaviour_provider: Result<Arc<dyn EntityBehaviourProvider>, _> =
            <dyn query_interface::Object>::query_arc(entity_behaviour_provider);
        if entity_behaviour_provider.is_err() {
            return Err(PluginError::NoEntityBehaviourProvider);
        }
        Ok(entity_behaviour_provider.unwrap())
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
