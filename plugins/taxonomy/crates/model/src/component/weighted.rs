use crate::NAMESPACE_TAXONOMY;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(WeightedProperties, (WEIGHT, "weight", 0));

component_ty!(COMPONENT_WEIGHTED, NAMESPACE_TAXONOMY, COMPONENT_NAME_WEIGHTED, "weighted");

component_model!(
    Weighted,
    data weight f64
);
