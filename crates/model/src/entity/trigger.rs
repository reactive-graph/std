use crate::model::entity_type;
use crate::model::properties;
use crate::NAMESPACE_LOGICAL;

properties!(TriggerProperties, (PAYLOAD, "payload", 0));

entity_type!(ENTITY_TYPE_TRIGGER, NAMESPACE_LOGICAL, ENTITY_TYPE_NAME_TRIGGER, "trigger");
