use crate::model::relation_model;
use crate::model::relation_ty;
use crate::NAMESPACE_DATE_TIME;

relation_ty!(RELATION_TYPE_LAST_MONTH, NAMESPACE_DATE_TIME, RELATION_TYPE_NAME_LAST_MONTH, "last_month");

relation_model!(LastMonth);
