use prisma::Rgb;

use crate::NAMESPACE_COLOR;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

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
        let red = self.get_red()?;
        let green = self.get_green()?;
        let blue = self.get_blue()?;
        Some(Rgb::new(red, green, blue))
    }

    fn set_rgb(&self, rgb: &Rgb<f64>) {
        self.set_red(rgb.red());
        self.set_green(rgb.green());
        self.set_blue(rgb.blue());
    }
}
