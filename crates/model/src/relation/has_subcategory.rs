use crate::model::relation_model;
use crate::model::relation_ty;
use crate::Weighted;
use crate::NAMESPACE_TAXONOMY;

relation_ty!(RELATION_TYPE_HAS_SUBCATEGORY, NAMESPACE_TAXONOMY, RELATION_TYPE_NAME_HAS_SUBCATEGORY, "has_subcategory");

relation_model!(HasSubcategory);
impl Weighted for HasSubcategory {}
