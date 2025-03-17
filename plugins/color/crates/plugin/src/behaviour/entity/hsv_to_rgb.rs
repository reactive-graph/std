use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use prisma::FromColor;

use reactive_graph_model_color::BrightnessProperties::BRIGHTNESS;
use reactive_graph_model_color::Hsv;
use reactive_graph_model_color::HueSaturationProperties::HUE;
use reactive_graph_model_color::HueSaturationProperties::SATURATION;
use reactive_graph_model_color::RgbProperties::BLUE;
use reactive_graph_model_color::RgbProperties::GREEN;
use reactive_graph_model_color::RgbProperties::RED;
use reactive_graph_model_color::TypedHsv;

entity_behaviour!(HsvToRgb, HsvToRgbFactory, HsvToRgbFsm, HsvToRgbBehaviourTransitions, HsvToRgbValidator);

behaviour_validator!(
    HsvToRgbValidator,
    Uuid,
    ReactiveEntity,
    HUE.as_ref(),
    SATURATION.as_ref(),
    BRIGHTNESS.as_ref(),
    RED.as_ref(),
    GREEN.as_ref(),
    BLUE.as_ref()
);

impl BehaviourInit<Uuid, ReactiveEntity> for HsvToRgbBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        convert_hsv_to_rgb(self.reactive_instance.clone());
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for HsvToRgbBehaviourTransitions {
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

impl BehaviourShutdown<Uuid, ReactiveEntity> for HsvToRgbBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for HsvToRgbBehaviourTransitions {}

fn convert_hsv_to_rgb(reactive_instance: ReactiveEntity) {
    let hsv_to_rgb = Hsv::from(reactive_instance);
    if let Some(hsv) = hsv_to_rgb.hsv() {
        let rgb = prisma::Rgb::from_color(&hsv);
        hsv_to_rgb.set(RED, json!(rgb.red()));
        hsv_to_rgb.set(GREEN, json!(rgb.green()));
        hsv_to_rgb.set(BLUE, json!(rgb.blue()));
    }
}
