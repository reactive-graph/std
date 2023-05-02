crate::plugins::component_provider_impl!(Binary, "types/components");
crate::plugins::entity_type_provider_impl!(Binary, "types/entities");

pub use web_resource_provider::*;

pub mod web_resource_provider;
