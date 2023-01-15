use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::NAMESPACE_RANDOM;

properties!(
    RandomI64RangeProperties,
    (TRIGGER, "trigger", false),
    (LOW, "low", -100),
    (HIGH, "high", 100),
    (RESULT, "result", 0)
);

entity_ty!(ENTITY_TYPE_RANDOM_I64_RANGE, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_I64_RANGE, "random_i64_range");
behaviour_ty!(BEHAVIOUR_RANDOM_I64_RANGE, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_I64_RANGE, "random_i64_range");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_I64_RANGE, ENTITY_TYPE_RANDOM_I64_RANGE, BEHAVIOUR_RANDOM_I64_RANGE);

entity_model!(
    RandomI64Range,
    trigger,
    set low i64,
    set high i64,
    get result i64
);
