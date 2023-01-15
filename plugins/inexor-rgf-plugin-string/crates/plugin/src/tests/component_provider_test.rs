use crate::model_string::COMPONENT_STRING_BOOL_OPERATION;
use crate::model_string::COMPONENT_STRING_COMPARISON;
use crate::model_string::COMPONENT_STRING_GATE;
use crate::model_string::COMPONENT_STRING_NUMBER_OPERATION;
use crate::model_string::COMPONENT_STRING_OPERATION;
use crate::model_string::COMPONENT_STRING_STRING_NUMBER_GATE;
use crate::plugins::ComponentProvider;
use crate::providers::StringComponentProviderImpl;

#[test]
fn components_should_exist() {
    let expected_components = vec![
        COMPONENT_STRING_BOOL_OPERATION.clone(),
        COMPONENT_STRING_COMPARISON.clone(),
        COMPONENT_STRING_GATE.clone(),
        COMPONENT_STRING_NUMBER_OPERATION.clone(),
        COMPONENT_STRING_OPERATION.clone(),
        COMPONENT_STRING_STRING_NUMBER_GATE.clone(),
    ];
    let component_provider = StringComponentProviderImpl {};
    let components = component_provider.get_components();
    assert_eq!(
        expected_components.len(),
        components
            .into_iter()
            .map(|component| component.ty)
            .filter(|ty| expected_components.contains(&ty))
            .count()
    );
}
