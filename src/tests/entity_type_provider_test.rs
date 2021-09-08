use crate::plugins::EntityTypeProvider;
use crate::provider::ArithmeticEntityTypeProviderImpl;

#[test]
fn entity_types_should_exist() {
    let expected_entity_types = vec!["add", "div", "max", "min", "mod", "mul", "sub"];
    let entity_type_provider = ArithmeticEntityTypeProviderImpl {};
    let entity_types = entity_type_provider.get_entity_types();
    assert_eq!(expected_entity_types.len(), entity_types.len());
    println!("{}", expected_entity_types.join(", "));
    println!(
        "{}",
        entity_types
            .clone()
            .into_iter()
            .map(|entity_type| entity_type.name)
            .collect::<String>()
    );
    assert_eq!(
        expected_entity_types.len(),
        entity_types
            .into_iter()
            .filter(|entity_type| expected_entity_types.contains(&entity_type.name.as_str()))
            .count()
    );
}
