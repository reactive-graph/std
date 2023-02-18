use crate::model::relation_model;
use crate::model::relation_ty;
use crate::NAMESPACE_DATE_TIME;

relation_ty!(RELATION_TYPE_NEXT_MONTH, NAMESPACE_DATE_TIME, RELATION_TYPE_NAME_NEXT_MONTH, "next_month");

relation_model!(NextMonth);
