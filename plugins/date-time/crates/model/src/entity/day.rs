use crate::NAMESPACE_DATE_TIME;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_reactive_api::entity_model;

properties!(DayProperties, (DAY_OF_MONTH, "day_of_month", 0), (ISO8601, "iso8601", ""));

entity_ty!(ENTITY_TYPE_DAY, NAMESPACE_DATE_TIME, ENTITY_TYPE_NAME_DAY, "day");

entity_model!(
    Day,
    get day_of_month u64,
    get iso8601 string
);
