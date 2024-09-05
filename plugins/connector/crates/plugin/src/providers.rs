use reactive_graph_plugin_api::prelude::providers::*;

#[derive(TypeProvider, Component)]
#[type_provider(tys = "Components", path = "types/components")]
pub struct ConnectorComponentsProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "RelationTypes", path = "types/relations")]
pub struct ConnectorRelationTypesProvider {}
