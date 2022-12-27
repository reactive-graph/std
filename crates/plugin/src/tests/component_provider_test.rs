use crate::model::ComponentTypeId;
use crate::model_numeric::COMPONENT_NAME_NUMERIC_GATE;
use crate::model_numeric::COMPONENT_NAME_NUMERIC_OPERATION;
use crate::model_numeric::NAMESPACE_NUMERIC;
use crate::plugins::ComponentProvider;
use crate::providers::NumericComponentProviderImpl;

#[test]
fn components_should_exist() {
    let expected_components = vec![
        ComponentTypeId::new_from_type(NAMESPACE_NUMERIC, COMPONENT_NAME_NUMERIC_OPERATION),
        ComponentTypeId::new_from_type(NAMESPACE_NUMERIC, COMPONENT_NAME_NUMERIC_GATE),
    ];
    let component_provider = NumericComponentProviderImpl {};
    let components = component_provider.get_components();
    assert_eq!(
        expected_components.len(),
        components.into_iter().filter(|component| expected_components.contains(&component.ty)).count()
    );
}
