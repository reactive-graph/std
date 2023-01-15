use crate::model::NamespacedTypeGetter;
use crate::plugins::ComponentProvider;
use crate::providers::ArithmeticComponentProviderImpl;

#[test]
fn components_should_exist() {
    let expected_components = vec!["arithmetic_gate", "arithmetic_operation"];
    let component_provider = ArithmeticComponentProviderImpl {};
    let components = component_provider.get_components();
    assert_eq!(
        expected_components.len(),
        components
            .into_iter()
            .filter(|component| expected_components.contains(&component.type_name().as_str()))
            .count()
    );
}
