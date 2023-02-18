use crate::model::properties;
use crate::model::relation_model;
use crate::model::relation_ty;
use crate::NAMESPACE_DATE_TIME;

properties!(MonthOfYearProperties, (MONTH_OF_YEAR, "month_of_year", 0));

relation_ty!(RELATION_TYPE_MONTH_OF_YEAR, NAMESPACE_DATE_TIME, RELATION_TYPE_NAME_MONTH_OF_YEAR, "month_of_year");

relation_model!(
    MonthOfYear,
    get month_of_year u64
);
