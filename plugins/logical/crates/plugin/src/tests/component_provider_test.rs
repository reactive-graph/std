use crate::plugins::ComponentProvider;
use crate::providers::LogicalComponentProviderImpl;
use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_model_logical::NAMESPACE_LOGICAL;

#[test]
fn components_should_exist() {
    let expected_components = vec![
        ComponentTypeId::new_from_type(NAMESPACE_LOGICAL, "logical_operation"),
        ComponentTypeId::new_from_type(NAMESPACE_LOGICAL, "logical_gate"),
    ];
    let component_provider = LogicalComponentProviderImpl {};
    let components = component_provider.get_components();
    assert_eq!(
        expected_components.len(),
        components.into_iter().filter(|component| expected_components.contains(&component.ty)).count()
    );
}
