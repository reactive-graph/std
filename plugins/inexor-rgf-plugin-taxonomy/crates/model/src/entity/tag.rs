use crate::model::entity_model;
use crate::model::entity_ty;
use crate::NAMESPACE_TAXONOMY;

entity_ty!(COMPONENT_TAG, NAMESPACE_TAXONOMY, COMPONENT_NAME_TAG, "tag");

entity_model!(
    Tag,
    get name string
);
// impl Named for Category {}
