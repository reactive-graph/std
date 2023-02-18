use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_COLOR;

properties!(HueSaturationProperties, (HUE, "hue", 0.0), (SATURATION, "saturation", 0.0));

component_ty!(COMPONENT_HS, NAMESPACE_COLOR, COMPONENT_NAME_HS, "hs");

component_model!(
    HueSaturationComponent,
    data hue f64,
    data saturation f64,
);
