use crate::NAMESPACE_TAXONOMY;
use reactive_graph_graph::entity_ty;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_std_base_model::Describable;
use reactive_graph_std_base_model::Named;

entity_ty!(ENTITY_TYPE_CATEGORY, NAMESPACE_TAXONOMY, ENTITY_TYPE_NAME_CATEGORY, "category");

entity_model!(
    Category,
    get name string,
    get description string
);
impl Named for Category {}
impl Describable for Category {}
