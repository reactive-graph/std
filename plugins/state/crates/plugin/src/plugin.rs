use std::ops::Deref;

use reactive_graph_plugin_api::prelude::plugin::*;
use reactive_graph_plugin_api::prelude::providers::*;

use crate::behaviour::component::state::STATE_BEHAVIOURS;
use crate::behaviour::component::state::STATE_FACTORIES;
use crate::behaviour::component::state_debugger::STATE_DEBUGGER_BEHAVIOURS;

export_plugin!({
    "dependencies": [
        { "name": "reactive-graph-std-base", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-std-value", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[derive(Component)]
pub struct StatePlugin {
    component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,

    #[component(default = "component_provider_registry")]
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,

    entity_types_provider: Arc<dyn TypeProvider<EntityTypes> + Send + Sync>,

    #[component(default = "entity_types_provider_registry")]
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,

    #[component(default = "inject_plugin_context")]
    context: Arc<dyn PluginContext + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for StatePlugin {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;
        self.context
            .get_entity_component_behaviour_registry()
            .register_all(STATE_FACTORIES.deref())
            .await;
        self.context
            .get_entity_component_behaviour_registry()
            .register_all(&STATE_DEBUGGER_BEHAVIOURS.get_factories())
            .await;
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.context
            .get_entity_component_behaviour_registry()
            .unregister_all(&STATE_DEBUGGER_BEHAVIOURS.to_component_behaviour_tys())
            .await;
        self.context
            .get_entity_component_behaviour_registry()
            .unregister_all(&STATE_BEHAVIOURS.deref().into())
            .await;
        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}
