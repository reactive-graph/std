use crate::NAMESPACE_RANDOM;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

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
