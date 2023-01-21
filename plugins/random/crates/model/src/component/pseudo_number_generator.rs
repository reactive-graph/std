use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_RANDOM;

properties!(PseudoNumberGeneratorProperties, (SEED, "seed", 0));

component_ty!(
    COMPONENT_PSEUDO_NUMBER_GENERATOR,
    NAMESPACE_RANDOM,
    COMPONENT_NAME_PSEUDO_NUMBER_GENERATOR,
    "pseudo_number_generator"
);

component_model!(
    PseudoNumberGenerator,
    set seed u64
);
