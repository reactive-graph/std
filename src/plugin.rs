use std::sync::{Arc, RwLock};

use crate::di::*;
use async_trait::async_trait;
use log::{debug, error};

use crate::plugins::plugin::PluginMetadata;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::{
    ComponentBehaviourProvider, ComponentProvider, EntityBehaviourProvider, EntityTypeProvider,
    FlowProvider, Plugin, PluginError, RelationBehaviourProvider, RelationTypeProvider,
    WebResourceProvider,
};
use crate::provider::SystemEnvironmentEntityTypeProviderImpl;
use inexor_rgf_core_builder::EntityInstanceBuilder;
use serde_json::json;
use std::env;
use uuid::Uuid;

pub static NAMESPACE_SYSTEM_ENVIRONMENT: Uuid = Uuid::from_u128(0x6ba7b8109dad11d180b400c04fd430c7);

const SYSTEM_ENV: &str = "system_env";

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<std::sync::Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait SystemEnvironmentPlugin: Plugin + Send + Sync {}

#[module]
pub struct SystemEnvironmentPluginImpl {
    entity_type_provider: Wrc<SystemEnvironmentEntityTypeProviderImpl>,

    context: PluginContextContainer,
}

impl SystemEnvironmentPluginImpl {
    fn create_system_environment_variables(&self) {
        for (name, value) in env::vars() {
            self.create_system_environment_variable(name, value);
        }
    }

    fn create_system_environment_variable(&self, name: String, value: String) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader
            .as_ref()
            .unwrap()
            .get_entity_instance_manager()
            .clone();
        // TODO: Extract constants for properties (name, value, label) and the LABEL_BASE_PATH
        let entity_instance = EntityInstanceBuilder::new(SYSTEM_ENV)
            .id(Uuid::new_v5(&NAMESPACE_SYSTEM_ENVIRONMENT, name.as_bytes()))
            .property("name", json!(name.clone()))
            .property("value", json!(value))
            .property(
                "label",
                json!(format!(
                    "/org/inexor/system/env/{}",
                    name.clone().to_lowercase()
                )),
            )
            .get();
        let reactive_entity_instance = entity_instance_manager.create(entity_instance);
        match reactive_entity_instance {
            Ok(reactive_entity_instance) => {
                debug!(
                    "Created system environment variable {} = {} as entity instance {}",
                    name, value, reactive_entity_instance.id
                );
            }
            Err(_) => {
                error!(
                    "Failed to create entity instance for system environment variable {} {}!",
                    name, value
                );
            }
        }
    }
}

interfaces!(SystemEnvironmentPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl SystemEnvironmentPlugin for SystemEnvironmentPluginImpl {}

impl Plugin for SystemEnvironmentPluginImpl {
    fn metadata(&self) -> Result<PluginMetadata, PluginError> {
        Ok(PluginMetadata {
            name: env!("CARGO_PKG_NAME").into(),
            description: env!("CARGO_PKG_DESCRIPTION").into(),
            version: env!("CARGO_PKG_VERSION").into(),
        })
    }

    fn init(&self) -> Result<(), PluginError> {
        Ok(())
    }

    fn post_init(&self) -> Result<(), PluginError> {
        self.create_system_environment_variables();
        Ok(())
    }

    fn pre_shutdown(&self) -> Result<(), PluginError> {
        Ok(())
    }

    fn shutdown(&self) -> Result<(), PluginError> {
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginError> {
        self.context.0.write().unwrap().replace(context);
        Ok(())
    }

    fn get_component_provider(&self) -> Result<Arc<dyn ComponentProvider>, PluginError> {
        Err(PluginError::NoComponentProvider)
    }

    fn get_entity_type_provider(&self) -> Result<Arc<dyn EntityTypeProvider>, PluginError> {
        let entity_type_provider = self.entity_type_provider.clone();
        let entity_type_provider: Result<Arc<dyn EntityTypeProvider>, _> =
            <dyn query_interface::Object>::query_arc(entity_type_provider);
        if entity_type_provider.is_err() {
            return Err(PluginError::NoEntityTypeProvider);
        }
        Ok(entity_type_provider.unwrap())
    }

    fn get_relation_type_provider(&self) -> Result<Arc<dyn RelationTypeProvider>, PluginError> {
        Err(PluginError::NoRelationTypeProvider)
    }

    fn get_component_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn ComponentBehaviourProvider>, PluginError> {
        Err(PluginError::NoComponentBehaviourProvider)
    }

    fn get_entity_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn EntityBehaviourProvider>, PluginError> {
        Err(PluginError::NoEntityBehaviourProvider)
    }

    fn get_relation_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn RelationBehaviourProvider>, PluginError> {
        Err(PluginError::NoRelationBehaviourProvider)
    }

    fn get_flow_provider(&self) -> Result<Arc<dyn FlowProvider>, PluginError> {
        Err(PluginError::NoFlowProvider)
    }

    fn get_web_resource_provider(&self) -> Result<Arc<dyn WebResourceProvider>, PluginError> {
        Err(PluginError::NoWebResourceProvider)
    }
}
