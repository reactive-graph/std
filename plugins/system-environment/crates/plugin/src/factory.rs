use std::env;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::LazyLock;

use inexor_rgf_graph::prelude::*;
use inexor_rgf_model_runtime::LabeledProperties::LABEL;
use inexor_rgf_model_runtime::COMPONENT_LABELED;
use inexor_rgf_plugin_api::prelude::plugin::*;
use inexor_rgf_reactive::ReactiveEntity;
use inexor_rgf_reactive::ReactiveProperties;
use inexor_rgf_rt_api::ReactiveEntityRegistrationError;
use log::info;
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;

use inexor_rgf_model_base::NamedProperties::NAME;
use inexor_rgf_model_base::COMPONENT_NAMED;
use inexor_rgf_model_value::ValueProperties::VALUE;
use inexor_rgf_model_value::COMPONENT_VALUE;

use crate::model_system_environment::ENTITY_TYPE_SYSTEM_ENV_VAR;
use crate::model_system_environment::NAMESPACE_SYSTEM_ENVIRONMENT_ID;

const SYSTEM_ENV_COMPONENTS: LazyLock<ComponentTypeIds> = LazyLock::new(|| {
    ComponentTypeIds::new()
        .component(COMPONENT_LABELED.clone())
        .component(COMPONENT_VALUE.clone())
        .component(COMPONENT_NAMED.clone())
});
#[derive(Component)]
pub struct SystemEnvironmentReactiveEntityFactory {
    #[component(default = "crate::plugin::inject_plugin_context")]
    context: Arc<dyn PluginContext + Send + Sync>,
}

#[derive(Debug, Error)]
pub enum SystemEnvironmentFactoryError {
    #[error("Entity type not found")]
    EntityTypeNotFound,
    #[error("Failed to create reactive entity: {0}")]
    ReactiveEntityRegistrationError(#[from] ReactiveEntityRegistrationError),
}

impl SystemEnvironmentReactiveEntityFactory {
    pub async fn create_entity_instances(&self) -> Result<(), SystemEnvironmentFactoryError> {
        let entity_type_manager = self.context.get_entity_type_manager();
        let ty = ENTITY_TYPE_SYSTEM_ENV_VAR.deref();
        // let ty = EntityTypeId::new_from_type("asdf", "asdf");
        info!("{ty}");
        let Some(entity_type) = entity_type_manager.get(&ty) else {
            return Err(SystemEnvironmentFactoryError::EntityTypeNotFound);
        };
        let entity_instance_manager = self.context.get_entity_instance_manager();
        // let components = ComponentTypeIds::new()
        //     .component(&COMPONENT_LABELED)
        //     .component(&COMPONENT_VALUE)
        //     .component(&COMPONENT_NAMED);
        let properties = PropertyInstances::new_from_property_types_with_defaults(&entity_type.properties);
        for (name, value) in env::vars() {
            let id = Uuid::new_v5(&NAMESPACE_SYSTEM_ENVIRONMENT_ID, name.as_bytes());
            if entity_instance_manager.has(id) {
                continue;
            }
            let mut properties = properties.clone();
            properties.set(LABEL, json!(format!("/org/inexor/system/env/{}", name.clone().to_lowercase())));
            properties.set(VALUE, json!(value));
            properties.set(NAME, json!(name.clone()));
            let properties = ReactiveProperties::new_with_id_from_properties(id, properties);
            let reactive_entity = ReactiveEntity::builder()
                // let entity_instance = EntityInstance::builder()
                .ty(&entity_type.ty)
                .id(id)
                .components(SYSTEM_ENV_COMPONENTS.clone())
                .properties(properties)
                .build();
            if let Err(e) = entity_instance_manager.register(reactive_entity) {
                return Err(e.into());
            }
        }
        Ok(())
    }
}
