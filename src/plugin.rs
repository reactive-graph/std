use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;

use crate::di::*;
use crate::plugins::plugin::PluginMetadata;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::ComponentBehaviourProvider;
use crate::plugins::ComponentProvider;
use crate::plugins::EntityBehaviourProvider;
use crate::plugins::EntityTypeProvider;
use crate::plugins::FlowProvider;
use crate::plugins::Plugin;
use crate::plugins::PluginError;
use crate::plugins::RelationBehaviourProvider;
use crate::plugins::RelationTypeProvider;
use crate::plugins::WebResourceProvider;
use crate::provider::GraphQlClientWebResourceProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<std::sync::Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait GraphQlClientPlugin: Plugin + Send + Sync {}

#[module]
pub struct GraphQlClientPluginImpl {
    web_resource_provider: Wrc<GraphQlClientWebResourceProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(GraphQlClientPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl GraphQlClientPlugin for GraphQlClientPluginImpl {}

impl Plugin for GraphQlClientPluginImpl {
    fn metadata(&self) -> Result<PluginMetadata, PluginError> {
        Ok(PluginMetadata {
            name: env!("CARGO_PKG_NAME").into(),
            description: env!("CARGO_PKG_DESCRIPTION").into(),
            version: env!("CARGO_PKG_VERSION").into(),
        })
    }

    fn init(&self) -> Result<(), PluginError> {
        debug!("GraphQlClientPluginModuleImpl::init()");
        Ok(())
    }

    fn post_init(&self) -> Result<(), PluginError> {
        debug!("GraphQlClientPluginModuleImpl::post_init()");
        Ok(())
    }

    fn pre_shutdown(&self) -> Result<(), PluginError> {
        debug!("GraphQlClientPluginModuleImpl::pre_shutdown()");
        Ok(())
    }

    fn shutdown(&self) -> Result<(), PluginError> {
        debug!("GraphQlClientPluginModuleImpl::shutdown()");
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginError> {
        self.context.0.write().unwrap().replace(context);
        Ok(())
    }

    fn get_component_provider(&self) -> Result<Arc<dyn ComponentProvider>, PluginError> {
        Err(PluginError::NoComponentProvider)
    }

    fn get_entity_type_provider(&self) -> Result<Arc<dyn EntityTypeProvider>, PluginError> {
        Err(PluginError::NoEntityTypeProvider)
    }

    fn get_relation_type_provider(&self) -> Result<Arc<dyn RelationTypeProvider>, PluginError> {
        Err(PluginError::NoRelationTypeProvider)
    }

    fn get_component_behaviour_provider(&self) -> Result<Arc<dyn ComponentBehaviourProvider>, PluginError> {
        Err(PluginError::NoComponentBehaviourProvider)
    }

    fn get_entity_behaviour_provider(&self) -> Result<Arc<dyn EntityBehaviourProvider>, PluginError> {
        Err(PluginError::NoEntityBehaviourProvider)
    }

    fn get_relation_behaviour_provider(&self) -> Result<Arc<dyn RelationBehaviourProvider>, PluginError> {
        Err(PluginError::NoRelationBehaviourProvider)
    }

    fn get_flow_provider(&self) -> Result<Arc<dyn FlowProvider>, PluginError> {
        Err(PluginError::NoFlowProvider)
    }

    fn get_web_resource_provider(&self) -> Result<Arc<dyn WebResourceProvider>, PluginError> {
        let web_resource_provider = self.web_resource_provider.clone();
        let web_resource_provider: Result<Arc<dyn WebResourceProvider>, _> = <dyn query_interface::Object>::query_arc(web_resource_provider);
        if web_resource_provider.is_err() {
            return Err(PluginError::NoWebResourceProvider);
        }
        Ok(web_resource_provider.unwrap())
    }
}
