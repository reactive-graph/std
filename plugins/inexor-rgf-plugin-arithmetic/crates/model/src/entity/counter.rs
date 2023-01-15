use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::NAMESPACE_ARITHMETIC_U64;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_COUNTER, NAMESPACE_ARITHMETIC_U64, ENTITY_TYPE_NAME_COUNTER, "counter");
behaviour_ty!(BEHAVIOUR_COUNTER, NAMESPACE_ARITHMETIC_U64, BEHAVIOUR_NAME_COUNTER, "counter");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_COUNTER, ENTITY_TYPE_COUNTER, BEHAVIOUR_COUNTER);

entity_model!(
    Counter,
    get result u64,
    set trigger bool,
);
