use crate::NAMESPACE_DATE_TIME;
use inexor_rgf_graph::relation_ty;
use inexor_rgf_reactive_api::relation_model;

relation_ty!(RELATION_TYPE_NEXT_YEAR, NAMESPACE_DATE_TIME, RELATION_TYPE_NAME_NEXT_YEAR, "next_year");

relation_model!(NextYear);
