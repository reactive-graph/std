use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::model_result::ResultString;
use crate::model_runtime::Action;
use crate::NAMESPACE_RANDOM;

properties!(RandomStringProperties, (LENGTH, "length", 10));

entity_ty!(ENTITY_TYPE_RANDOM_STRING, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_STRING, "random_string");
behaviour_ty!(BEHAVIOUR_RANDOM_STRING, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_STRING, "random_string");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_STRING, ENTITY_TYPE_RANDOM_STRING, BEHAVIOUR_RANDOM_STRING);

entity_model!(
    RandomString,
    set length u64
);
impl Action for RandomString {}
impl ResultString for RandomString {}
