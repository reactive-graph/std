use crate::behaviour::component::config_file::ConfigFileFactory;
use reactive_graph_model_config::BEHAVIOUR_CONFIG_FILE;
use reactive_graph_model_config::COMPONENT_BEHAVIOUR_CONFIG_FILE;
use reactive_graph_plugin_api::prelude::plugin::*;
use reactive_graph_plugin_api::prelude::providers::*;
use reactive_graph_plugin_api::EntityComponentBehaviourRegistry;

export_plugin!({
    "dependencies": [
        { "name": "reactive-graph-plugin-base", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-plugin-file", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-plugin-trigger", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-plugin-result", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait ConfigPlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct ConfigPluginImpl {
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
impl Plugin for ConfigPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;
        let factory = Arc::new(ConfigFileFactory::new(BEHAVIOUR_CONFIG_FILE.clone()));
        self.entity_component_behaviour_registry
            .register(COMPONENT_BEHAVIOUR_CONFIG_FILE.clone(), factory)
            .await;
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_CONFIG_FILE).await;
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
// use crate::behaviour::component::config_file::ConfigFileFactory;
// use crate::di::*;
// use crate::model_config::BEHAVIOUR_CONFIG_FILE;
// use crate::model_config::COMPONENT_BEHAVIOUR_CONFIG_FILE;
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
// use crate::providers::ConfigComponentProviderImpl;
// use crate::providers::ConfigEntityTypeProviderImpl;
//
// #[wrapper]
// pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);
//
// #[provides]
// fn create_empty_plugin_context_container() -> PluginContextContainer {
//     PluginContextContainer(RwLock::new(None))
// }
//
// #[async_trait]
// pub trait ConfigPlugin: Plugin + Send + Sync {}
//
// #[component]
// pub struct ConfigPluginImpl {
//     component_provider: Wrc<ConfigComponentProviderImpl>,
//     entity_type_provider: Wrc<ConfigEntityTypeProviderImpl>,
//
//     context: PluginContextContainer,
// }
//
// interfaces!(ConfigPluginImpl: dyn Plugin);
//
// #[async_trait]
// #[provides]
// impl ConfigPlugin for ConfigPluginImpl {}
//
// #[async_trait]
// impl Plugin for ConfigPluginImpl {
//     async fn activate(&self) -> Result<(), PluginActivationError> {
//         let guard = self.context.0.read().unwrap();
//         if let Some(context) = guard.clone() {
//             let entity_component_behaviour_registry = context.get_entity_component_behaviour_registry();
//
//             // Component Behaviour ConfigFile
//             let factory = Arc::new(ConfigFileFactory::new(BEHAVIOUR_CONFIG_FILE.clone()));
//             entity_component_behaviour_registry.register(COMPONENT_BEHAVIOUR_CONFIG_FILE.clone(), factory);
//         }
//         Ok(())
//     }
//     async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
//         let guard = self.context.0.read().unwrap();
//         if let Some(context) = guard.clone() {
//             let entity_component_behaviour_registry = context.get_entity_component_behaviour_registry();
//             entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_CONFIG_FILE);
//         }
//         Ok(())
//     }
//
//     fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
//         self.context.0.write().unwrap().replace(context);
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
