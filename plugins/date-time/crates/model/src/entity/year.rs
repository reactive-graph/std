use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::NAMESPACE_DATE_TIME;

properties!(YearProperties, (YEAR, "year", 0), (LEAP, "leap", false));

entity_ty!(ENTITY_TYPE_YEAR, NAMESPACE_DATE_TIME, ENTITY_TYPE_NAME_YEAR, "year");

entity_model!(
    Year,
    get year i64,
    get leap bool
);
