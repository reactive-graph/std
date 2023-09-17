use inexor_rgf_plugin_api::prelude::plugin::*;
use inexor_rgf_plugin_api::prelude::providers::*;
use inexor_rgf_plugin_api::EntityBehaviourRegistry;
use inexor_rgf_plugin_api::EntityComponentBehaviourRegistry;

use crate::behaviour::component::load_json::LoadJsonFactory;
use crate::behaviour::component::save_json::SaveJsonFactory;
use crate::behaviour::entity::array_contains::ArrayContainsFactory;
use crate::behaviour::entity::array_get_by_index::ArrayGetByIndexFactory;
use crate::behaviour::entity::array_length::ArrayLengthFactory;
use crate::behaviour::entity::array_pop::ArrayPopFactory;
use crate::behaviour::entity::array_push::ArrayPushFactory;
use crate::behaviour::entity::array_reverse::ArrayReverseFactory;
use crate::behaviour::entity::object_get_property::ObjectGetPropertyFactory;
use crate::behaviour::entity::object_keys::ObjectKeysFactory;
use crate::behaviour::entity::object_remove_property::ObjectRemovePropertyFactory;
use crate::behaviour::entity::object_set_property::ObjectSetPropertyFactory;
use inexor_rgf_model_json::BEHAVIOUR_ARRAY_CONTAINS;
use inexor_rgf_model_json::BEHAVIOUR_ARRAY_GET_BY_INDEX;
use inexor_rgf_model_json::BEHAVIOUR_ARRAY_LENGTH;
use inexor_rgf_model_json::BEHAVIOUR_ARRAY_POP;
use inexor_rgf_model_json::BEHAVIOUR_ARRAY_PUSH;
use inexor_rgf_model_json::BEHAVIOUR_ARRAY_REVERSE;
use inexor_rgf_model_json::BEHAVIOUR_LOAD_JSON;
use inexor_rgf_model_json::BEHAVIOUR_OBJECT_GET_PROPERTY;
use inexor_rgf_model_json::BEHAVIOUR_OBJECT_KEYS;
use inexor_rgf_model_json::BEHAVIOUR_OBJECT_REMOVE_PROPERTY;
use inexor_rgf_model_json::BEHAVIOUR_OBJECT_SET_PROPERTY;
use inexor_rgf_model_json::BEHAVIOUR_SAVE_JSON;
use inexor_rgf_model_json::COMPONENT_BEHAVIOUR_LOAD_JSON;
use inexor_rgf_model_json::COMPONENT_BEHAVIOUR_SAVE_JSON;
use inexor_rgf_model_json::ENTITY_BEHAVIOUR_ARRAY_CONTAINS;
use inexor_rgf_model_json::ENTITY_BEHAVIOUR_ARRAY_GET_BY_INDEX;
use inexor_rgf_model_json::ENTITY_BEHAVIOUR_ARRAY_LENGTH;
use inexor_rgf_model_json::ENTITY_BEHAVIOUR_ARRAY_POP;
use inexor_rgf_model_json::ENTITY_BEHAVIOUR_ARRAY_PUSH;
use inexor_rgf_model_json::ENTITY_BEHAVIOUR_ARRAY_REVERSE;
use inexor_rgf_model_json::ENTITY_BEHAVIOUR_OBJECT_GET_PROPERTY;
use inexor_rgf_model_json::ENTITY_BEHAVIOUR_OBJECT_KEYS;
use inexor_rgf_model_json::ENTITY_BEHAVIOUR_OBJECT_REMOVE_PROPERTY;
use inexor_rgf_model_json::ENTITY_BEHAVIOUR_OBJECT_SET_PROPERTY;

export_plugin!({
    "dependencies": [
        { "name": "inexor-rgf-plugin-base", "version": ">=0.10.0, <0.11.0" },
        { "name": "inexor-rgf-plugin-file", "version": ">=0.10.0, <0.11.0" },
        { "name": "inexor-rgf-plugin-result", "version": ">=0.10.0, <0.11.0" },
        { "name": "inexor-rgf-plugin-trigger", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait JsonPlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct JsonPluginImpl {
    component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,

    #[component(default = "component_provider_registry")]
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,

    entity_types_provider: Arc<dyn TypeProvider<EntityTypes> + Send + Sync>,

    #[component(default = "entity_types_provider_registry")]
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,

    #[component(default = "entity_component_behaviour_registry")]
    entity_component_behaviour_registry: Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>,

    #[component(default = "entity_behaviour_registry")]
    entity_behaviour_registry: Arc<dyn EntityBehaviourRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for JsonPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;

        // Load Json
        let factory = Arc::new(LoadJsonFactory::new(BEHAVIOUR_LOAD_JSON.clone()));
        self.entity_component_behaviour_registry
            .register(COMPONENT_BEHAVIOUR_LOAD_JSON.clone(), factory)
            .await;

        // Save Json
        let factory = Arc::new(SaveJsonFactory::new(BEHAVIOUR_SAVE_JSON.clone()));
        self.entity_component_behaviour_registry
            .register(COMPONENT_BEHAVIOUR_SAVE_JSON.clone(), factory)
            .await;

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

        // Object Get Property
        let factory = Arc::new(ObjectGetPropertyFactory::new(BEHAVIOUR_OBJECT_GET_PROPERTY.clone()));
        self.entity_behaviour_registry
            .register(ENTITY_BEHAVIOUR_OBJECT_GET_PROPERTY.clone(), factory)
            .await;

        // Object Keys
        let factory = Arc::new(ObjectKeysFactory::new(BEHAVIOUR_OBJECT_KEYS.clone()));
        self.entity_behaviour_registry.register(ENTITY_BEHAVIOUR_OBJECT_KEYS.clone(), factory).await;

        // Object Remove Property
        let factory = Arc::new(ObjectRemovePropertyFactory::new(BEHAVIOUR_OBJECT_REMOVE_PROPERTY.clone()));
        self.entity_behaviour_registry
            .register(ENTITY_BEHAVIOUR_OBJECT_REMOVE_PROPERTY.clone(), factory)
            .await;

        // Object Set Property
        let factory = Arc::new(ObjectSetPropertyFactory::new(BEHAVIOUR_OBJECT_SET_PROPERTY.clone()));
        self.entity_behaviour_registry
            .register(ENTITY_BEHAVIOUR_OBJECT_SET_PROPERTY.clone(), factory)
            .await;

        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_LOAD_JSON).await;
        self.entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_SAVE_JSON).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_CONTAINS).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_GET_BY_INDEX).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_LENGTH).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_POP).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_PUSH).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_REVERSE).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_OBJECT_GET_PROPERTY).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_OBJECT_KEYS).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_OBJECT_REMOVE_PROPERTY).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_OBJECT_SET_PROPERTY).await;

        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}
