use inexor_rgf_plugin_api::prelude::providers::*;

/// Reads component type definitions from json or toml files.
///
/// Each component is defined in it's own file.
/// The path is the location of the folder containing the files.
///
/// Implements trait TypeProvider<Components>
#[derive(TypeProvider, Component)]
#[type_provider(tys = "Components", path = "types/components")]
pub struct BaseComponentsProvider {}

#[cfg(test)]
mod tests {
    use super::*;
    use inexor_rgf_graph::ComponentTypeId;
    use inexor_rgf_graph::ComponentTypeIds;
    use inexor_rgf_plugin_api::TypeProvider;

    #[test]
    fn components_should_exist() {
        let expected_tys = ComponentTypeIds::new()
            .component(ComponentTypeId::new_from_type("base", "describable"))
            .component(ComponentTypeId::new_from_type("base", "licensed"))
            .component(ComponentTypeId::new_from_type("base", "named"))
            .component(ComponentTypeId::new_from_type("base", "versioned"));
        let component_provider = BaseComponentsProvider {};
        let components = component_provider.get_types();
        assert_eq!(expected_tys.len(), components.into_iter().filter(|(ty, _)| expected_tys.contains(ty)).count());
    }
}
