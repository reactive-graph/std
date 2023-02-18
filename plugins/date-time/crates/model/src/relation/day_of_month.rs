use crate::model::properties;
use crate::model::relation_model;
use crate::model::relation_ty;
use crate::NAMESPACE_DATE_TIME;

properties!(DayOfMonthProperties, (DAY_OF_MONTH, "day_of_month", 0));

relation_ty!(RELATION_TYPE_DAY_OF_MONTH, NAMESPACE_DATE_TIME, RELATION_TYPE_NAME_DAY_OF_MONTH, "day_of_month");

relation_model!(
    DayOfMonth,
    get day_of_month u64
);
