use crate::behaviour::component::FsNotifyFactory;
use reactive_graph_model_file::BEHAVIOUR_FS_NOTIFY;
use reactive_graph_model_file::COMPONENT_BEHAVIOUR_FS_NOTIFY;
use reactive_graph_plugin_api::EntityComponentBehaviourRegistry;
use reactive_graph_plugin_api::prelude::plugin::*;
use reactive_graph_plugin_api::prelude::providers::*;

export_plugin!({
    "dependencies": [
        { "name": "reactive-graph-plugin-base", "version": ">=0.10.0, <0.11.0" }
        { "name": "reactive-graph-plugin-trigger", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait FilePlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct FilePluginImpl {
    component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,

    #[component(default = "component_provider_registry")]
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,

    #[component(default = "entity_component_behaviour_registry")]
    entity_component_behaviour_registry: Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for FilePluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_component_behaviour_registry
            .register(COMPONENT_BEHAVIOUR_FS_NOTIFY.clone(), Arc::new(FsNotifyFactory::new(BEHAVIOUR_FS_NOTIFY.clone())))
            .await;
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_FS_NOTIFY).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}
