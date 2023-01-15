use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::NAMESPACE_RANDOM;

properties!(RandomBoolProperties, (TRIGGER, "trigger", false), (RESULT, "result", false));

entity_ty!(ENTITY_TYPE_RANDOM_BOOL, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_BOOL, "random_bool");
behaviour_ty!(BEHAVIOUR_RANDOM_BOOL, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_BOOL, "random_bool");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_BOOL, ENTITY_TYPE_RANDOM_BOOL, BEHAVIOUR_RANDOM_BOOL);

entity_model!(
    RandomBool,
    trigger,
    get result bool
);
