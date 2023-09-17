use crate::NAMESPACE_COLOR;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(HueSaturationProperties, (HUE, "hue", 0.0), (SATURATION, "saturation", 0.0));

component_ty!(COMPONENT_HS, NAMESPACE_COLOR, COMPONENT_NAME_HS, "hs");

component_model!(
    HueSaturationComponent,
    data hue f64,
    data saturation f64,
);
