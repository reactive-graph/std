use crate::NAMESPACE_DATE_TIME;
use reactive_graph_graph::properties;
use reactive_graph_graph::relation_ty;
use reactive_graph_reactive_model_api::relation_model;

properties!(MonthOfYearProperties, (MONTH_OF_YEAR, "month_of_year", 0));

relation_ty!(RELATION_TYPE_MONTH_OF_YEAR, NAMESPACE_DATE_TIME, RELATION_TYPE_NAME_MONTH_OF_YEAR, "month_of_year");

relation_model!(
    MonthOfYear,
    get month_of_year u64
);
