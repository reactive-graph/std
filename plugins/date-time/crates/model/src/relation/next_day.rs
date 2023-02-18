use crate::model::relation_model;
use crate::model::relation_ty;
use crate::NAMESPACE_DATE_TIME;

relation_ty!(RELATION_TYPE_NEXT_DAY, NAMESPACE_DATE_TIME, RELATION_TYPE_NAME_NEXT_DAY, "next_day");

relation_model!(NextDay);
