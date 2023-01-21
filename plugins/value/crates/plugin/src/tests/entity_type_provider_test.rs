use crate::model::NamespacedTypeGetter;
use crate::plugins::EntityTypeProvider;
use crate::providers::ValueEntityTypeProviderImpl;

#[test]
fn entity_types_should_exist() {
    let expected_entity_types = vec!["value_array", "value_boolean", "value_number", "value_object", "value_string"];
    let entity_type_provider = ValueEntityTypeProviderImpl {};
    let entity_types = entity_type_provider.get_entity_types();
    assert_eq!(expected_entity_types.len(), entity_types.len());
    assert_eq!(
        expected_entity_types.len(),
        entity_types
            .into_iter()
            .filter(|entity_type| expected_entity_types.contains(&entity_type.type_name().as_str()))
            .count()
    );
}
