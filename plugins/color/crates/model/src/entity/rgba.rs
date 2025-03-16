use crate::Alpha;
use crate::NAMESPACE_COLOR;
use crate::RgbComponent;
use crate::TypedRgbComponent;
use reactive_graph_graph::entity_ty;
use reactive_graph_reactive_model_api::entity_model;

entity_ty!(ENTITY_TYPE_RGBA, NAMESPACE_COLOR, ENTITY_TYPE_NAME_RGBA, "rgba");

entity_model!(Rgba);
impl RgbComponent for Rgba {}
impl Alpha for Rgba {}
impl TypedRgbComponent for Rgba {}
