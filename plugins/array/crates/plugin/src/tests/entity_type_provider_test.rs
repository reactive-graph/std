use crate::plugins::EntityTypeProvider;
use crate::providers::ArrayEntityTypeProviderImpl;
use reactive_graph_std_array_model::ENTITY_TYPE_ARRAY_CONTAINS;
use reactive_graph_std_array_model::ENTITY_TYPE_ARRAY_GET_BY_INDEX;
use reactive_graph_std_array_model::ENTITY_TYPE_ARRAY_LENGTH;
use reactive_graph_std_array_model::ENTITY_TYPE_ARRAY_POP;
use reactive_graph_std_array_model::ENTITY_TYPE_ARRAY_PUSH;
use reactive_graph_std_array_model::ENTITY_TYPE_ARRAY_REVERSE;

#[test]
fn entity_types_should_exist() {
    let expected_entity_types = vec![
        ENTITY_TYPE_ARRAY_CONTAINS.clone(),
        ENTITY_TYPE_ARRAY_GET_BY_INDEX.clone(),
        ENTITY_TYPE_ARRAY_LENGTH.clone(),
        ENTITY_TYPE_ARRAY_POP.clone(),
        ENTITY_TYPE_ARRAY_PUSH.clone(),
        ENTITY_TYPE_ARRAY_REVERSE.clone(),
    ];
    let entity_type_provider = ArrayEntityTypeProviderImpl {};
    let entity_types = entity_type_provider.get_entity_types();
    assert_eq!(expected_entity_types.len(), entity_types.len());
    assert_eq!(
        expected_entity_types.len(),
        entity_types
            .into_iter()
            .map(|entity_type| entity_type.ty)
            .filter(|ty| expected_entity_types.contains(&ty))
            .count()
    );
}
