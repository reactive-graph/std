use crate::plugins::EntityTypeProvider;
use crate::provider::ValueEntityTypeProviderImpl;
use inexor_rgf_core_model::NamespacedTypeGetter;

#[test]
fn entity_types_should_exist() {
    let expected_entity_types = vec![
        "state_array",
        "state_boolean",
        "state_number",
        "state_object",
        "state_string",
        "value_array",
        "value_boolean",
        "value_number",
        "value_object",
        "value_string",
    ];
    let entity_type_provider = ValueEntityTypeProviderImpl {};
    let entity_types = entity_type_provider.get_entity_types();
    assert_eq!(expected_entity_types.len(), entity_types.len());
    println!("{}", expected_entity_types.join(", "));
    println!("{}", entity_types.clone().into_iter().map(|entity_type| entity_type.type_name()).collect::<String>());
    assert_eq!(
        expected_entity_types.len(),
        entity_types
            .into_iter()
            .filter(|entity_type| expected_entity_types.contains(&entity_type.type_name().as_str()))
            .count()
    );
}
