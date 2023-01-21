use crate::model::EntityTypeId;
use crate::model_numeric::NAMESPACE_NUMERIC_F64;
use crate::plugins::EntityTypeProvider;
use crate::providers::NumericEntityTypeProviderImpl;
use inexor_rgf_model_numeric::NAMESPACE_NUMERIC_I64;

#[test]
fn entity_types_should_exist() {
    let expected_entity_types = vec![
        // f64
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "abs"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "acos"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "acosh"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "asin"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "asinh"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "atan"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "atan2"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "atanh"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "cbrt"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "ceil"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "cos"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "cosh"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "exp"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "exp2"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "floor"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "fract"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "hypot"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "ln"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "log"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "log2"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "log10"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "pow"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "recip"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "round"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "signum"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "sin"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "sinh"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "sqrt"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "tan"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "tanh"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "to_degrees"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "to_radians"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, "trunc"),
        // i64
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_I64, "abs"),
        EntityTypeId::new_from_type(NAMESPACE_NUMERIC_I64, "signum"),
    ];
    let entity_type_provider = NumericEntityTypeProviderImpl {};
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
