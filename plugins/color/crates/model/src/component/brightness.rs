use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_COLOR;

properties!(BrightnessProperties, (BRIGHTNESS, "brightness", 0.0));

component_ty!(COMPONENT_BRIGHTNESS, NAMESPACE_COLOR, COMPONENT_NAME_BRIGHTNESS, "brightness");

component_model!(
    Brightness,
    data brightness f64,
);
