use reactive_graph_plugin_api::prelude::providers::*;

#[derive(TypeProvider, Component)]
#[type_provider(tys = "Components", path = "types/components")]
pub struct NumericComponentsProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "EntityTypes", path = "types/entities")]
pub struct NumericEntityTypesProvider {}

#[cfg(test)]
mod tests {
    use reactive_graph_graph::prelude::*;

    use reactive_graph_std_numeric_model::NAMESPACE_NUMERIC;
    use reactive_graph_std_numeric_model::NAMESPACE_NUMERIC_F64;
    use reactive_graph_std_numeric_model::NAMESPACE_NUMERIC_I64;
    use reactive_graph_std_numeric_model::NAMESPACE_NUMERIC_U64;

    use super::*;

    #[test]
    fn components_should_exist() {
        let expected_components = ComponentTypeIds::with_namespace(NAMESPACE_NUMERIC).ty("numeric_gate").ty("numeric_operation");
        let component_provider = NumericComponentsProvider {};
        assert!(
            component_provider
                .get_types()
                .iter()
                .all(|component| expected_components.contains(component.key()))
        );
    }

    #[test]
    fn entity_types_should_exist() {
        let entity_type_provider = NumericEntityTypesProvider {};
        let expected_entity_types_f64 = EntityTypeIds::with_namespace(NAMESPACE_NUMERIC_F64)
            .ty("abs")
            .ty("acos")
            .ty("acosh")
            .ty("asin")
            .ty("asinh")
            .ty("atan")
            .ty("atan2")
            .ty("atanh")
            .ty("cbrt")
            .ty("ceil")
            .ty("cos")
            .ty("cosh")
            .ty("exp")
            .ty("exp2")
            .ty("floor")
            .ty("fract")
            .ty("hypot")
            .ty("ln")
            .ty("log")
            .ty("log2")
            .ty("log10")
            .ty("pow")
            .ty("recip")
            .ty("round")
            .ty("signum")
            .ty("sin")
            .ty("sinh")
            .ty("sqrt")
            .ty("tan")
            .ty("tanh")
            .ty("to_degrees")
            .ty("to_radians")
            .ty("trunc");
        let expected_entity_types_i64 = EntityTypeIds::with_namespace(NAMESPACE_NUMERIC_I64).ty("abs").ty("signum");
        let expected_entity_types_u64 = EntityTypeIds::with_namespace(NAMESPACE_NUMERIC_U64);
        assert!(entity_type_provider.get_types().iter().all(|entity_type| {
            expected_entity_types_f64.contains(entity_type.key())
                || expected_entity_types_i64.contains(entity_type.key())
                || expected_entity_types_u64.contains(entity_type.key())
        }));
    }
}
