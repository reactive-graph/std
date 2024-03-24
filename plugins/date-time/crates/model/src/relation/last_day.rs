use crate::NAMESPACE_DATE_TIME;
use inexor_rgf_graph::relation_ty;
use inexor_rgf_reactive_model_api::relation_model;

relation_ty!(RELATION_TYPE_LAST_DAY, NAMESPACE_DATE_TIME, RELATION_TYPE_NAME_LAST_DAY, "last_day");

relation_model!(LastDay);
