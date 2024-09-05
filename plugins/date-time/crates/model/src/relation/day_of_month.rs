use crate::NAMESPACE_DATE_TIME;
use reactive_graph_graph::properties;
use reactive_graph_graph::relation_ty;
use reactive_graph_reactive_model_api::relation_model;

properties!(DayOfMonthProperties, (DAY_OF_MONTH, "day_of_month", 0));

relation_ty!(RELATION_TYPE_DAY_OF_MONTH, NAMESPACE_DATE_TIME, RELATION_TYPE_NAME_DAY_OF_MONTH, "day_of_month");

relation_model!(
    DayOfMonth,
    get day_of_month u64
);
