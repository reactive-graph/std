crate::plugins::component_provider_impl!(Binary, "$CARGO_MANIFEST_DIR/../../types/components");
crate::plugins::entity_type_provider_impl!(Binary, "$CARGO_MANIFEST_DIR/../../types/entities");

pub use web_resource_provider::*;

pub mod web_resource_provider;
