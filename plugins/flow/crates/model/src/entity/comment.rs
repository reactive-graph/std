use inexor_rgf_graph::entity_ty;
use inexor_rgf_reactive_api::entity_model;

use inexor_rgf_model_base::Describable;
use inexor_rgf_model_base::Named;

use crate::NAMESPACE_FLOW;

entity_ty!(ENTITY_TYPE_COMMENT, NAMESPACE_FLOW, ENTITY_TYPE_NAME_COMMENT, "comment");

entity_model!(Comment);
impl Named for Comment {}
impl Describable for Comment {}
