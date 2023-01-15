use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::NAMESPACE_RANDOM;

properties!(RandomU64PseudoProperties, (TRIGGER, "trigger", false), (SEED, "seed", 0), (RESULT, "result", 0));

entity_ty!(ENTITY_TYPE_RANDOM_U64_PSEUDO, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_U64_PSEUDO, "random_u64_pseudo");
behaviour_ty!(BEHAVIOUR_RANDOM_U64_PSEUDO, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_U64_PSEUDO, "random_u64_pseudo");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_U64_PSEUDO, ENTITY_TYPE_RANDOM_U64_PSEUDO, BEHAVIOUR_RANDOM_U64_PSEUDO);

entity_model!(
    RandomU64Pseudo,
    trigger,
    set seed u64,
    get result u64
);
