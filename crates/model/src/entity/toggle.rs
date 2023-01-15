use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::NAMESPACE_LOGICAL;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_TOGGLE, NAMESPACE_LOGICAL, ENTITY_TYPE_NAME_TOGGLE, "toggle");
behaviour_ty!(BEHAVIOUR_TOGGLE, NAMESPACE_LOGICAL, BEHAVIOUR_NAME_TOGGLE, "toggle");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_TOGGLE, ENTITY_TYPE_TOGGLE, BEHAVIOUR_TOGGLE);

entity_model!(
    Toggle,
    trigger,
    get result bool
);
