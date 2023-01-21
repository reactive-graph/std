use crate::model::relation_model;
use crate::model::relation_ty;
use crate::Weighted;
use crate::NAMESPACE_TAXONOMY;

relation_ty!(RELATION_TYPE_CATEGORIZED_AS, NAMESPACE_TAXONOMY, RELATION_TYPE_NAME_CATEGORIZED_AS, "categorized_as");

relation_model!(CategorizedAs);
impl Weighted for CategorizedAs {}
