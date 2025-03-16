use crate::behaviour::entity::random_bool::RandomBoolFactory;
use crate::behaviour::entity::random_f64::RandomF64Factory;
use crate::behaviour::entity::random_f64_pseudo::RandomF64PseudoFactory;
use crate::behaviour::entity::random_f64_range::RandomF64RangeFactory;
use crate::behaviour::entity::random_i64::RandomI64Factory;
use crate::behaviour::entity::random_i64_pseudo::RandomI64PseudoFactory;
use crate::behaviour::entity::random_i64_range::RandomI64RangeFactory;
use crate::behaviour::entity::random_string::RandomStringFactory;
use crate::behaviour::entity::random_u64::RandomU64Factory;
use crate::behaviour::entity::random_u64_pseudo::RandomU64PseudoFactory;
use crate::behaviour::entity::random_u64_range::RandomU64RangeFactory;
use crate::behaviour::entity::random_uuid::RandomUuidFactory;
use reactive_graph_model_random::BEHAVIOUR_RANDOM_BOOL;
use reactive_graph_model_random::BEHAVIOUR_RANDOM_F64;
use reactive_graph_model_random::BEHAVIOUR_RANDOM_F64_PSEUDO;
use reactive_graph_model_random::BEHAVIOUR_RANDOM_F64_RANGE;
use reactive_graph_model_random::BEHAVIOUR_RANDOM_I64;
use reactive_graph_model_random::BEHAVIOUR_RANDOM_I64_PSEUDO;
use reactive_graph_model_random::BEHAVIOUR_RANDOM_I64_RANGE;
use reactive_graph_model_random::BEHAVIOUR_RANDOM_STRING;
use reactive_graph_model_random::BEHAVIOUR_RANDOM_U64;
use reactive_graph_model_random::BEHAVIOUR_RANDOM_U64_PSEUDO;
use reactive_graph_model_random::BEHAVIOUR_RANDOM_U64_RANGE;
use reactive_graph_model_random::BEHAVIOUR_RANDOM_UUID;
use reactive_graph_model_random::ENTITY_BEHAVIOUR_RANDOM_BOOL;
use reactive_graph_model_random::ENTITY_BEHAVIOUR_RANDOM_F64;
use reactive_graph_model_random::ENTITY_BEHAVIOUR_RANDOM_F64_PSEUDO;
use reactive_graph_model_random::ENTITY_BEHAVIOUR_RANDOM_F64_RANGE;
use reactive_graph_model_random::ENTITY_BEHAVIOUR_RANDOM_I64;
use reactive_graph_model_random::ENTITY_BEHAVIOUR_RANDOM_I64_PSEUDO;
use reactive_graph_model_random::ENTITY_BEHAVIOUR_RANDOM_I64_RANGE;
use reactive_graph_model_random::ENTITY_BEHAVIOUR_RANDOM_STRING;
use reactive_graph_model_random::ENTITY_BEHAVIOUR_RANDOM_U64;
use reactive_graph_model_random::ENTITY_BEHAVIOUR_RANDOM_U64_PSEUDO;
use reactive_graph_model_random::ENTITY_BEHAVIOUR_RANDOM_U64_RANGE;
use reactive_graph_model_random::ENTITY_BEHAVIOUR_RANDOM_UUID;
use reactive_graph_plugin_api::EntityBehaviourRegistry;
use reactive_graph_plugin_api::prelude::plugin::*;
use reactive_graph_plugin_api::prelude::providers::*;

export_plugin!({
    "dependencies": [
        { "name": "reactive-graph-plugin-base", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-plugin-result", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-plugin-trigger", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait RandomPlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct RandomPluginImpl {
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
impl Plugin for RandomPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;
        self.entity_behaviour_registry
            .register(ENTITY_BEHAVIOUR_RANDOM_BOOL.clone(), Arc::new(RandomBoolFactory::new(BEHAVIOUR_RANDOM_BOOL.clone())))
            .await;
        self.entity_behaviour_registry
            .register(ENTITY_BEHAVIOUR_RANDOM_F64.clone(), Arc::new(RandomF64Factory::new(BEHAVIOUR_RANDOM_F64.clone())))
            .await;
        self.entity_behaviour_registry
            .register(
                ENTITY_BEHAVIOUR_RANDOM_F64_PSEUDO.clone(),
                Arc::new(RandomF64PseudoFactory::new(BEHAVIOUR_RANDOM_F64_PSEUDO.clone())),
            )
            .await;
        self.entity_behaviour_registry
            .register(
                ENTITY_BEHAVIOUR_RANDOM_F64_RANGE.clone(),
                Arc::new(RandomF64RangeFactory::new(BEHAVIOUR_RANDOM_F64_RANGE.clone())),
            )
            .await;
        self.entity_behaviour_registry
            .register(ENTITY_BEHAVIOUR_RANDOM_I64.clone(), Arc::new(RandomI64Factory::new(BEHAVIOUR_RANDOM_I64.clone())))
            .await;
        self.entity_behaviour_registry
            .register(
                ENTITY_BEHAVIOUR_RANDOM_I64_PSEUDO.clone(),
                Arc::new(RandomI64PseudoFactory::new(BEHAVIOUR_RANDOM_I64_PSEUDO.clone())),
            )
            .await;
        self.entity_behaviour_registry
            .register(
                ENTITY_BEHAVIOUR_RANDOM_I64_RANGE.clone(),
                Arc::new(RandomI64RangeFactory::new(BEHAVIOUR_RANDOM_I64_RANGE.clone())),
            )
            .await;
        self.entity_behaviour_registry
            .register(ENTITY_BEHAVIOUR_RANDOM_STRING.clone(), Arc::new(RandomStringFactory::new(BEHAVIOUR_RANDOM_STRING.clone())))
            .await;
        self.entity_behaviour_registry
            .register(ENTITY_BEHAVIOUR_RANDOM_U64.clone(), Arc::new(RandomU64Factory::new(BEHAVIOUR_RANDOM_U64.clone())))
            .await;
        self.entity_behaviour_registry
            .register(
                ENTITY_BEHAVIOUR_RANDOM_U64_PSEUDO.clone(),
                Arc::new(RandomU64PseudoFactory::new(BEHAVIOUR_RANDOM_U64_PSEUDO.clone())),
            )
            .await;
        self.entity_behaviour_registry
            .register(
                ENTITY_BEHAVIOUR_RANDOM_U64_RANGE.clone(),
                Arc::new(RandomU64RangeFactory::new(BEHAVIOUR_RANDOM_U64_RANGE.clone())),
            )
            .await;
        self.entity_behaviour_registry
            .register(ENTITY_BEHAVIOUR_RANDOM_UUID.clone(), Arc::new(RandomUuidFactory::new(BEHAVIOUR_RANDOM_UUID.clone())))
            .await;
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_BOOL).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_F64).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_F64_PSEUDO).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_F64_RANGE).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_I64).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_I64_PSEUDO).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_I64_RANGE).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_STRING).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_U64).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_U64_PSEUDO).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_U64_RANGE).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_UUID).await;

        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_BOOL).await;
        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}
