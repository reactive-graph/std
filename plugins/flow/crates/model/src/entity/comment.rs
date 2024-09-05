use reactive_graph_graph::entity_ty;
use reactive_graph_reactive_model_api::entity_model;

use reactive_graph_model_base::Describable;
use reactive_graph_model_base::Named;

use crate::NAMESPACE_FLOW;

entity_ty!(ENTITY_TYPE_COMMENT, NAMESPACE_FLOW, ENTITY_TYPE_NAME_COMMENT, "comment");

entity_model!(Comment);
impl Named for Comment {}
impl Describable for Comment {}
