use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_base::Describable;
use crate::model_base::Named;
use crate::NAMESPACE_FLOW;

entity_ty!(ENTITY_TYPE_COMMENT, NAMESPACE_FLOW, ENTITY_TYPE_NAME_COMMENT, "comment");

entity_model!(Comment);
impl Named for Comment {}
impl Describable for Comment {}
