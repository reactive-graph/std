use crate::plugins::ComponentProvider;
use crate::providers::ComparisonComponentProviderImpl;
use reactive_graph_graph::NamespacedTypeGetter;

#[test]
fn components_should_exist() {
    let expected_components = vec!["comparison_gate"];
    let component_provider = ComparisonComponentProviderImpl {};
    let components = component_provider.get_components();
    assert_eq!(
        expected_components.len(),
        components
            .into_iter()
            .filter(|component| expected_components.contains(&component.type_name().as_str()))
            .count()
    );
}
