use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::model_result::ResultAny;
use crate::Condition;
use crate::NAMESPACE_LOGICAL;

properties!(IfThenElseProperties, (THEN_PAYLOAD, "then_payload", 0), (ELSE_PAYLOAD, "else_payload", 0));

entity_ty!(ENTITY_TYPE_IF_THEN_ELSE, NAMESPACE_LOGICAL, ENTITY_TYPE_NAME_IF_THEN_ELSE, "if_then_else");
behaviour_ty!(BEHAVIOUR_IF_THEN_ELSE, NAMESPACE_LOGICAL, BEHAVIOUR_NAME_IF_THEN_ELSE, "if_then_else");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_IF_THEN_ELSE, ENTITY_TYPE_IF_THEN_ELSE, BEHAVIOUR_IF_THEN_ELSE);

entity_model!(
    IfThenElse,
    set then_payload value,
    set else_payload value
);
impl Condition for IfThenElse {}
impl ResultAny for IfThenElse {}
