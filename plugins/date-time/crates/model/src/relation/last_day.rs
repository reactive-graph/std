use crate::model::relation_model;
use crate::model::relation_ty;
use crate::NAMESPACE_DATE_TIME;

relation_ty!(RELATION_TYPE_LAST_DAY, NAMESPACE_DATE_TIME, RELATION_TYPE_NAME_LAST_DAY, "last_day");

relation_model!(LastDay);
