use crate::NAMESPACE_DATE_TIME;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_reactive_model_api::entity_model;

properties!(MonthProperties, (MONTH_OF_YEAR, "month_of_year", 0), (MONTH_AND_YEAR, "month_and_year", ""));

entity_ty!(ENTITY_TYPE_MONTH, NAMESPACE_DATE_TIME, ENTITY_TYPE_NAME_MONTH, "month");

entity_model!(
    Month,
    get month_of_year u64,
    get month_and_year string
);
