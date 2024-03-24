use crate::NAMESPACE_DATE_TIME;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_reactive_model_api::entity_model;

properties!(YearProperties, (YEAR, "year", 0), (LEAP, "leap", false));

entity_ty!(ENTITY_TYPE_YEAR, NAMESPACE_DATE_TIME, ENTITY_TYPE_NAME_YEAR, "year");

entity_model!(
    Year,
    get year i64,
    get leap bool
);
