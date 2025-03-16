use crate::NAMESPACE_COLOR;
use crate::RgbComponent;
use crate::TypedRgbComponent;
use reactive_graph_graph::entity_ty;
use reactive_graph_reactive_model_api::entity_model;

entity_ty!(ENTITY_TYPE_RGB, NAMESPACE_COLOR, ENTITY_TYPE_NAME_RGB, "rgb");

entity_model!(Rgb);
impl RgbComponent for Rgb {}
impl TypedRgbComponent for Rgb {}
