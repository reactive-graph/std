use crate::model_metadata::COMPONENT_DUBLIN_CORE;
use crate::model_metadata::COMPONENT_ID3;
use crate::plugins::ComponentProvider;
use crate::providers::MetaDataComponentProviderImpl;

#[test]
fn components_should_exist() {
    let expected_components = vec![COMPONENT_DUBLIN_CORE.clone(), COMPONENT_ID3.clone()];
    let component_provider = MetaDataComponentProviderImpl {};
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
