use crate::behaviour::entity::counter::CounterFactory;
use crate::behaviour::entity::gate::function::ARITHMETIC_GATES_F64;
use crate::behaviour::entity::gate::function::ARITHMETIC_GATES_I64;
use crate::behaviour::entity::gate::function::ARITHMETIC_GATES_U64;
use crate::behaviour::entity::operation::function::ARITHMETIC_OPERATIONS_F64;
use crate::behaviour::entity::operation::function::ARITHMETIC_OPERATIONS_I64;
use crate::behaviour::entity::operation::function::ARITHMETIC_OPERATIONS_U64;
use reactive_graph_model_arithmetic::BEHAVIOUR_COUNTER;
use reactive_graph_model_arithmetic::ENTITY_BEHAVIOUR_COUNTER;
use reactive_graph_plugin_api::prelude::plugin::*;
use reactive_graph_plugin_api::prelude::providers::*;
use reactive_graph_plugin_api::EntityBehaviourRegistry;

export_plugin!({
    "dependencies": [
        { "name": "reactive-graph-plugin-base", "version": ">=0.10.0, <0.11.0" }
        { "name": "reactive-graph-plugin-trigger", "version": ">=0.10.0, <0.11.0" }
        { "name": "reactive-graph-plugin-result", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait ArithmeticPlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct ArithmeticPluginImpl {
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
impl Plugin for ArithmeticPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;
        self.entity_behaviour_registry.register_all(&ARITHMETIC_OPERATIONS_F64.get_factories()).await;
        self.entity_behaviour_registry.register_all(&ARITHMETIC_OPERATIONS_I64.get_factories()).await;
        self.entity_behaviour_registry.register_all(&ARITHMETIC_OPERATIONS_U64.get_factories()).await;
        self.entity_behaviour_registry.register_all(&ARITHMETIC_GATES_F64.get_factories()).await;
        self.entity_behaviour_registry.register_all(&ARITHMETIC_GATES_I64.get_factories()).await;
        self.entity_behaviour_registry.register_all(&ARITHMETIC_GATES_U64.get_factories()).await;
        self.entity_behaviour_registry
            .register(ENTITY_BEHAVIOUR_COUNTER.clone(), Arc::new(CounterFactory::new(BEHAVIOUR_COUNTER.clone())))
            .await;
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_COUNTER).await;
        self.entity_behaviour_registry
            .unregister_all(&ARITHMETIC_GATES_U64.to_entity_behaviour_tys())
            .await;
        self.entity_behaviour_registry
            .unregister_all(&ARITHMETIC_GATES_I64.to_entity_behaviour_tys())
            .await;
        self.entity_behaviour_registry
            .unregister_all(&ARITHMETIC_GATES_F64.to_entity_behaviour_tys())
            .await;
        self.entity_behaviour_registry
            .unregister_all(&ARITHMETIC_OPERATIONS_U64.to_entity_behaviour_tys())
            .await;
        self.entity_behaviour_registry
            .unregister_all(&ARITHMETIC_OPERATIONS_I64.to_entity_behaviour_tys())
            .await;
        self.entity_behaviour_registry
            .unregister_all(&ARITHMETIC_OPERATIONS_F64.to_entity_behaviour_tys())
            .await;
        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}
