use crate::Brightness;
use crate::HueSaturationComponent;
use crate::NAMESPACE_COLOR;
use crate::RgbComponent;
use crate::TypedHsv;
use crate::TypedRgbComponent;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_reactive_model_api::entity_model;

entity_ty!(ENTITY_TYPE_HSV_TO_RGB, NAMESPACE_COLOR, ENTITY_TYPE_NAME_HSV_TO_RGB, "hsv_to_rgb");
behaviour_ty!(BEHAVIOUR_HSV_TO_RGB, NAMESPACE_COLOR, BEHAVIOUR_NAME_HSV_TO_RGB, "hsv_to_rgb");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_HSV_TO_RGB, ENTITY_TYPE_HSV_TO_RGB, BEHAVIOUR_HSV_TO_RGB);

entity_model!(HsvToRgb);
impl RgbComponent for HsvToRgb {}
impl HueSaturationComponent for HsvToRgb {}
impl Brightness for HsvToRgb {}
impl TypedRgbComponent for HsvToRgb {}
impl TypedHsv for HsvToRgb {}
