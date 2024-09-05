use crate::plugins::ComponentProvider;
use crate::providers::JsonComponentProviderImpl;
use reactive_graph_graph::NamespacedTypeGetter;

#[test]
fn components_should_exist() {
    let expected_components = vec!["load_json", "save_json"];
    let component_provider = JsonComponentProviderImpl {};
    let components = component_provider.get_components();
    assert_eq!(
        expected_components.len(),
        components
            .into_iter()
            .filter(|component| expected_components.contains(&component.type_name().as_str()))
            .count()
    );
}
