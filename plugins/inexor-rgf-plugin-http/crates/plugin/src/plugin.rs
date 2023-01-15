use crate::behaviour::component::http::HttpFactory;
use crate::behaviour::component::json_rpc::JsonRpcFactory;
use std::sync::Arc;
use std::sync::RwLock;

use crate::di::*;
use crate::model_http::BEHAVIOUR_HTTP;
use crate::model_http::BEHAVIOUR_JSON_RPC;
use crate::model_http::COMPONENT_BEHAVIOUR_HTTP;
use crate::model_http::COMPONENT_BEHAVIOUR_JSON_RPC;
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
use crate::providers::HttpComponentProviderImpl;
use crate::providers::HttpEntityTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<std::sync::Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

pub trait HttpPlugin: Plugin + Send + Sync {}

#[module]
pub struct HttpPluginImpl {
    component_provider: Wrc<HttpComponentProviderImpl>,
    entity_type_provider: Wrc<HttpEntityTypeProviderImpl>,

    context: PluginContextContainer,
}

impl HttpPluginImpl {}

impl HttpPluginImpl {}

interfaces!(HttpPluginImpl: dyn Plugin);

#[provides]
impl HttpPlugin for HttpPluginImpl {}

impl Plugin for HttpPluginImpl {
    fn activate(&self) -> Result<(), PluginActivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_component_behaviour_registry = context.get_entity_component_behaviour_registry();
            // HTTP
            let factory = Arc::new(HttpFactory::new(BEHAVIOUR_HTTP.clone()));
            entity_component_behaviour_registry.register(COMPONENT_BEHAVIOUR_HTTP.clone(), factory);

            // JSON_RPC
            let factory = Arc::new(JsonRpcFactory::new(BEHAVIOUR_JSON_RPC.clone()));
            entity_component_behaviour_registry.register(COMPONENT_BEHAVIOUR_JSON_RPC.clone(), factory);
        }
        Ok(())
    }

    fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_component_behaviour_registry = context.get_entity_component_behaviour_registry();
            entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_HTTP);
            entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_JSON_RPC);
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
