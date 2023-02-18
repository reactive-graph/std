use angular_units::Deg;

use crate::model::entity_model;
use crate::model::entity_ty;
use crate::Brightness;
use crate::HueSaturationComponent;
use crate::NAMESPACE_COLOR;

entity_ty!(ENTITY_TYPE_HSV, NAMESPACE_COLOR, ENTITY_TYPE_NAME_HSV, "hsv");

entity_model!(Hsv);
impl HueSaturationComponent for Hsv {}
impl Brightness for Hsv {}

pub trait TypedHsv: HueSaturationComponent + Brightness {
    fn hsv(&self) -> Option<prisma::Hsv<f64>> {
        let Some(hue) = self.get_hue() else {
            return None;
        };
        let Some(saturation) = self.get_saturation() else {
            return None;
        };
        let Some(brightness) = self.get_brightness() else {
            return None;
        };
        Some(prisma::Hsv::new(Deg(hue), saturation, brightness))
    }

    fn set_hsv(&self, hsv: &prisma::Hsv<f64>) {
        self.set_hue(hsv.hue().0);
        self.set_saturation(hsv.saturation());
        self.set_brightness(hsv.value());
    }
}
impl TypedHsv for Hsv {}
