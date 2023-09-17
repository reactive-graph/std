use crate::Weighted;
use crate::NAMESPACE_TAXONOMY;
use inexor_rgf_graph::relation_ty;
use inexor_rgf_reactive_api::relation_model;

relation_ty!(RELATION_TYPE_CATEGORIZED_AS, NAMESPACE_TAXONOMY, RELATION_TYPE_NAME_CATEGORIZED_AS, "categorized_as");

relation_model!(CategorizedAs);
impl Weighted for CategorizedAs {}
