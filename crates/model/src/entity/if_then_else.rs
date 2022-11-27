use crate::model::entity_type;
use crate::model::properties;
use crate::NAMESPACE_LOGICAL;

properties!(IfThenElseProperties, (THEN_PAYLOAD, "then_payload", 0), (ELSE_PAYLOAD, "else_payload", 0));

entity_type!(ENTITY_TYPE_IF_THEN_ELSE, NAMESPACE_LOGICAL, ENTITY_TYPE_NAME_IF_THEN_ELSE, "if_then_else");
