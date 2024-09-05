use crate::NAMESPACE_DATE_TIME;
use reactive_graph_graph::entity_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;

properties!(HourProperties, (HOUR, "hour_of_day", 0));

entity_ty!(ENTITY_TYPE_HOUR, NAMESPACE_DATE_TIME, ENTITY_TYPE_NAME_HOUR, "hour");

entity_model!(Hour, get hour_of_day u64);
