use crate::NAMESPACE_TAXONOMY;
use crate::Weighted;
use reactive_graph_graph::relation_ty;
use reactive_graph_reactive_model_api::relation_model;

relation_ty!(RELATION_TYPE_TAGGED_WITH, NAMESPACE_TAXONOMY, RELATION_TYPE_NAME_TAGGED_WITH, "tagged_with");

relation_model!(TaggedWith);
impl Weighted for TaggedWith {}
