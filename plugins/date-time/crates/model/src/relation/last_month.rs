use crate::NAMESPACE_DATE_TIME;
use reactive_graph_graph::relation_ty;
use reactive_graph_reactive_model_api::relation_model;

relation_ty!(RELATION_TYPE_LAST_MONTH, NAMESPACE_DATE_TIME, RELATION_TYPE_NAME_LAST_MONTH, "last_month");

relation_model!(LastMonth);
