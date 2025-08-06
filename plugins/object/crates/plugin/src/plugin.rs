use reactive_graph_plugin_api::EntityBehaviourRegistry;
use reactive_graph_plugin_api::prelude::plugin::*;
use reactive_graph_plugin_api::prelude::providers::*;

use crate::behaviour::entity::get_property::ObjectGetPropertyFactory;
use crate::behaviour::entity::keys::ObjectKeysFactory;
use crate::behaviour::entity::remove_property::ObjectRemovePropertyFactory;
use crate::behaviour::entity::set_property::ObjectSetPropertyFactory;
use reactive_graph_std_object_model::BEHAVIOUR_OBJECT_GET_PROPERTY;
use reactive_graph_std_object_model::BEHAVIOUR_OBJECT_KEYS;
use reactive_graph_std_object_model::BEHAVIOUR_OBJECT_REMOVE_PROPERTY;
use reactive_graph_std_object_model::BEHAVIOUR_OBJECT_SET_PROPERTY;
use reactive_graph_std_object_model::ENTITY_BEHAVIOUR_OBJECT_GET_PROPERTY;
use reactive_graph_std_object_model::ENTITY_BEHAVIOUR_OBJECT_KEYS;
use reactive_graph_std_object_model::ENTITY_BEHAVIOUR_OBJECT_REMOVE_PROPERTY;
use reactive_graph_std_object_model::ENTITY_BEHAVIOUR_OBJECT_SET_PROPERTY;

export_plugin!({
    "dependencies": [
        { "name": "reactive-graph-std-base", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-std-result", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-std-trigger", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait ObjectPlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct ObjectPluginImpl {
    entity_types_provider: Arc<dyn TypeProvider<EntityTypes> + Send + Sync>,

    #[component(default = "entity_types_provider_registry")]
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,

    #[component(default = "entity_behaviour_registry")]
    entity_behaviour_registry: Arc<dyn EntityBehaviourRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for ObjectPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;

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
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_OBJECT_GET_PROPERTY).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_OBJECT_KEYS).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_OBJECT_REMOVE_PROPERTY).await;
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_OBJECT_SET_PROPERTY).await;

        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        Ok(())
    }
}
