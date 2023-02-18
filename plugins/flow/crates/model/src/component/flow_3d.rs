use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_FLOW;

properties!(
    Flow3DProperties,
    (FLOW_3D_X, "f3dx", 0.0),
    (FLOW_3D_Y, "f3dy", 0.0),
    (FLOW_3D_Z, "f3dz", 0.0),
    (FLOW_3D_W, "f2dw", 10.0),
    (FLOW_3D_H, "f2dh", 10.0),
    (FLOW_3D_D, "f2dd", 10.0)
);

component_ty!(COMPONENT_FLOW_3D, NAMESPACE_FLOW, COMPONENT_NAME_FLOW_3D, "flow_3d");

component_model!(
    Flow3D,
    data f3dx f64,
    data f3dy f64,
    data f3dz f64,
    data f3dw f64,
    data f3dh f64,
    data f3dd f64,
);

pub trait Flow3DElement: Flow3D {
    fn get_x(&self) -> Option<f64> {
        self.get_f3dx()
    }

    fn get_y(&self) -> Option<f64> {
        self.get_f3dy()
    }

    fn get_z(&self) -> Option<f64> {
        self.get_f3dz()
    }

    fn get_width(&self) -> Option<f64> {
        self.get_f3dw()
    }

    fn get_height(&self) -> Option<f64> {
        self.get_f3dh()
    }

    fn get_depth(&self) -> Option<f64> {
        self.get_f3dd()
    }
}
