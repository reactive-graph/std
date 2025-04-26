use crate::plugins::EntityTypeProvider;
use crate::providers::RandomEntityTypeProviderImpl;
use reactive_graph_std_random_model::ENTITY_TYPE_RANDOM_BOOL;
use reactive_graph_std_random_model::ENTITY_TYPE_RANDOM_F64;
use reactive_graph_std_random_model::ENTITY_TYPE_RANDOM_F64_PSEUDO;
use reactive_graph_std_random_model::ENTITY_TYPE_RANDOM_F64_RANGE;
use reactive_graph_std_random_model::ENTITY_TYPE_RANDOM_I64;
use reactive_graph_std_random_model::ENTITY_TYPE_RANDOM_I64_RANGE;
use reactive_graph_std_random_model::ENTITY_TYPE_RANDOM_STRING;
use reactive_graph_std_random_model::ENTITY_TYPE_RANDOM_U64;
use reactive_graph_std_random_model::ENTITY_TYPE_RANDOM_U64_RANGE;
use reactive_graph_std_random_model::ENTITY_TYPE_RANDOM_UUID;

#[test]
fn entity_types_should_exist() {
    let expected_entity_types = vec![
        ENTITY_TYPE_RANDOM_BOOL.clone(),
        ENTITY_TYPE_RANDOM_F64.clone(),
        ENTITY_TYPE_RANDOM_F64_PSEUDO.clone(),
        ENTITY_TYPE_RANDOM_F64_RANGE.clone(),
        ENTITY_TYPE_RANDOM_I64.clone(),
        ENTITY_TYPE_RANDOM_I64_RANGE.clone(),
        ENTITY_TYPE_RANDOM_STRING.clone(),
        ENTITY_TYPE_RANDOM_U64.clone(),
        ENTITY_TYPE_RANDOM_U64_RANGE.clone(),
        ENTITY_TYPE_RANDOM_UUID.clone(),
    ];
    let entity_type_provider = RandomEntityTypeProviderImpl {};
    let entity_types = entity_type_provider.get_entity_types();
    assert_eq!(
        expected_entity_types.len(),
        entity_types
            .into_iter()
            .map(|entity_type| entity_type.ty)
            .filter(|ty| expected_entity_types.contains(&ty))
            .count()
    );
}
