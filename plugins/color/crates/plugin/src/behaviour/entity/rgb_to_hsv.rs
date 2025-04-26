use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use angular_units::Deg;
use prisma::FromColor;
use prisma::Hsv;

use reactive_graph_std_color_model::BrightnessProperties::BRIGHTNESS;
use reactive_graph_std_color_model::HueSaturationProperties::HUE;
use reactive_graph_std_color_model::HueSaturationProperties::SATURATION;
use reactive_graph_std_color_model::Rgb;
use reactive_graph_std_color_model::RgbProperties::BLUE;
use reactive_graph_std_color_model::RgbProperties::GREEN;
use reactive_graph_std_color_model::RgbProperties::RED;
use reactive_graph_std_color_model::TypedRgbComponent;

entity_behaviour!(RgbToHsv, RgbToHsvFactory, RgbToHsvFsm, RgbToHsvBehaviourTransitions, RgbToHsvValidator);

behaviour_validator!(
    RgbToHsvValidator,
    Uuid,
    ReactiveEntity,
    HUE.as_ref(),
    SATURATION.as_ref(),
    BRIGHTNESS.as_ref(),
    RED.as_ref(),
    GREEN.as_ref(),
    BLUE.as_ref()
);

impl BehaviourInit<Uuid, ReactiveEntity> for RgbToHsvBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        convert_rgb_to_hsv(self.reactive_instance.clone());
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for RgbToHsvBehaviourTransitions {
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

impl BehaviourShutdown<Uuid, ReactiveEntity> for RgbToHsvBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for RgbToHsvBehaviourTransitions {}

fn convert_rgb_to_hsv(reactive_instance: ReactiveEntity) {
    let rgb_to_hsv = Rgb::from(reactive_instance);
    if let Some(rgb) = rgb_to_hsv.rgb() {
        let hsv: Hsv<f64, Deg<f64>> = Hsv::from_color(&rgb);
        rgb_to_hsv.set(HUE, json!(hsv.hue().0));
        rgb_to_hsv.set(SATURATION, json!(hsv.saturation()));
        rgb_to_hsv.set(BRIGHTNESS, json!(hsv.value()));
    }
}
