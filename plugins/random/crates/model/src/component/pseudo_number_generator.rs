use crate::NAMESPACE_RANDOM;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

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
