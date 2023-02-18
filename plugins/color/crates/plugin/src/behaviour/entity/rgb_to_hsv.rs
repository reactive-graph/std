use angular_units::Deg;
use prisma::FromColor;
use prisma::Hsv;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;

use crate::model::*;
use crate::model_color::BrightnessProperties::BRIGHTNESS;
use crate::model_color::HueSaturationProperties::HUE;
use crate::model_color::HueSaturationProperties::SATURATION;
use crate::model_color::Rgb;
use crate::model_color::RgbProperties::BLUE;
use crate::model_color::RgbProperties::GREEN;
use crate::model_color::RgbProperties::RED;
use crate::model_color::TypedRgbComponent;
use crate::reactive::*;

entity_behaviour!(RgbToHsv, RgbToHsvFactory, RgbToHsvFsm, RgbToHsvBehaviourTransitions, RgbToHsvValidator);

behaviour_validator!(
    RgbToHsvValidator,
    ReactiveEntityInstance,
    HUE.as_ref(),
    SATURATION.as_ref(),
    BRIGHTNESS.as_ref(),
    RED.as_ref(),
    GREEN.as_ref(),
    BLUE.as_ref()
);

impl BehaviourInit<ReactiveEntityInstance> for RgbToHsvBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        convert_rgb_to_hsv(self.reactive_instance.clone());
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for RgbToHsvBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(RED.as_ref(), move |_: &Value| {
            convert_rgb_to_hsv(reactive_instance.clone());
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(GREEN.as_ref(), move |_: &Value| {
            convert_rgb_to_hsv(reactive_instance.clone());
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(BLUE.as_ref(), move |_: &Value| {
            convert_rgb_to_hsv(reactive_instance.clone());
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for RgbToHsvBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for RgbToHsvBehaviourTransitions {}

fn convert_rgb_to_hsv(reactive_instance: Arc<ReactiveEntityInstance>) {
    let rgb_to_hsv = Rgb::from(reactive_instance);
    if let Some(rgb) = rgb_to_hsv.rgb() {
        let hsv: Hsv<f64, Deg<f64>> = Hsv::from_color(&rgb);
        rgb_to_hsv.set(HUE, json!(hsv.hue().0));
        rgb_to_hsv.set(SATURATION, json!(hsv.saturation()));
        rgb_to_hsv.set(BRIGHTNESS, json!(hsv.value()));
    }
}
