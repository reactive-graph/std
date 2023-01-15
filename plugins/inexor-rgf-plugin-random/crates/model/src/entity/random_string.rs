use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::NAMESPACE_RANDOM;

properties!(RandomStringProperties, (TRIGGER, "trigger", false), (LENGTH, "length", 10), (RESULT, "result", ""));

entity_ty!(ENTITY_TYPE_RANDOM_STRING, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_STRING, "random_string");
behaviour_ty!(BEHAVIOUR_RANDOM_STRING, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_STRING, "random_string");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_STRING, ENTITY_TYPE_RANDOM_STRING, BEHAVIOUR_RANDOM_STRING);

entity_model!(
    RandomString,
    trigger,
    set length u64,
    get result string
);
