use crate::NAMESPACE_COLOR;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(LightnessProperties, (LIGHTNESS, "lightness", 0.0));

component_ty!(COMPONENT_LIGHTNESS, NAMESPACE_COLOR, COMPONENT_NAME_LIGHTNESS, "lightness");

component_model!(
    Lightness,
    data lightness f64,
);
