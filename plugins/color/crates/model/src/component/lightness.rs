use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_COLOR;

properties!(LightnessProperties, (LIGHTNESS, "lightness", 0.0));

component_ty!(COMPONENT_LIGHTNESS, NAMESPACE_COLOR, COMPONENT_NAME_LIGHTNESS, "lightness");

component_model!(
    Lightness,
    data lightness f64,
);
