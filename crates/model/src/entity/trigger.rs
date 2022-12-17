use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_ty;
use crate::model::properties;
use crate::NAMESPACE_LOGICAL;

properties!(TriggerProperties, (PAYLOAD, "payload", 0));

entity_ty!(ENTITY_TYPE_TRIGGER, NAMESPACE_LOGICAL, ENTITY_TYPE_NAME_TRIGGER, "trigger");
behaviour_ty!(BEHAVIOUR_TRIGGER, NAMESPACE_LOGICAL, BEHAVIOUR_NAME_TRIGGER, "trigger");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_TRIGGER, ENTITY_TYPE_TRIGGER, BEHAVIOUR_TRIGGER);
