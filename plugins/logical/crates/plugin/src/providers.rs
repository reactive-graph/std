use reactive_graph_plugin_api::prelude::providers::*;

#[derive(TypeProvider, Component)]
#[type_provider(tys = "Components", path = "types/components")]
pub struct LogicalComponentsProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "EntityTypes", path = "types/entities")]
pub struct LogicalEntityTypesProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "FlowTypes", path = "types/flows")]
pub struct LogicalFlowTypesProvider {}
