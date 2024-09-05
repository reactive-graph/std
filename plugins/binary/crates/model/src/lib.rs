pub use component::binary_data::*;
pub use component::load_binary_data::*;
pub use component::save_binary_data::*;
pub use entity::load_binary_data::*;
pub use entity::save_binary_data::*;

pub mod component;
pub mod entity;

pub const NAMESPACE_BINARY: &str = "binary";
