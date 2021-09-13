use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use waiter_di::*;

use crate::behaviour::entity::entity_behaviour_provider::NumericEntityBehaviourProviderImpl;
use crate::plugins::{
    ComponentProvider, EntityBehaviourProvider, EntityTypeProvider, FlowProvider, Plugin,
    PluginError, RelationBehaviourProvider, RelationTypeProvider, WebResourceProvider,
};
use crate::provider::{NumericComponentProviderImpl, NumericEntityTypeProviderImpl};

#[async_trait]
pub trait NumericPlugin: Plugin + Send + Sync {}

#[module]
pub struct NumericPluginImpl {
    component_provider: Wrc<NumericComponentProviderImpl>,
    entity_type_provider: Wrc<NumericEntityTypeProviderImpl>,
    entity_behaviour_provider: Wrc<NumericEntityBehaviourProviderImpl>,
}

interfaces!(NumericPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl NumericPlugin for NumericPluginImpl {}

impl Plugin for NumericPluginImpl {
    fn init(&self) -> Result<(), PluginError> {
        debug!("NumericPluginModuleImpl::init()");
        Ok(())
    }

    fn post_init(&self) -> Result<(), PluginError> {
        debug!("NumericPluginModuleImpl::post_init()");
        Ok(())
    }

    fn pre_shutdown(&self) -> Result<(), PluginError> {
        debug!("NumericPluginModuleImpl::pre_shutdown()");
        Ok(())
    }

    fn shutdown(&self) -> Result<(), PluginError> {
        debug!("NumericPluginModuleImpl::shutdown()");
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
