use reactive_graph_plugin_api::EntityComponentBehaviourRegistry;
use reactive_graph_plugin_api::prelude::plugin::*;
use reactive_graph_plugin_api::prelude::providers::*;

use crate::behaviour::component::http::HttpFactory;
use crate::behaviour::component::json_rpc::JsonRpcFactory;
use reactive_graph_model_http::BEHAVIOUR_HTTP;
use reactive_graph_model_http::BEHAVIOUR_JSON_RPC;
use reactive_graph_model_http::COMPONENT_BEHAVIOUR_HTTP;
use reactive_graph_model_http::COMPONENT_BEHAVIOUR_JSON_RPC;

export_plugin!({
    "dependencies": [
        { "name": "reactive-graph-plugin-base", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-plugin-result", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-plugin-trigger", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait HttpPlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct HttpPluginImpl {
    component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,

    #[component(default = "component_provider_registry")]
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,

    entity_types_provider: Arc<dyn TypeProvider<EntityTypes> + Send + Sync>,

    #[component(default = "entity_types_provider_registry")]
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,

    #[component(default = "entity_component_behaviour_registry")]
    entity_component_behaviour_registry: Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for HttpPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;

        // HTTP
        let factory = Arc::new(HttpFactory::new(BEHAVIOUR_HTTP.clone()));
        self.entity_component_behaviour_registry
            .register(COMPONENT_BEHAVIOUR_HTTP.clone(), factory)
            .await;

        // JSON_RPC
        let factory = Arc::new(JsonRpcFactory::new(BEHAVIOUR_JSON_RPC.clone()));
        self.entity_component_behaviour_registry
            .register(COMPONENT_BEHAVIOUR_JSON_RPC.clone(), factory)
            .await;

        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_HTTP).await;
        self.entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_JSON_RPC).await;
        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}

// use std::sync::Arc;
// use std::sync::RwLock;
//
// use async_trait::async_trait;
//
// use crate::behaviour::component::http::HttpFactory;
// use crate::behaviour::component::json_rpc::JsonRpcFactory;
// use crate::di::*;
// use reactive_graph_model_http::BEHAVIOUR_HTTP;
// use reactive_graph_model_http::BEHAVIOUR_JSON_RPC;
// use reactive_graph_model_http::COMPONENT_BEHAVIOUR_HTTP;
// use reactive_graph_model_http::COMPONENT_BEHAVIOUR_JSON_RPC;
// use crate::plugins::component_provider;
// use crate::plugins::entity_type_provider;
// use crate::plugins::plugin_context::PluginContext;
// use crate::plugins::ComponentProvider;
// use crate::plugins::ComponentProviderError;
// use crate::plugins::EntityTypeProvider;
// use crate::plugins::EntityTypeProviderError;
// use crate::plugins::Plugin;
// use crate::plugins::PluginActivationError;
// use crate::plugins::PluginContextDeinitializationError;
// use crate::plugins::PluginContextInitializationError;
// use crate::plugins::PluginDeactivationError;
// use crate::providers::HttpComponentProviderImpl;
// use crate::providers::HttpEntityTypeProviderImpl;
//
// #[wrapper]
// pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);
//
// #[provides]
// fn create_empty_plugin_context_container() -> PluginContextContainer {
//     PluginContextContainer(RwLock::new(None))
// }
//
// pub trait HttpPlugin: Plugin + Send + Sync {}
//
// #[component]
// pub struct HttpPluginImpl {
//     component_provider: Wrc<HttpComponentProviderImpl>,
//     entity_type_provider: Wrc<HttpEntityTypeProviderImpl>,
//
//     context: PluginContextContainer,
// }
//
// impl HttpPluginImpl {}
//
// impl HttpPluginImpl {}
//
// interfaces!(HttpPluginImpl: dyn Plugin);
//
// #[provides]
// #[async_trait]
// impl HttpPlugin for HttpPluginImpl {}
//
// #[async_trait]
// impl Plugin for HttpPluginImpl {
//     async fn activate(&self) -> Result<(), PluginActivationError> {
//         let guard = self.context.0.read().unwrap();
//         if let Some(context) = guard.clone() {
//             let entity_component_behaviour_registry = context.get_entity_component_behaviour_registry();
//             // HTTP
//             let factory = Arc::new(HttpFactory::new(BEHAVIOUR_HTTP.clone()));
//             entity_component_behaviour_registry.register(COMPONENT_BEHAVIOUR_HTTP.clone(), factory);
//
//             // JSON_RPC
//             let factory = Arc::new(JsonRpcFactory::new(BEHAVIOUR_JSON_RPC.clone()));
//             entity_component_behaviour_registry.register(COMPONENT_BEHAVIOUR_JSON_RPC.clone(), factory);
//         }
//         Ok(())
//     }
//
//     async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
//         let guard = self.context.0.read().unwrap();
//         if let Some(context) = guard.clone() {
//             let entity_component_behaviour_registry = context.get_entity_component_behaviour_registry();
//             entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_HTTP);
//             entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_JSON_RPC);
//         }
//         Ok(())
//     }
//
//     fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
//         self.context.0.write().unwrap().replace(context.clone());
//         Ok(())
//     }
//
//     fn remove_context(&self) -> Result<(), PluginContextDeinitializationError> {
//         let mut writer = self.context.0.write().unwrap();
//         *writer = None;
//         Ok(())
//     }
//
//     fn get_component_provider(&self) -> Result<Option<Arc<dyn ComponentProvider>>, ComponentProviderError> {
//         component_provider!(self.component_provider)
//     }
//
//     fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
//         entity_type_provider!(self.entity_type_provider)
//     }
// }
