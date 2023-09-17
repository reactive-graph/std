use crate::Alpha;
use crate::RgbComponent;
use crate::TypedRgbComponent;
use crate::NAMESPACE_COLOR;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_reactive_api::entity_model;

entity_ty!(ENTITY_TYPE_RGBA, NAMESPACE_COLOR, ENTITY_TYPE_NAME_RGBA, "rgba");

entity_model!(Rgba);
impl RgbComponent for Rgba {}
impl Alpha for Rgba {}
impl TypedRgbComponent for Rgba {}
