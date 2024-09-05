use reactive_graph_plugin_api::prelude::providers::*;

#[derive(TypeProvider, Component)]
#[type_provider(tys = "Components", path = "types/components")]
pub struct TaxonomyComponentsProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "EntityTypes", path = "types/entities")]
pub struct TaxonomyEntityTypesProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "RelationTypes", path = "types/relations")]
pub struct TaxonomyRelationTypesProvider {}
