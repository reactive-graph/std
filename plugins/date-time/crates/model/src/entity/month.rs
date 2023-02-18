use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::NAMESPACE_DATE_TIME;

properties!(MonthProperties, (MONTH_OF_YEAR, "month_of_year", 0), (MONTH_AND_YEAR, "month_and_year", ""));

entity_ty!(ENTITY_TYPE_MONTH, NAMESPACE_DATE_TIME, ENTITY_TYPE_NAME_MONTH, "month");

entity_model!(
    Month,
    get month_of_year u64,
    get month_and_year string
);
