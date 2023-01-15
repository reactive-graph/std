use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_base::Describable;
use crate::model_base::Named;
use crate::NAMESPACE_TAXONOMY;

entity_ty!(ENTITY_TYPE_CATEGORY, NAMESPACE_TAXONOMY, ENTITY_TYPE_NAME_CATEGORY, "category");

entity_model!(
    Category,
    get name string,
    get description string
);
impl Named for Category {}
impl Describable for Category {}
