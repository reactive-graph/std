use crate::model::Component;
use crate::plugins::ComponentProvider;
use crate::provider::NumericComponentProviderImpl;

#[test]
fn components_should_exist() {
    let expected_components = vec!["numeric_operation", "numeric_gate"];
    let component_provider = NumericComponentProviderImpl {};
    let components = component_provider.get_components();
    assert_eq!(
        expected_components.len(),
        components
            .into_iter()
            .filter(|component| expected_components.contains(&component.name.as_str()))
            .count()
    );
}
