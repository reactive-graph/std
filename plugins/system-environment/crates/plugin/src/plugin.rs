use std::env;
use std::sync::Arc;
use std::sync::RwLock;

use serde_json::json;
use uuid::Uuid;

use async_trait::async_trait;

use crate::builder::EntityInstanceBuilder;
use crate::di::*;
use crate::model_system_environment::ENTITY_TYPE_SYSTEM_ENV;
use crate::model_system_environment::NAMESPACE_SYSTEM_ENVIRONMENT_ID;
use crate::plugins::entity_type_provider;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginActivationError;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::providers::SystemEnvironmentEntityTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

pub trait SystemEnvironmentPlugin: Plugin + Send + Sync {}

#[module]
pub struct SystemEnvironmentPluginImpl {
    entity_type_provider: Wrc<SystemEnvironmentEntityTypeProviderImpl>,

    context: PluginContextContainer,
}

impl SystemEnvironmentPluginImpl {}

interfaces!(SystemEnvironmentPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl SystemEnvironmentPlugin for SystemEnvironmentPluginImpl {}

#[async_trait]
impl Plugin for SystemEnvironmentPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_instance_manager = context.get_entity_instance_manager();
            for (name, value) in env::vars() {
                let id = Uuid::new_v5(&NAMESPACE_SYSTEM_ENVIRONMENT_ID, name.as_bytes());
                if entity_instance_manager.has(id) {
                    continue;
                }
                let system_env = EntityInstanceBuilder::new(ENTITY_TYPE_SYSTEM_ENV.clone())
                    .id(Uuid::new_v5(&NAMESPACE_SYSTEM_ENVIRONMENT_ID, name.as_bytes()))
                    .property("name", json!(name.clone()))
                    .property("value", json!(value))
                    .property("label", json!(format!("/org/inexor/system/env/{}", name.clone().to_lowercase())))
                    .build();
                let _ = entity_instance_manager.create(system_env);
            }
        }
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context);
        Ok(())
    }

    fn remove_context(&self) -> Result<(), PluginContextDeinitializationError> {
        let mut writer = self.context.0.write().unwrap();
        *writer = None;
        Ok(())
    }

    fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
        entity_type_provider!(self.entity_type_provider)
    }
}
