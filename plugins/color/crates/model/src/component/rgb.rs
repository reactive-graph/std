use prisma::Rgb;

use crate::NAMESPACE_COLOR;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(RgbProperties, (RED, "red", 0.0), (GREEN, "green", 0.0), (BLUE, "blue", 0.0));

component_ty!(COMPONENT_RGB, NAMESPACE_COLOR, COMPONENT_NAME_RGB, "rgb");

component_model!(
    RgbComponent,
    data red f64,
    data green f64,
    data blue f64,
);

pub trait TypedRgbComponent: RgbComponent {
    fn rgb(&self) -> Option<Rgb<f64>> {
        let Some(red) = self.get_red() else {
            return None;
        };
        let Some(green) = self.get_green() else {
            return None;
        };
        let Some(blue) = self.get_blue() else {
            return None;
        };
        Some(Rgb::new(red, green, blue))
    }

    fn set_rgb(&self, rgb: &Rgb<f64>) {
        self.set_red(rgb.red());
        self.set_green(rgb.green());
        self.set_blue(rgb.blue());
    }
}
