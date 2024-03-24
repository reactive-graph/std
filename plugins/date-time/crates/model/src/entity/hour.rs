use crate::NAMESPACE_DATE_TIME;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_reactive_model_api::entity_model;

properties!(HourProperties, (HOUR, "hour_of_day", 0));

entity_ty!(ENTITY_TYPE_HOUR, NAMESPACE_DATE_TIME, ENTITY_TYPE_NAME_HOUR, "hour");

entity_model!(Hour, get hour_of_day u64);
