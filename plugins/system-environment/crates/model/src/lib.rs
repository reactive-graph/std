use inexor_rgf_core_model as model;
use uuid::Uuid;

pub use entity::*;

pub mod entity;

pub const NAMESPACE_SYSTEM_ENVIRONMENT: &str = "system_environment";

pub static NAMESPACE_SYSTEM_ENVIRONMENT_ID: Uuid = Uuid::from_u128(0x6ba7b8109dad11d180b400c04fd430c7);
