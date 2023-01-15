use crate::model::EntityTypeId;
use crate::model_arithmetic::NAMESPACE_ARITHMETIC_F64;
use crate::model_arithmetic::NAMESPACE_ARITHMETIC_I64;
use crate::model_arithmetic::NAMESPACE_ARITHMETIC_U64;
use crate::plugins::EntityTypeProvider;
use crate::provider::ArithmeticEntityTypeProviderImpl;

#[test]
fn entity_types_should_exist() {
    let expected_entity_types = vec![
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, "add"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, "decrement"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, "div"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, "increment"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, "max"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, "min"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, "mod"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, "mul"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, "sub"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_I64, "add"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_I64, "decrement"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_I64, "div"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_I64, "increment"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_I64, "max"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_I64, "min"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_I64, "mod"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_I64, "mul"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_I64, "sub"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_U64, "add"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_U64, "decrement"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_U64, "div"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_U64, "increment"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_U64, "max"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_U64, "min"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_U64, "mod"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_U64, "mul"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_U64, "sub"),
        EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_U64, "counter"), // only for u64
    ];
    let entity_type_provider = ArithmeticEntityTypeProviderImpl {};
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
