use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use log::error;
use reactive_graph_graph::prelude::*;
use reactive_graph_plugin_api::component_alias;
use reactive_graph_plugin_api::Component;
use reactive_graph_plugin_api::EntityInstanceManager;
use reactive_graph_plugin_api::EntityTypeManager;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_model_impl::ReactiveProperties;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_model_value::ENTITY_TYPE_VALUE_NUMBER;

use crate::api::numeric_constants_factory::NumericConstantsFactory;
use crate::constants::NUMERIC_CONSTANTS;
use crate::constants::UUID_NAMESPACE_NUMERIC_CONSTANTS;

#[derive(Component)]
pub struct NumericConstantsFactoryImpl {
    #[component(default = "crate::plugin::entity_type_manager")]
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    #[component(default = "crate::plugin::entity_instance_manager")]
    entity_instance_manager: Arc<dyn EntityInstanceManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl NumericConstantsFactory for NumericConstantsFactoryImpl {
    async fn create_numeric_constants(&self) {
        let ty = ENTITY_TYPE_VALUE_NUMBER.deref();
        let Some(entity_type) = self.entity_type_manager.get(ty) else {
            return;
        };
        for (name, value) in NUMERIC_CONSTANTS.iter() {
            let id = get_id_for_numeric_constant(name);
            if self.entity_instance_manager.has(id) {
                continue;
            }
            let mut properties = PropertyInstances::new_from_property_types_with_defaults(&entity_type.properties);
            properties.set(*name, json!(value));
            let properties = ReactiveProperties::new_with_id_from_properties(id, properties);
            let reactive_entity = ReactiveEntity::builder()
                .ty(ty)
                .id(id)
                .components(entity_type.components.clone())
                .properties(properties)
                .build();
            let reactive_entity = self.entity_instance_manager.register(reactive_entity);
            match reactive_entity {
                Ok(reactive_entity) => {
                    debug!("Created numeric constant {} {} as entity instance {}", name, value, reactive_entity.id);
                }
                Err(_) => {
                    error!("Failed to create entity instance for constant {} {}!", name, value);
                }
            }
        }
    }
}

fn get_id_for_numeric_constant(name: &str) -> Uuid {
    Uuid::new_v5(&UUID_NAMESPACE_NUMERIC_CONSTANTS, name.as_bytes())
}
