use crate::model::component_type;
use crate::model::properties;
use crate::NAMESPACE_TAXONOMY;

properties!(WeightedProperties, (WEIGHT, "weight", 0));

component_type!(COMPONENT_WEIGHTED, NAMESPACE_TAXONOMY, COMPONENT_NAME_WEIGHTED, "weighted");
