use crate::plugins::EntityTypeProvider;
use crate::providers::JsonEntityTypeProviderImpl;
use reactive_graph_model_json::ENTITY_TYPE_OBJECT_GET_PROPERTY;
use reactive_graph_model_json::ENTITY_TYPE_OBJECT_KEYS;
use reactive_graph_model_json::ENTITY_TYPE_OBJECT_REMOVE_PROPERTY;
use reactive_graph_model_json::ENTITY_TYPE_OBJECT_SET_PROPERTY;

#[test]
fn entity_types_should_exist() {
    let expected_entity_types = vec![
        ENTITY_TYPE_OBJECT_GET_PROPERTY.clone(),
        ENTITY_TYPE_OBJECT_KEYS.clone(),
        ENTITY_TYPE_OBJECT_REMOVE_PROPERTY.clone(),
        ENTITY_TYPE_OBJECT_SET_PROPERTY.clone(),
    ];
    let entity_type_provider = ObjectEntityTypeProviderImpl {};
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
