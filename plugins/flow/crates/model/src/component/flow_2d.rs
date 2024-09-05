use crate::NAMESPACE_FLOW;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(
    Flow2DProperties,
    (FLOW_2D_X, "f2dx", 0.0),
    (FLOW_2D_Y, "f2dy", 0.0),
    (FLOW_2D_W, "f2dw", 10.0),
    (FLOW_2D_H, "f2dh", 10.0)
);

component_ty!(COMPONENT_FLOW_2D, NAMESPACE_FLOW, COMPONENT_NAME_FLOW_2D, "flow_2d");

component_model!(
    Flow2D,
    data f2dx f64,
    data f2dy f64,
    data f2dw f64,
    data f2dh f64,
);

pub trait Flow2DElement: Flow2D {
    fn get_x(&self) -> Option<f64> {
        self.get_f2dx()
    }

    fn get_y(&self) -> Option<f64> {
        self.get_f2dy()
    }

    fn get_width(&self) -> Option<f64> {
        self.get_f2dw()
    }

    fn get_height(&self) -> Option<f64> {
        self.get_f2dh()
    }
}
