use crate::plugins::EntityTypeProvider;
use crate::provider::NumericEntityTypeProviderImpl;

#[test]
fn entity_types_should_exist() {
    let expected_entity_types = vec![
        "abs",
        "acos",
        "acosh",
        "asin",
        "asinh",
        "atan",
        "atan2",
        "atanh",
        "cbrt",
        "ceil",
        "cos",
        "cosh",
        "exp",
        "exp2",
        "floor",
        "fract",
        "hypot",
        "ln",
        "log",
        "log2",
        "log10",
        "pow",
        "recip",
        "round",
        "signum",
        "sin",
        "sinh",
        "sqrt",
        "tan",
        "tanh",
        "to_degrees",
        "to_radians",
        "trunc",
    ];
    let entity_type_provider = NumericEntityTypeProviderImpl {};
    let entity_types = entity_type_provider.get_entity_types();
    assert_eq!(
        expected_entity_types.len(),
        entity_types
            .into_iter()
            .filter(|entity_type| expected_entity_types.contains(&entity_type.name.as_str()))
            .count()
    );
}
