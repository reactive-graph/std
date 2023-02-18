use crate::model::relation_model;
use crate::model::relation_ty;
use crate::NAMESPACE_DATE_TIME;

relation_ty!(RELATION_TYPE_FIRST_DAY, NAMESPACE_DATE_TIME, RELATION_TYPE_NAME_FIRST_DAY, "first_day");

relation_model!(FirstDay);
