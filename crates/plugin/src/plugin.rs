use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use inexor_rgf_model_json::BEHAVIOUR_OBJECT_GET_PROPERTY;
use inexor_rgf_model_json::BEHAVIOUR_OBJECT_KEYS;
use inexor_rgf_model_json::BEHAVIOUR_OBJECT_REMOVE_PROPERTY;
use inexor_rgf_model_json::BEHAVIOUR_OBJECT_SET_PROPERTY;
use inexor_rgf_model_json::ENTITY_BEHAVIOUR_OBJECT_GET_PROPERTY;
use inexor_rgf_model_json::ENTITY_BEHAVIOUR_OBJECT_KEYS;
use inexor_rgf_model_json::ENTITY_BEHAVIOUR_OBJECT_REMOVE_PROPERTY;
use inexor_rgf_model_json::ENTITY_BEHAVIOUR_OBJECT_SET_PROPERTY;

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
use crate::di::*;
use crate::model_json::BEHAVIOUR_ARRAY_CONTAINS;
use crate::model_json::BEHAVIOUR_ARRAY_GET_BY_INDEX;
use crate::model_json::BEHAVIOUR_ARRAY_LENGTH;
use crate::model_json::BEHAVIOUR_ARRAY_POP;
use crate::model_json::BEHAVIOUR_ARRAY_PUSH;
use crate::model_json::BEHAVIOUR_ARRAY_REVERSE;
use crate::model_json::BEHAVIOUR_LOAD_JSON;
use crate::model_json::BEHAVIOUR_SAVE_JSON;
use crate::model_json::COMPONENT_BEHAVIOUR_LOAD_JSON;
use crate::model_json::COMPONENT_BEHAVIOUR_SAVE_JSON;
use crate::model_json::ENTITY_BEHAVIOUR_ARRAY_CONTAINS;
use crate::model_json::ENTITY_BEHAVIOUR_ARRAY_GET_BY_INDEX;
use crate::model_json::ENTITY_BEHAVIOUR_ARRAY_LENGTH;
use crate::model_json::ENTITY_BEHAVIOUR_ARRAY_POP;
use crate::model_json::ENTITY_BEHAVIOUR_ARRAY_PUSH;
use crate::model_json::ENTITY_BEHAVIOUR_ARRAY_REVERSE;
use crate::plugins::component_provider;
use crate::plugins::entity_type_provider;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::ComponentProvider;
use crate::plugins::ComponentProviderError;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginActivationError;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::plugins::PluginDeactivationError;
use crate::providers::JsonComponentProviderImpl;
use crate::providers::JsonEntityTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    return PluginContextContainer(RwLock::new(None));
}

#[async_trait]
pub trait JsonPlugin: Plugin + Send + Sync {}

#[module]
pub struct JsonPluginImpl {
    component_provider: Wrc<JsonComponentProviderImpl>,
    entity_type_provider: Wrc<JsonEntityTypeProviderImpl>,
    context: PluginContextContainer,
}

impl JsonPluginImpl {}

interfaces!(JsonPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl JsonPlugin for JsonPluginImpl {}

impl Plugin for JsonPluginImpl {
    fn activate(&self) -> Result<(), PluginActivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();
            let entity_component_behaviour_registry = context.get_entity_component_behaviour_registry();

            // Load Json
            let factory = Arc::new(LoadJsonFactory::new(BEHAVIOUR_LOAD_JSON.clone()));
            entity_component_behaviour_registry.register(COMPONENT_BEHAVIOUR_LOAD_JSON.clone(), factory);

            // Save Json
            let factory = Arc::new(SaveJsonFactory::new(BEHAVIOUR_SAVE_JSON.clone()));
            entity_component_behaviour_registry.register(COMPONENT_BEHAVIOUR_SAVE_JSON.clone(), factory);

            // Array Contains
            let factory = Arc::new(ArrayContainsFactory::new(BEHAVIOUR_ARRAY_CONTAINS.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_ARRAY_CONTAINS.clone(), factory);

            // Array Get By Index
            let factory = Arc::new(ArrayGetByIndexFactory::new(BEHAVIOUR_ARRAY_GET_BY_INDEX.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_ARRAY_GET_BY_INDEX.clone(), factory);

            // Array Length
            let factory = Arc::new(ArrayLengthFactory::new(BEHAVIOUR_ARRAY_LENGTH.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_ARRAY_LENGTH.clone(), factory);

            // Array Pop
            let factory = Arc::new(ArrayPopFactory::new(BEHAVIOUR_ARRAY_POP.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_ARRAY_POP.clone(), factory);

            // Array Push
            let factory = Arc::new(ArrayPushFactory::new(BEHAVIOUR_ARRAY_PUSH.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_ARRAY_PUSH.clone(), factory);

            // Array Reverse
            let factory = Arc::new(ArrayReverseFactory::new(BEHAVIOUR_ARRAY_REVERSE.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_ARRAY_REVERSE.clone(), factory);

            // Object Get Property
            let factory = Arc::new(ObjectGetPropertyFactory::new(BEHAVIOUR_OBJECT_GET_PROPERTY.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_OBJECT_GET_PROPERTY.clone(), factory);

            // Object Keys
            let factory = Arc::new(ObjectKeysFactory::new(BEHAVIOUR_OBJECT_KEYS.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_OBJECT_KEYS.clone(), factory);

            // Object Remove Property
            let factory = Arc::new(ObjectRemovePropertyFactory::new(BEHAVIOUR_OBJECT_REMOVE_PROPERTY.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_OBJECT_REMOVE_PROPERTY.clone(), factory);

            // Object Set Property
            let factory = Arc::new(ObjectSetPropertyFactory::new(BEHAVIOUR_OBJECT_SET_PROPERTY.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_OBJECT_SET_PROPERTY.clone(), factory);
        }
        Ok(())
    }

    fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();
            let entity_component_behaviour_registry = context.get_entity_component_behaviour_registry();
            entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_LOAD_JSON);
            entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_SAVE_JSON);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_CONTAINS);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_GET_BY_INDEX);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_LENGTH);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_POP);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_PUSH);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_ARRAY_REVERSE);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_OBJECT_GET_PROPERTY);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_OBJECT_KEYS);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_OBJECT_REMOVE_PROPERTY);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_OBJECT_SET_PROPERTY);
        }
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context.clone());
        Ok(())
    }

    fn remove_context(&self) -> Result<(), PluginContextDeinitializationError> {
        let mut writer = self.context.0.write().unwrap();
        *writer = None;
        Ok(())
    }

    fn get_component_provider(&self) -> Result<Option<Arc<dyn ComponentProvider>>, ComponentProviderError> {
        component_provider!(self.component_provider)
    }

    fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
        entity_type_provider!(self.entity_type_provider)
    }
}
