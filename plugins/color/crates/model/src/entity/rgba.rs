use crate::model::entity_model;
use crate::model::entity_ty;
use crate::Alpha;
use crate::RgbComponent;
use crate::TypedRgbComponent;
use crate::NAMESPACE_COLOR;

entity_ty!(ENTITY_TYPE_RGBA, NAMESPACE_COLOR, ENTITY_TYPE_NAME_RGBA, "rgba");

entity_model!(Rgba);
impl RgbComponent for Rgba {}
impl Alpha for Rgba {}
impl TypedRgbComponent for Rgba {}
