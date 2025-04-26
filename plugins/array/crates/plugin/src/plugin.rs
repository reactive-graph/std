use reactive_graph_plugin_api::EntityBehaviourRegistry;
use reactive_graph_plugin_api::prelude::plugin::*;
use reactive_graph_plugin_api::prelude::providers::*;

use crate::behaviour::entity::array_contains::ArrayContainsFactory;
use crate::behaviour::entity::array_get_by_index::ArrayGetByIndexFactory;
use crate::behaviour::entity::array_length::ArrayLengthFactory;
use crate::behaviour::entity::array_pop::ArrayPopFactory;
use crate::behaviour::entity::array_push::ArrayPushFactory;
use crate::behaviour::entity::array_reverse::ArrayReverseFactory;
use reactive_graph_std_array_model::BEHAVIOUR_ARRAY_CONTAINS;
use reactive_graph_std_array_model::BEHAVIOUR_ARRAY_GET_BY_INDEX;
use reactive_graph_std_array_model::BEHAVIOUR_ARRAY_LENGTH;
use reactive_graph_std_array_model::BEHAVIOUR_ARRAY_POP;
use reactive_graph_std_array_model::BEHAVIOUR_ARRAY_PUSH;
use reactive_graph_std_array_model::BEHAVIOUR_ARRAY_REVERSE;
use reactive_graph_std_array_model::ENTITY_BEHAVIOUR_ARRAY_CONTAINS;
use reactive_graph_std_array_model::ENTITY_BEHAVIOUR_ARRAY_GET_BY_INDEX;
use reactive_graph_std_array_model::ENTITY_BEHAVIOUR_ARRAY_LENGTH;
use reactive_graph_std_array_model::ENTITY_BEHAVIOUR_ARRAY_POP;
use reactive_graph_std_array_model::ENTITY_BEHAVIOUR_ARRAY_PUSH;
use reactive_graph_std_array_model::ENTITY_BEHAVIOUR_ARRAY_REVERSE;

export_plugin!({
    "dependencies": [
        { "name": "reactive-graph-std-base", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-std-result", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-std-trigger", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait ArrayPlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct ArrayPluginImpl {
    entity_types_provider: Arc<dyn TypeProvider<EntityTypes> + Send + Sync>,

    #[component(default = "entity_types_provider_registry")]
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,

    #[component(default = "entity_behaviour_registry")]
    entity_behaviour_registry: Arc<dyn EntityBehaviourRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for ArrayPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;

        // Array Contains
        let factory = Arc::new(ArrayContainsFactory::new(BEHAVIOUR_ARRAY_CONTAINS.clone()));
        self.entity_behaviour_registry.register(ENTITY_BEHAVIOUR_ARRAY_CONTAINS.clone(), factory).await;

        // Array Get By Index
        let factory = Arc::new(ArrayGetByIndexFactory::new(BEHAVIOUR_ARRAY_GET_BY_INDEX.clone()));
        self.entity_behaviour_registry
            .register(ENTITY_BEHAVIOUR_ARRAY_GET_BY_INDEX.clone(), factory)
            .await;

        // Array Length
        let factory = Arc::new(ArrayLengthFactory::new(BEHAVIOUR_ARRAY_LENGTH.clone()));
        self.entity_behaviour_registry.register(ENTITY_BEHAVIOUR_ARRAY_LENGTH.clone(), factory).await;

        // Array Pop
        let factory = Arc::new(ArrayPopFactory::new(BEHAVIOUR_ARRAY_POP.clone()));
        self.entity_behaviour_registry.register(ENTITY_BEHAVIOUR_ARRAY_POP.clone(), factory).await;

        // Array Push
        let factory = Arc::new(ArrayPushFactory::new(BEHAVIOUR_ARRAY_PUSH.clone()));
        self.entity_behaviour_registry.register(ENTITY_BEHAVIOUR_ARRAY_PUSH.clone(), factory).await;

        // Array Reverse
        let factory = Arc::new(ArrayReverseFactory::new(BEHAVIOUR_ARRAY_REVERSE.clone()));
        self.entity_behaviour_registry.register(ENTITY_BEHAVIOUR_ARRAY_REVERSE.clone(), factory).await;

        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_CONTAINS).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_GET_BY_INDEX).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_LENGTH).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_POP).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_PUSH).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_REVERSE).await;

        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        Ok(())
    }
}
