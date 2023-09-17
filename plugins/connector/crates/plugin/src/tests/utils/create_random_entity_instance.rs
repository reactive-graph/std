use inexor_rgf_core_builder::ReactiveEntityInstanceBuilder;
use std::sync::Arc;

use serde_json::json;

use crate::tests::utils::r_string;
use inexor_rgf_graph::ReactiveEntityInstance;

pub fn create_random_entity_instance<S: Into<String>>(property_name: S) -> Arc<ReactiveEntityInstance> {
    ReactiveEntityInstanceBuilder::new_from_type(r_string(), r_string())
        .property(property_name, json!(r_string()))
        .build()
}
