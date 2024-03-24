use crate::RgbComponent;
use crate::TypedRgbComponent;
use crate::NAMESPACE_COLOR;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_reactive_model_api::entity_model;

entity_ty!(ENTITY_TYPE_RGB, NAMESPACE_COLOR, ENTITY_TYPE_NAME_RGB, "rgb");

entity_model!(Rgb);
impl RgbComponent for Rgb {}
impl TypedRgbComponent for Rgb {}
