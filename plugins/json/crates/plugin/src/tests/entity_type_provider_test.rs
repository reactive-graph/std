use crate::plugins::EntityTypeProvider;
use crate::providers::JsonEntityTypeProviderImpl;
use reactive_graph_model_json::ENTITY_TYPE_ARRAY_CONTAINS;
use reactive_graph_model_json::ENTITY_TYPE_ARRAY_GET_BY_INDEX;
use reactive_graph_model_json::ENTITY_TYPE_ARRAY_LENGTH;
use reactive_graph_model_json::ENTITY_TYPE_ARRAY_POP;
use reactive_graph_model_json::ENTITY_TYPE_ARRAY_PUSH;
use reactive_graph_model_json::ENTITY_TYPE_ARRAY_REVERSE;
use reactive_graph_model_json::ENTITY_TYPE_LOAD_JSON;
use reactive_graph_model_json::ENTITY_TYPE_OBJECT_GET_PROPERTY;
use reactive_graph_model_json::ENTITY_TYPE_OBJECT_KEYS;
use reactive_graph_model_json::ENTITY_TYPE_OBJECT_REMOVE_PROPERTY;
use reactive_graph_model_json::ENTITY_TYPE_OBJECT_SET_PROPERTY;
use reactive_graph_model_json::ENTITY_TYPE_SAVE_JSON;

#[test]
fn entity_types_should_exist() {
    let expected_entity_types = vec![
        ENTITY_TYPE_ARRAY_CONTAINS.clone(),
        ENTITY_TYPE_ARRAY_GET_BY_INDEX.clone(),
        ENTITY_TYPE_ARRAY_LENGTH.clone(),
        ENTITY_TYPE_ARRAY_POP.clone(),
        ENTITY_TYPE_ARRAY_PUSH.clone(),
        ENTITY_TYPE_ARRAY_REVERSE.clone(),
        ENTITY_TYPE_LOAD_JSON.clone(),
        ENTITY_TYPE_OBJECT_GET_PROPERTY.clone(),
        ENTITY_TYPE_OBJECT_KEYS.clone(),
        ENTITY_TYPE_OBJECT_REMOVE_PROPERTY.clone(),
        ENTITY_TYPE_OBJECT_SET_PROPERTY.clone(),
        ENTITY_TYPE_SAVE_JSON.clone(),
    ];
    let entity_type_provider = JsonEntityTypeProviderImpl {};
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
