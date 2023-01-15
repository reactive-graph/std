use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::NAMESPACE_RANDOM;

properties!(RandomU64Properties, (TRIGGER, "trigger", false), (RESULT, "result", 0));

entity_ty!(ENTITY_TYPE_RANDOM_U64, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_U64, "random_u64");
behaviour_ty!(BEHAVIOUR_RANDOM_U64, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_U64, "random_u64");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_U64, ENTITY_TYPE_RANDOM_U64, BEHAVIOUR_RANDOM_U64);

entity_model!(
    RandomU64,
    trigger,
    get result u64
);
