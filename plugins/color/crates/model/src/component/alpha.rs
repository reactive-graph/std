use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_COLOR;

properties!(AlphaProperties, (ALPHA, "alpha", 0.0));

component_ty!(COMPONENT_ALPHA, NAMESPACE_COLOR, COMPONENT_NAME_ALPHA, "alpha");

component_model!(
    Alpha,
    data alpha f64
);
