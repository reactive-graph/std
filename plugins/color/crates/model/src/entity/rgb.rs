use crate::model::entity_model;
use crate::model::entity_ty;
use crate::RgbComponent;
use crate::TypedRgbComponent;
use crate::NAMESPACE_COLOR;

entity_ty!(ENTITY_TYPE_RGB, NAMESPACE_COLOR, ENTITY_TYPE_NAME_RGB, "rgb");

entity_model!(Rgb);
impl RgbComponent for Rgb {}
impl TypedRgbComponent for Rgb {}
