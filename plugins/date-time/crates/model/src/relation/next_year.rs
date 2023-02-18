use crate::model::relation_model;
use crate::model::relation_ty;
use crate::NAMESPACE_DATE_TIME;

relation_ty!(RELATION_TYPE_NEXT_YEAR, NAMESPACE_DATE_TIME, RELATION_TYPE_NAME_NEXT_YEAR, "next_year");

relation_model!(NextYear);
