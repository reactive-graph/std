use crate::NAMESPACE_COLOR;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(BrightnessProperties, (BRIGHTNESS, "brightness", 0.0));

component_ty!(COMPONENT_BRIGHTNESS, NAMESPACE_COLOR, COMPONENT_NAME_BRIGHTNESS, "brightness");

component_model!(
    Brightness,
    data brightness f64,
);
