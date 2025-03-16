use angular_units::Deg;

use crate::Brightness;
use crate::HueSaturationComponent;
use crate::NAMESPACE_COLOR;
use reactive_graph_graph::entity_ty;
use reactive_graph_reactive_model_api::entity_model;

entity_ty!(ENTITY_TYPE_HSV, NAMESPACE_COLOR, ENTITY_TYPE_NAME_HSV, "hsv");

entity_model!(Hsv);
impl HueSaturationComponent for Hsv {}
impl Brightness for Hsv {}

pub trait TypedHsv: HueSaturationComponent + Brightness {
    fn hsv(&self) -> Option<prisma::Hsv<f64>> {
        let hue = self.get_hue()?;
        let saturation = self.get_saturation()?;
        let brightness = self.get_brightness()?;
        Some(prisma::Hsv::new(Deg(hue), saturation, brightness))
    }

    fn set_hsv(&self, hsv: &prisma::Hsv<f64>) {
        self.set_hue(hsv.hue().0);
        self.set_saturation(hsv.saturation());
        self.set_brightness(hsv.value());
    }
}
impl TypedHsv for Hsv {}
