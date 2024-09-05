use crate::Weighted;
use crate::NAMESPACE_TAXONOMY;
use reactive_graph_graph::relation_ty;
use reactive_graph_reactive_model_api::relation_model;

relation_ty!(RELATION_TYPE_HAS_SUBCATEGORY, NAMESPACE_TAXONOMY, RELATION_TYPE_NAME_HAS_SUBCATEGORY, "has_subcategory");

relation_model!(HasSubcategory);
impl Weighted for HasSubcategory {}
