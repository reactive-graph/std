use std::sync::Arc;

use inexor_rgf_core_builder::ReactiveRelationInstanceBuilder;
use serde_json::json;

use crate::model::ReactiveEntityInstance;
use crate::model::ReactiveRelationInstance;
use crate::model::RelationInstanceTypeId;
use crate::model::RelationTypeId;
use crate::model_connector::ConnectorProperties::INBOUND_PROPERTY_NAME;
use crate::model_connector::ConnectorProperties::OUTBOUND_PROPERTY_NAME;

pub fn create_connector<S: Into<String>>(
    outbound: Arc<ReactiveEntityInstance>,
    ty: RelationTypeId,
    inbound: Arc<ReactiveEntityInstance>,
    outbound_property_name: S,
    inbound_property_name: S,
) -> Arc<ReactiveRelationInstance> {
    let outbound_property_name = outbound_property_name.into();
    let inbound_property_name = inbound_property_name.into();
    let instance_id = format!("{}__{}", &outbound_property_name, &inbound_property_name);
    let ty = RelationInstanceTypeId::new_unique_for_instance_id(ty, instance_id);
    ReactiveRelationInstanceBuilder::new(outbound, ty, inbound)
        .property(OUTBOUND_PROPERTY_NAME, json!(outbound_property_name))
        .property(INBOUND_PROPERTY_NAME, json!(inbound_property_name))
        .build()
}
