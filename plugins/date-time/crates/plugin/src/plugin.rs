use reactive_graph_plugin_api::prelude::plugin::*;
use reactive_graph_plugin_api::prelude::providers::*;
use reactive_graph_plugin_api::EntityBehaviourRegistry;

use reactive_graph_model_date_time::BEHAVIOUR_UTC_NOW;
use reactive_graph_model_date_time::BEHAVIOUR_UTC_TIMESTAMP;
use reactive_graph_model_date_time::ENTITY_BEHAVIOUR_UTC_NOW;
use reactive_graph_model_date_time::ENTITY_BEHAVIOUR_UTC_TIMESTAMP;

use crate::api::TimeGraph;
use crate::behaviour::UtcNowFactory;
use crate::behaviour::UtcTimestampFactory;

export_plugin!({
    "dependencies": [
        { "name": "reactive-graph-plugin-base", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-plugin-result", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-plugin-trigger", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait DateTimePlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct DateTimePluginImpl {
    component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,

    #[component(default = "component_provider_registry")]
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,

    entity_types_provider: Arc<dyn TypeProvider<EntityTypes> + Send + Sync>,

    #[component(default = "entity_types_provider_registry")]
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,

    #[component(default = "entity_behaviour_registry")]
    entity_behaviour_registry: Arc<dyn EntityBehaviourRegistry + Send + Sync>,

    relation_types_provider: Arc<dyn TypeProvider<RelationTypes> + Send + Sync>,

    #[component(default = "relation_types_provider_registry")]
    relation_type_provider_registry: Arc<dyn RelationTypeProviderRegistry + Send + Sync>,

    time_graph: Arc<dyn TimeGraph + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for DateTimePluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;
        self.relation_type_provider_registry
            .register_provider(self.relation_types_provider.clone())
            .await;
        // Utc Timestamp
        let factory = Arc::new(UtcTimestampFactory::new(BEHAVIOUR_UTC_TIMESTAMP.clone()));
        self.entity_behaviour_registry.register(ENTITY_BEHAVIOUR_UTC_TIMESTAMP.clone(), factory).await;
        // Utc Now
        let factory = Arc::new(UtcNowFactory::new(BEHAVIOUR_UTC_NOW.clone()));
        self.entity_behaviour_registry.register(ENTITY_BEHAVIOUR_UTC_NOW.clone(), factory).await;

        self.time_graph.init().await;

        let time_graph = self.time_graph.clone();

        async_std::task::spawn(async move {
            time_graph.init().await;
        });

        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.time_graph.shutdown().await;

        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_UTC_TIMESTAMP).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_UTC_NOW).await;

        self.relation_type_provider_registry
            .unregister_provider(self.relation_types_provider.id())
            .await;
        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}
