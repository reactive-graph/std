use crate::NAMESPACE_COLOR;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(AlphaProperties, (ALPHA, "alpha", 0.0));

component_ty!(COMPONENT_ALPHA, NAMESPACE_COLOR, COMPONENT_NAME_ALPHA, "alpha");

component_model!(
    Alpha,
    data alpha f64
);
