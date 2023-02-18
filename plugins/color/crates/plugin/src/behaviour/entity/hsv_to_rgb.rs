use std::sync::Arc;

use prisma::FromColor;
use serde_json::json;
use serde_json::Value;

use crate::model::*;
use crate::model_color::BrightnessProperties::BRIGHTNESS;
use crate::model_color::Hsv;
use crate::model_color::HueSaturationProperties::HUE;
use crate::model_color::HueSaturationProperties::SATURATION;
use crate::model_color::RgbProperties::BLUE;
use crate::model_color::RgbProperties::GREEN;
use crate::model_color::RgbProperties::RED;
use crate::model_color::TypedHsv;
use crate::reactive::*;

entity_behaviour!(HsvToRgb, HsvToRgbFactory, HsvToRgbFsm, HsvToRgbBehaviourTransitions, HsvToRgbValidator);

behaviour_validator!(
    HsvToRgbValidator,
    ReactiveEntityInstance,
    HUE.as_ref(),
    SATURATION.as_ref(),
    BRIGHTNESS.as_ref(),
    RED.as_ref(),
    GREEN.as_ref(),
    BLUE.as_ref()
);

impl BehaviourInit<ReactiveEntityInstance> for HsvToRgbBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        convert_hsv_to_rgb(self.reactive_instance.clone());
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for HsvToRgbBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(HUE.as_ref(), move |_: &Value| {
            convert_hsv_to_rgb(reactive_instance.clone());
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(SATURATION.as_ref(), move |_: &Value| {
            convert_hsv_to_rgb(reactive_instance.clone());
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(BRIGHTNESS.as_ref(), move |_: &Value| {
            convert_hsv_to_rgb(reactive_instance.clone());
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for HsvToRgbBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for HsvToRgbBehaviourTransitions {}

fn convert_hsv_to_rgb(reactive_instance: Arc<ReactiveEntityInstance>) {
    let hsv_to_rgb = Hsv::from(reactive_instance);
    if let Some(hsv) = hsv_to_rgb.hsv() {
        let rgb = prisma::Rgb::from_color(&hsv);
        hsv_to_rgb.set(RED, json!(rgb.red()));
        hsv_to_rgb.set(GREEN, json!(rgb.green()));
        hsv_to_rgb.set(BLUE, json!(rgb.blue()));
    }
}
