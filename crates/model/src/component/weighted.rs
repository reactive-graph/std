use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_TAXONOMY;

properties!(WeightedProperties, (WEIGHT, "weight", 0));

component_ty!(COMPONENT_WEIGHTED, NAMESPACE_TAXONOMY, COMPONENT_NAME_WEIGHTED, "weighted");

component_model!(
    Weighted,
    data weight f64
);
