use reactive_graph_plugin_api::prelude::providers::*;

#[derive(TypeProvider, Component)]
#[type_provider(tys = "Components", path = "types/components")]
pub struct ValueComponentsProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "EntityTypes", path = "types/entities")]
pub struct ValueEntityTypesProvider {}

#[cfg(test)]
mod tests {
    use reactive_graph_graph::prelude::*;

    use reactive_graph_std_value_model::NAMESPACE_VALUE;

    use super::*;

    #[test]
    fn components_should_exist() {
        let expected_components = ComponentTypeIds::with_namespace(NAMESPACE_VALUE)
            .ty("value_array")
            .ty("value_boolean")
            .ty("value_debugger_debug")
            .ty("value_debugger_trace")
            .ty("value_number")
            .ty("value_object")
            .ty("value_string");
        let component_provider = ValueComponentsProvider {};
        assert!(
            component_provider
                .get_types()
                .iter()
                .all(|component| expected_components.contains(component.key()))
        );
    }

    #[test]
    fn entity_types_should_exist() {
        let expected_entity_types = EntityTypeIds::with_namespace(NAMESPACE_VALUE)
            .ty("value_array")
            .ty("value_boolean")
            .ty("value_number")
            .ty("value_object")
            .ty("value_string");
        let entity_type_provider = ValueEntityTypesProvider {};
        assert!(
            entity_type_provider
                .get_types()
                .iter()
                .all(|entity_type| expected_entity_types.contains(entity_type.key()))
        );
    }
}
