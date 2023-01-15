use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::NAMESPACE_RANDOM;

properties!(
    RandomF64RangeProperties,
    (TRIGGER, "trigger", false),
    (LOW, "low", 0.0),
    (HIGH, "high", 1.0),
    (RESULT, "result", 0.0)
);

entity_ty!(ENTITY_TYPE_RANDOM_F64_RANGE, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_F64_RANGE, "random_f64_range");
behaviour_ty!(BEHAVIOUR_RANDOM_F64_RANGE, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_F64_RANGE, "random_f64_range");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_F64_RANGE, ENTITY_TYPE_RANDOM_F64_RANGE, BEHAVIOUR_RANDOM_F64_RANGE);

entity_model!(
    RandomF64Range,
    trigger,
    set low f64,
    set high f64,
    get result f64
);
