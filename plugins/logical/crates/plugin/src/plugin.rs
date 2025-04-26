use reactive_graph_plugin_api::EntityBehaviourRegistry;
use reactive_graph_plugin_api::prelude::plugin::*;
use reactive_graph_plugin_api::prelude::providers::*;

use reactive_graph_std_logical_model::BEHAVIOUR_IF_THEN_ELSE;
use reactive_graph_std_logical_model::BEHAVIOUR_TOGGLE;
use reactive_graph_std_logical_model::BEHAVIOUR_TRIGGER;
use reactive_graph_std_logical_model::ENTITY_BEHAVIOUR_IF_THEN_ELSE;
use reactive_graph_std_logical_model::ENTITY_BEHAVIOUR_TOGGLE;
use reactive_graph_std_logical_model::ENTITY_BEHAVIOUR_TRIGGER;

use crate::behaviour::entity::gate::function::LOGICAL_GATES;
use crate::behaviour::entity::if_then_else::IfThenElseFactory;
use crate::behaviour::entity::operation::function::LOGICAL_OPERATIONS;
use crate::behaviour::entity::toggle::ToggleFactory;
use crate::behaviour::entity::trigger::TriggerFactory;

export_plugin!({
    "dependencies": [
        { "name": "reactive-graph-std-base", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-std-trigger", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-std-result", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-std-connector", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait LogicalPlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct LogicalPluginImpl {
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
impl Plugin for LogicalPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;

        // If Then Else
        let factory = Arc::new(IfThenElseFactory::new(BEHAVIOUR_IF_THEN_ELSE.clone()));
        self.entity_behaviour_registry.register(ENTITY_BEHAVIOUR_IF_THEN_ELSE.clone(), factory).await;

        // Toggle
        let factory = Arc::new(ToggleFactory::new(BEHAVIOUR_TOGGLE.clone()));
        self.entity_behaviour_registry.register(ENTITY_BEHAVIOUR_TOGGLE.clone(), factory).await;

        // Trigger
        let factory = Arc::new(TriggerFactory::new(BEHAVIOUR_TRIGGER.clone()));
        self.entity_behaviour_registry.register(ENTITY_BEHAVIOUR_TRIGGER.clone(), factory).await;

        // Logical operations
        self.entity_behaviour_registry.register_all(&LOGICAL_OPERATIONS.get_factories()).await;
        self.entity_behaviour_registry.register_all(&LOGICAL_GATES.get_factories()).await;

        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.entity_behaviour_registry.unregister_all(&LOGICAL_GATES.to_entity_behaviour_tys()).await;
        self.entity_behaviour_registry
            .unregister_all(&LOGICAL_OPERATIONS.to_entity_behaviour_tys())
            .await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_TRIGGER).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_TOGGLE).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_IF_THEN_ELSE).await;

        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}
