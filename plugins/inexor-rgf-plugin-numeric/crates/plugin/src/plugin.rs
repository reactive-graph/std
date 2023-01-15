use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use log::error;
use serde_json::json;
use uuid::Uuid;

use crate::behaviour::entity::gate::behaviour_f64::NumericGateF64Factory;
use crate::behaviour::entity::gate::function::NUMERIC_GATES_F64;
use crate::behaviour::entity::operation::behaviour_f64::NumericOperationF64Factory;
use crate::behaviour::entity::operation::behaviour_i64::NumericOperationI64Factory;
use crate::behaviour::entity::operation::function::NUMERIC_OPERATIONS_F64;
use crate::behaviour::entity::operation::function::NUMERIC_OPERATIONS_I64;
use crate::builder::EntityInstanceBuilder;
use crate::constants::NAMESPACE_NUMERIC;
use crate::constants::NUMERIC_CONSTANTS;
use crate::di::*;
use crate::model::EntityBehaviourTypeId;
use crate::model_base::NamedProperties;
use crate::model_value::ValueProperties;
use crate::model_value::ENTITY_TYPE_VALUE_NUMBER;
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
use crate::providers::NumericComponentProviderImpl;
use crate::providers::NumericEntityTypeProviderImpl;

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

    context: PluginContextContainer,
}

interfaces!(NumericPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl NumericPlugin for NumericPluginImpl {}

impl Plugin for NumericPluginImpl {
    fn activate(&self) -> Result<(), PluginActivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();

            // Numeric operations (f64)
            for (behaviour_ty, f) in NUMERIC_OPERATIONS_F64.iter() {
                entity_behaviour_registry
                    .register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(NumericOperationF64Factory::new(behaviour_ty.clone(), *f)));
            }

            // Numeric operations (i64)
            for (behaviour_ty, f) in NUMERIC_OPERATIONS_I64.iter() {
                entity_behaviour_registry
                    .register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(NumericOperationI64Factory::new(behaviour_ty.clone(), *f)));
            }

            // Numeric gates (f64)
            for (behaviour_ty, f) in NUMERIC_GATES_F64.iter() {
                entity_behaviour_registry.register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(NumericGateF64Factory::new(behaviour_ty.clone(), *f)));
            }
        }
        self.create_numeric_constants();
        Ok(())
    }

    fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        // self.delete_numeric_constants();
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();
            for behaviour_ty in NUMERIC_OPERATIONS_F64.keys() {
                entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
            }
            for behaviour_ty in NUMERIC_OPERATIONS_I64.keys() {
                entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
            }
            for behaviour_ty in NUMERIC_GATES_F64.keys() {
                entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
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

    fn get_component_provider(&self) -> Result<Option<Arc<dyn ComponentProvider>>, ComponentProviderError> {
        component_provider!(self.component_provider)
    }

    fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
        entity_type_provider!(self.entity_type_provider)
    }
}

impl NumericPluginImpl {
    fn create_numeric_constants(&self) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        for (name, value) in NUMERIC_CONSTANTS.iter() {
            let id = get_id_for_numeric_constant(name);
            if entity_instance_manager.has(id) {
                continue;
            }
            let entity_instance = EntityInstanceBuilder::new(ENTITY_TYPE_VALUE_NUMBER.clone())
                .id(id)
                .property(ValueProperties::VALUE, json!(value))
                .property(NamedProperties::NAME, json!(name))
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

    // fn delete_numeric_constants(&self) {
    //     let reader = self.context.0.read().unwrap();
    //     let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
    //     for (name, _) in NUMERIC_CONSTANTS.iter() {
    //         entity_instance_manager.delete(get_id_for_numeric_constant(name));
    //     }
    // }
}

fn get_id_for_numeric_constant(name: &str) -> Uuid {
    Uuid::new_v5(&NAMESPACE_NUMERIC, name.as_bytes())
}
