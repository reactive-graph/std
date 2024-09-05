use reactive_graph_plugin_api::prelude::plugin::*;
use reactive_graph_plugin_api::prelude::providers::*;

export_plugin!({
    "dependencies": [
        { "name": "reactive-graph-plugin-base", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[derive(Component)]
pub struct ResultPlugin {
    component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,

    #[component(default = "component_provider_registry")]
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for ResultPlugin {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}

// use std::sync::Arc;
// use std::sync::RwLock;
//
// use crate::di::*;
// use crate::plugins::component_provider;
// use crate::plugins::plugin_context::PluginContext;
// use crate::plugins::ComponentProvider;
// use crate::plugins::ComponentProviderError;
// use crate::plugins::Plugin;
// use crate::plugins::PluginContextDeinitializationError;
// use crate::plugins::PluginContextInitializationError;
// use crate::providers::ResultComponentProviderImpl;
//
// #[wrapper]
// pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);
//
// #[provides]
// fn create_empty_plugin_context_container() -> PluginContextContainer {
//     PluginContextContainer(RwLock::new(None))
// }
//
// pub trait ResultPlugin: Plugin + Send + Sync {}
//
// #[component]
// pub struct ResultPluginImpl {
//     component_provider: Wrc<ResultComponentProviderImpl>,
//
//     context: PluginContextContainer,
// }
//
// interfaces!(ResultPluginImpl: dyn Plugin);
//
// #[provides]
// impl ResultPlugin for ResultPluginImpl {}
//
// impl Plugin for ResultPluginImpl {
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
// }
