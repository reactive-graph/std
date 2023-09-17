use inexor_rgf_plugin_api::prelude::providers::*;

#[derive(TypeProvider, Component)]
#[type_provider(tys = "Components", path = "types/components")]
pub struct FlowComponentsProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "EntityTypes", path = "types/entities")]
pub struct FlowEntityTypesProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "FlowTypes", path = "types/flows")]
pub struct FlowFlowTypesProvider {}
