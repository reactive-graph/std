use crate::model::relation_model;
use crate::model::relation_ty;
use crate::Weighted;
use crate::NAMESPACE_TAXONOMY;

relation_ty!(RELATION_TYPE_TAGGED_WITH, NAMESPACE_TAXONOMY, RELATION_TYPE_NAME_TAGGED_WITH, "tagged_with");

relation_model!(TaggedWith);
impl Weighted for TaggedWith {}
