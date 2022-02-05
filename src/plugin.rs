use std::sync::{Arc, RwLock};

use crate::di::*;
use async_trait::async_trait;
use log::{debug, error};

use crate::behaviour::entity::entity_behaviour_provider::NumericEntityBehaviourProviderImpl;
use crate::builder::EntityInstanceBuilder;
use crate::constants::{NAMESPACE_NUMERIC, NUMERIC_CONSTANTS};
use crate::plugins::plugin::PluginMetadata;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::{
    ComponentBehaviourProvider, ComponentProvider, EntityBehaviourProvider, EntityTypeProvider, FlowProvider, Plugin, PluginError, RelationBehaviourProvider,
    RelationTypeProvider, WebResourceProvider,
};
use crate::provider::{NumericComponentProviderImpl, NumericEntityTypeProviderImpl};
use serde_json::json;
use uuid::Uuid;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    return PluginContextContainer(RwLock::new(None));
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
    fn metadata(&self) -> Result<PluginMetadata, PluginError> {
        Ok(PluginMetadata {
            name: env!("CARGO_PKG_NAME").into(),
            description: env!("CARGO_PKG_DESCRIPTION").into(),
            version: env!("CARGO_PKG_VERSION").into(),
        })
    }

    fn init(&self) -> Result<(), PluginError> {
        debug!("NumericPluginModuleImpl::init()");
        Ok(())
    }

    fn post_init(&self) -> Result<(), PluginError> {
        debug!("NumericPluginModuleImpl::post_init()");
        self.create_numeric_constants();
        Ok(())
    }

    fn pre_shutdown(&self) -> Result<(), PluginError> {
        debug!("NumericPluginModuleImpl::pre_shutdown()");
        Ok(())
    }

    fn shutdown(&self) -> Result<(), PluginError> {
        debug!("NumericPluginModuleImpl::shutdown()");
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginError> {
        self.context.0.write().unwrap().replace(context);
        Ok(())
    }

    fn get_component_provider(&self) -> Result<Arc<dyn ComponentProvider>, PluginError> {
        let component_provider = self.component_provider.clone();
        let component_provider: Result<Arc<dyn ComponentProvider>, _> = <dyn query_interface::Object>::query_arc(component_provider);
        if component_provider.is_err() {
            return Err(PluginError::NoComponentProvider);
        }
        Ok(component_provider.unwrap())
    }

    fn get_entity_type_provider(&self) -> Result<Arc<dyn EntityTypeProvider>, PluginError> {
        let entity_type_provider = self.entity_type_provider.clone();
        let entity_type_provider: Result<Arc<dyn EntityTypeProvider>, _> = <dyn query_interface::Object>::query_arc(entity_type_provider);
        if entity_type_provider.is_err() {
            return Err(PluginError::NoEntityTypeProvider);
        }
        Ok(entity_type_provider.unwrap())
    }

    fn get_relation_type_provider(&self) -> Result<Arc<dyn RelationTypeProvider>, PluginError> {
        Err(PluginError::NoRelationTypeProvider)
    }

    fn get_component_behaviour_provider(&self) -> Result<Arc<dyn ComponentBehaviourProvider>, PluginError> {
        Err(PluginError::NoComponentBehaviourProvider)
    }

    fn get_entity_behaviour_provider(&self) -> Result<Arc<dyn EntityBehaviourProvider>, PluginError> {
        let entity_behaviour_provider = self.entity_behaviour_provider.clone();
        let entity_behaviour_provider: Result<Arc<dyn EntityBehaviourProvider>, _> = <dyn query_interface::Object>::query_arc(entity_behaviour_provider);
        if entity_behaviour_provider.is_err() {
            return Err(PluginError::NoEntityBehaviourProvider);
        }
        Ok(entity_behaviour_provider.unwrap())
    }

    fn get_relation_behaviour_provider(&self) -> Result<Arc<dyn RelationBehaviourProvider>, PluginError> {
        Err(PluginError::NoRelationBehaviourProvider)
    }

    fn get_flow_provider(&self) -> Result<Arc<dyn FlowProvider>, PluginError> {
        Err(PluginError::NoFlowProvider)
    }

    fn get_web_resource_provider(&self) -> Result<Arc<dyn WebResourceProvider>, PluginError> {
        Err(PluginError::NoWebResourceProvider)
    }
}

impl NumericPluginImpl {
    fn create_numeric_constants(&self) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        for (name, value) in NUMERIC_CONSTANTS.iter() {
            let entity_instance = EntityInstanceBuilder::new("numeric_value")
                .id(Uuid::new_v5(&NAMESPACE_NUMERIC, name.as_bytes()))
                .property("value", json!(value))
                .property("name", json!(name))
                .get();
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
