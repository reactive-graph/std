use crate::NAMESPACE_TAXONOMY;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_base::Describable;
use inexor_rgf_model_base::Named;
use inexor_rgf_reactive_model_api::entity_model;

entity_ty!(ENTITY_TYPE_CATEGORY, NAMESPACE_TAXONOMY, ENTITY_TYPE_NAME_CATEGORY, "category");

entity_model!(
    Category,
    get name string,
    get description string
);
impl Named for Category {}
impl Describable for Category {}
