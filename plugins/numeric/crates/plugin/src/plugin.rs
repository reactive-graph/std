use crate::api::numeric_constants_factory::NumericConstantsFactory;
use crate::behaviour::entity::gate::function::NUMERIC_GATES_F64;
use crate::behaviour::entity::operation::function::NUMERIC_OPERATIONS_F64;
use crate::behaviour::entity::operation::function::NUMERIC_OPERATIONS_I64;
use inexor_rgf_plugin_api::prelude::plugin::*;
use inexor_rgf_plugin_api::prelude::providers::*;
use inexor_rgf_plugin_api::EntityBehaviourRegistry;

export_plugin!({
    "dependencies": [
        { "name": "inexor-rgf-plugin-base", "version": ">=0.10.0, <0.11.0" }
        { "name": "inexor-rgf-plugin-trigger", "version": ">=0.10.0, <0.11.0" }
        { "name": "inexor-rgf-plugin-result", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait NumericPlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct NumericPluginImpl {
    component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,

    #[component(default = "component_provider_registry")]
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,

    entity_types_provider: Arc<dyn TypeProvider<EntityTypes> + Send + Sync>,

    #[component(default = "entity_types_provider_registry")]
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,

    #[component(default = "entity_behaviour_registry")]
    entity_behaviour_registry: Arc<dyn EntityBehaviourRegistry + Send + Sync>,

    numeric_constants_factory: Arc<dyn NumericConstantsFactory + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for NumericPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;
        self.entity_behaviour_registry.register_all(&NUMERIC_OPERATIONS_F64.get_factories()).await;
        self.entity_behaviour_registry.register_all(&NUMERIC_OPERATIONS_I64.get_factories()).await;
        self.entity_behaviour_registry.register_all(&NUMERIC_GATES_F64.get_factories()).await;
        self.numeric_constants_factory.create_numeric_constants().await;
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.entity_behaviour_registry
            .unregister_all(&NUMERIC_GATES_F64.to_entity_behaviour_tys())
            .await;
        self.entity_behaviour_registry
            .unregister_all(&NUMERIC_OPERATIONS_I64.to_entity_behaviour_tys())
            .await;
        self.entity_behaviour_registry
            .unregister_all(&NUMERIC_OPERATIONS_F64.to_entity_behaviour_tys())
            .await;
        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}
