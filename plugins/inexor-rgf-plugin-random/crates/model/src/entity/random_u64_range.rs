use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::NAMESPACE_RANDOM;

properties!(
    RandomU64RangeProperties,
    (TRIGGER, "trigger", false),
    (LOW, "low", 0),
    (HIGH, "high", 100),
    (RESULT, "result", 0)
);

entity_ty!(ENTITY_TYPE_RANDOM_U64_RANGE, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_U64_RANGE, "random_u64_range");
behaviour_ty!(BEHAVIOUR_RANDOM_U64_RANGE, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_U64_RANGE, "random_u64_range");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_U64_RANGE, ENTITY_TYPE_RANDOM_U64_RANGE, BEHAVIOUR_RANDOM_U64_RANGE);

entity_model!(
    RandomU64Range,
    trigger,
    set low u64,
    set high u64,
    get result u64
);
