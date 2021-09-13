use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use waiter_di::*;

use crate::plugins::{
    ComponentProvider, EntityBehaviourProvider, EntityTypeProvider, FlowProvider, Plugin,
    PluginError, RelationBehaviourProvider, RelationTypeProvider, WebResourceProvider,
};
use crate::provider::MetaDataComponentProviderImpl;

#[async_trait]
pub trait MetaDataPlugin: Plugin + Send + Sync {}

#[module]
pub struct MetaDataPluginImpl {
    component_provider: Wrc<MetaDataComponentProviderImpl>,
}

interfaces!(MetaDataPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl MetaDataPlugin for MetaDataPluginImpl {}

impl Plugin for MetaDataPluginImpl {
    fn init(&self) -> Result<(), PluginError> {
        debug!("MetaDataPluginModuleImpl::init()");
        Ok(())
    }

    fn post_init(&self) -> Result<(), PluginError> {
        debug!("MetaDataPluginModuleImpl::post_init()");
        Ok(())
    }

    fn pre_shutdown(&self) -> Result<(), PluginError> {
        debug!("MetaDataPluginModuleImpl::shutdown()");
        Ok(())
    }

    fn shutdown(&self) -> Result<(), PluginError> {
        debug!("MetaDataPluginModuleImpl::shutdown()");
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
        Err(PluginError::NoEntityTypeProvider)
    }

    fn get_relation_type_provider(&self) -> Result<Arc<dyn RelationTypeProvider>, PluginError> {
        Err(PluginError::NoRelationTypeProvider)
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
