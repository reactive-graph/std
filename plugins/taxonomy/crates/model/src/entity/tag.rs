use inexor_rgf_reactive_api::entity_model;

use crate::NAMESPACE_TAXONOMY;
use inexor_rgf_graph::entity_ty;

entity_ty!(COMPONENT_TAG, NAMESPACE_TAXONOMY, COMPONENT_NAME_TAG, "tag");

entity_model!(
    Tag,
    get name string
);
// impl Named for Category {}
