use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::Brightness;
use crate::HueSaturationComponent;
use crate::RgbComponent;
use crate::TypedHsv;
use crate::TypedRgbComponent;
use crate::NAMESPACE_COLOR;

entity_ty!(ENTITY_TYPE_HSV_TO_RGB, NAMESPACE_COLOR, ENTITY_TYPE_NAME_HSV_TO_RGB, "hsv_to_rgb");
behaviour_ty!(BEHAVIOUR_HSV_TO_RGB, NAMESPACE_COLOR, BEHAVIOUR_NAME_HSV_TO_RGB, "hsv_to_rgb");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_HSV_TO_RGB, ENTITY_TYPE_HSV_TO_RGB, BEHAVIOUR_HSV_TO_RGB);

entity_model!(RgbToHsv);
impl RgbComponent for RgbToHsv {}
impl HueSaturationComponent for RgbToHsv {}
impl Brightness for RgbToHsv {}
impl TypedRgbComponent for RgbToHsv {}
impl TypedHsv for RgbToHsv {}
