use crate::behaviour::component::PropagationCounterFactory;
use inexor_rgf_model_connector::BEHAVIOUR_PROPAGATION_COUNTER;
use inexor_rgf_model_connector::COMPONENT_BEHAVIOUR_PROPAGATION_COUNTER;
use inexor_rgf_plugin_api::prelude::plugin::*;
use inexor_rgf_plugin_api::prelude::providers::*;
use inexor_rgf_plugin_api::RelationBehaviourRegistry;
use inexor_rgf_plugin_api::RelationComponentBehaviourRegistry;

use crate::behaviour::relation::complex_connector::COMPLEX_CONNECTOR_BEHAVIOURS;
use crate::behaviour::relation::connector::CONNECTOR_BEHAVIOURS;

export_plugin!({
    "dependencies": [
        { "name": "inexor-rgf-plugin-base", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait ValuePlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct ValuePluginImpl {
    component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,

    #[component(default = "component_provider_registry")]
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,

    relation_types_provider: Arc<dyn TypeProvider<RelationTypes> + Send + Sync>,

    #[component(default = "relation_types_provider_registry")]
    relation_type_provider_registry: Arc<dyn RelationTypeProviderRegistry + Send + Sync>,

    #[component(default = "relation_behaviour_registry")]
    relation_behaviour_registry: Arc<dyn RelationBehaviourRegistry + Send + Sync>,

    #[component(default = "relation_component_behaviour_registry")]
    relation_component_behaviour_registry: Arc<dyn RelationComponentBehaviourRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for ValuePluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.relation_type_provider_registry
            .register_provider(self.relation_types_provider.clone())
            .await;

        // Connector
        self.relation_behaviour_registry.register_all(&CONNECTOR_BEHAVIOURS.get_factories()).await;

        // Complex Connector
        self.relation_behaviour_registry
            .register_all(&COMPLEX_CONNECTOR_BEHAVIOURS.get_factories())
            .await;

        // PropagationCounter
        let factory = Arc::new(PropagationCounterFactory::new(BEHAVIOUR_PROPAGATION_COUNTER.clone()));
        self.relation_component_behaviour_registry
            .register(COMPONENT_BEHAVIOUR_PROPAGATION_COUNTER.clone(), factory)
            .await;

        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.relation_component_behaviour_registry
            .unregister(&COMPONENT_BEHAVIOUR_PROPAGATION_COUNTER)
            .await;

        self.relation_behaviour_registry
            .unregister_all(&COMPLEX_CONNECTOR_BEHAVIOURS.to_relation_behaviour_tys())
            .await;

        self.relation_behaviour_registry
            .unregister_all(&CONNECTOR_BEHAVIOURS.to_relation_behaviour_tys())
            .await;

        self.relation_type_provider_registry
            .unregister_provider(self.relation_types_provider.id())
            .await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}
