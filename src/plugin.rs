use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use log::error;
use serde_json::json;
use uuid::Uuid;

use crate::behaviour::entity::entity_behaviour_provider::NumericEntityBehaviourProviderImpl;
use crate::builder::EntityInstanceBuilder;
use crate::constants::NAMESPACE_NUMERIC;
use crate::constants::NUMERIC_CONSTANTS;
use crate::di::*;
use crate::plugins::component_provider;
use crate::plugins::entity_behaviour_provider;
use crate::plugins::entity_type_provider;
use crate::plugins::plugin::PluginMetadata;
use crate::plugins::plugin::PluginMetadataError;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::plugin_metadata;
use crate::plugins::ComponentProvider;
use crate::plugins::ComponentProviderError;
use crate::plugins::EntityBehaviourProvider;
use crate::plugins::EntityBehaviourProviderError;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginContextInitializationError;
use crate::plugins::PluginPostInitializationError;
use crate::provider::NumericComponentProviderImpl;
use crate::provider::NumericEntityTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait NumericPlugin: Plugin + Send + Sync {}

#[module]
pub struct NumericPluginImpl {
    component_provider: Wrc<NumericComponentProviderImpl>,
    entity_type_provider: Wrc<NumericEntityTypeProviderImpl>,
    entity_behaviour_provider: Wrc<NumericEntityBehaviourProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(NumericPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl NumericPlugin for NumericPluginImpl {}

impl Plugin for NumericPluginImpl {
    fn metadata(&self) -> Result<PluginMetadata, PluginMetadataError> {
        plugin_metadata!("inexor-rgf-plugin-value")
    }

    fn post_init(&self) -> Result<(), PluginPostInitializationError> {
        self.create_numeric_constants();
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context);
        Ok(())
    }

    fn get_component_provider(&self) -> Result<Option<Arc<dyn ComponentProvider>>, ComponentProviderError> {
        component_provider!(self.component_provider)
    }

    fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
        entity_type_provider!(self.entity_type_provider)
    }

    fn get_entity_behaviour_provider(&self) -> Result<Option<Arc<dyn EntityBehaviourProvider>>, EntityBehaviourProviderError> {
        entity_behaviour_provider!(self.entity_behaviour_provider)
    }
}

impl NumericPluginImpl {
    fn create_numeric_constants(&self) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        for (name, value) in NUMERIC_CONSTANTS.iter() {
            let entity_instance = EntityInstanceBuilder::new("value", "value_number")
                .id(Uuid::new_v5(&NAMESPACE_NUMERIC, name.as_bytes()))
                .property("value", json!(value))
                .property("name", json!(name))
                .build();
            let reactive_entity_instance = entity_instance_manager.create(entity_instance);
            match reactive_entity_instance {
                Ok(reactive_entity_instance) => {
                    debug!("Created numeric constant {} {} as entity instance {}", name, value, reactive_entity_instance.id);
                }
                Err(_) => {
                    error!("Failed to create entity instance for constant {} {}!", name, value);
                }
            }
        }
    }
}
