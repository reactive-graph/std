pub use component::alpha::*;
pub use component::brightness::*;
pub use component::hs::*;
pub use component::lightness::*;
pub use component::rgb::*;
pub use entity::hsv::*;
pub use entity::hsv_to_rgb::BEHAVIOUR_HSV_TO_RGB;
pub use entity::hsv_to_rgb::BEHAVIOUR_NAME_HSV_TO_RGB;
pub use entity::hsv_to_rgb::ENTITY_BEHAVIOUR_HSV_TO_RGB;
pub use entity::hsv_to_rgb::ENTITY_TYPE_HSV_TO_RGB;
pub use entity::hsv_to_rgb::ENTITY_TYPE_NAME_HSV_TO_RGB;
pub use entity::rgb::*;
pub use entity::rgb_to_hsv::*;
pub use entity::rgba::*;

pub mod component;
pub mod entity;

pub const NAMESPACE_COLOR: &str = "color";
