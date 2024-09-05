use crate::NAMESPACE_DATE_TIME;
use reactive_graph_graph::entity_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;

properties!(DayProperties, (DAY_OF_MONTH, "day_of_month", 0), (ISO8601, "iso8601", ""));

entity_ty!(ENTITY_TYPE_DAY, NAMESPACE_DATE_TIME, ENTITY_TYPE_NAME_DAY, "day");

entity_model!(
    Day,
    get day_of_month u64,
    get iso8601 string
);
