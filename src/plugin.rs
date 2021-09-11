use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use waiter_di::*;

use crate::behaviour::relation::relation_behaviour_provider::ConnectorRelationBehaviourProviderImpl;
use crate::plugins::{
    ComponentProvider, EntityBehaviourProvider, EntityTypeProvider, FlowProvider, Plugin,
    PluginError, RelationBehaviourProvider, RelationTypeProvider,
};
use crate::provider::{ConnectorComponentProviderImpl, ConnectorRelationTypeProviderImpl};

#[async_trait]
pub trait ConnectorPlugin: Plugin + Send + Sync {}

#[module]
pub struct ConnectorPluginImpl {
    component_provider: Wrc<ConnectorComponentProviderImpl>,
    relation_type_provider: Wrc<ConnectorRelationTypeProviderImpl>,
    relation_behaviour_provider: Wrc<ConnectorRelationBehaviourProviderImpl>,
}

interfaces!(ConnectorPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl ConnectorPlugin for ConnectorPluginImpl {}

impl Plugin for ConnectorPluginImpl {
    fn init(&self) -> Result<(), PluginError> {
        debug!("ConnectorPluginModuleImpl::init()");
        Ok(())
    }

    fn post_init(&self) -> Result<(), PluginError> {
        debug!("ConnectorPluginModuleImpl::post_init()");
        Ok(())
    }

    fn pre_shutdown(&self) -> Result<(), PluginError> {
        debug!("ConnectorPluginModuleImpl::pre_shutdown()");
        Ok(())
    }

    fn shutdown(&self) -> Result<(), PluginError> {
        debug!("ConnectorPluginModuleImpl::shutdown()");
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
        let relation_type_provider = self.relation_type_provider.clone();
        let relation_type_provider: Result<Arc<dyn RelationTypeProvider>, _> =
            <dyn query_interface::Object>::query_arc(relation_type_provider);
        if relation_type_provider.is_err() {
            return Err(PluginError::NoRelationTypeProvider);
        }
        Ok(relation_type_provider.unwrap())
    }

    fn get_entity_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn EntityBehaviourProvider>, PluginError> {
        Err(PluginError::NoEntityBehaviourProvider)
    }

    fn get_relation_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn RelationBehaviourProvider>, PluginError> {
        let relation_behaviour_provider = self.relation_behaviour_provider.clone();
        let relation_behaviour_provider: Result<Arc<dyn RelationBehaviourProvider>, _> =
            <dyn query_interface::Object>::query_arc(relation_behaviour_provider);
        if relation_behaviour_provider.is_err() {
            return Err(PluginError::NoRelationBehaviourProvider);
        }
        Ok(relation_behaviour_provider.unwrap())
    }

    fn get_flow_provider(&self) -> Result<Arc<dyn FlowProvider>, PluginError> {
        Err(PluginError::NoFlowProvider)
    }
}
