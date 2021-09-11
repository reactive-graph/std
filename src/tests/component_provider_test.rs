use crate::plugins::ComponentProvider;
use crate::provider::LogicalComponentProviderImpl;

#[test]
fn components_should_exist() {
    let expected_components = vec!["logical_operation", "logical_gate"];
    let component_provider = LogicalComponentProviderImpl {};
    let components = component_provider.get_components();
    assert_eq!(
        expected_components.len(),
        components
            .into_iter()
            .filter(|component| expected_components.contains(&component.name.as_str()))
            .count()
    );
}
