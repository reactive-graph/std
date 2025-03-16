use reactive_graph_plugin_api::EntityBehaviourRegistry;
use reactive_graph_plugin_api::prelude::plugin::*;
use reactive_graph_plugin_api::prelude::providers::*;

use crate::behaviour::entity::gate::COMPARISON_GATES;

export_plugin!({
    "dependencies": [
        { "name": "reactive-graph-plugin-base", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-plugin-result", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait ComparisonPlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct ComparisonPluginImpl {
    component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,

    #[component(default = "component_provider_registry")]
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,

    entity_types_provider: Arc<dyn TypeProvider<EntityTypes> + Send + Sync>,

    #[component(default = "entity_types_provider_registry")]
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,

    #[component(default = "entity_behaviour_registry")]
    entity_behaviour_registry: Arc<dyn EntityBehaviourRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for ComparisonPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;
        self.entity_behaviour_registry.register_all(&COMPARISON_GATES.get_factories()).await;
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.entity_behaviour_registry.unregister_all(&COMPARISON_GATES.to_entity_behaviour_tys()).await;
        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}
