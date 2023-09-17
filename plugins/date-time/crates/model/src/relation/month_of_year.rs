use crate::NAMESPACE_DATE_TIME;
use inexor_rgf_graph::properties;
use inexor_rgf_graph::relation_ty;
use inexor_rgf_reactive_api::relation_model;

properties!(MonthOfYearProperties, (MONTH_OF_YEAR, "month_of_year", 0));

relation_ty!(RELATION_TYPE_MONTH_OF_YEAR, NAMESPACE_DATE_TIME, RELATION_TYPE_NAME_MONTH_OF_YEAR, "month_of_year");

relation_model!(
    MonthOfYear,
    get month_of_year u64
);
