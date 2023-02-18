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

entity_ty!(ENTITY_TYPE_RGB_TO_HSV, NAMESPACE_COLOR, ENTITY_TYPE_NAME_RGB_TO_HSV, "rgb_to_hsv");
behaviour_ty!(BEHAVIOUR_RGB_TO_HSV, NAMESPACE_COLOR, BEHAVIOUR_NAME_RGB_TO_HSV, "rgb_to_hsv");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RGB_TO_HSV, ENTITY_TYPE_RGB_TO_HSV, BEHAVIOUR_RGB_TO_HSV);

entity_model!(RgbToHsv);
impl RgbComponent for RgbToHsv {}
impl HueSaturationComponent for RgbToHsv {}
impl Brightness for RgbToHsv {}
impl TypedRgbComponent for RgbToHsv {}
impl TypedHsv for RgbToHsv {}
