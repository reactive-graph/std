use crate::behaviour::entity::string_comparison::StringComparisonFactory;
use crate::behaviour::entity::string_comparison::STRING_COMPARISONS;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_string::StringComparison;
use crate::model_string::StringComparisonProperties::LHS;
use crate::model_string::StringComparisonProperties::RHS;
use crate::model_string::COMPONENT_STRING_COMPARISON;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;
use inexor_rgf_graph::BehaviourTypeId;
use inexor_rgf_graph::EntityTypeId;
use inexor_rgf_model_result::ResultBoolean;
use inexor_rgf_model_result::ResultBooleanProperties::RESULT;
use inexor_rgf_model_string::NAMESPACE_STRING;

#[test]
fn rx_contains_test() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_STRING, "contains");
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_STRING, "contains");

    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty)
        .property_with_default(&LHS)
        .property_with_default(&RHS)
        .property_with_default(&RESULT)
        .component(COMPONENT_STRING_COMPARISON.clone())
        .build();
    let contains = StringComparison::from(reactive_instance.clone());

    // No behaviour -> no change
    contains.lhs("test");
    contains.rhs("es");
    assert!(!contains.result().unwrap());

    // With behaviour
    {
        let f = STRING_COMPARISONS.get(&behaviour_ty).cloned().expect("Failed to find behaviour function");
        let factory = StringComparisonFactory::new(behaviour_ty, f);
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        contains.lhs("test");
        contains.rhs("es");
        assert!(contains.result().unwrap());
        contains.lhs("tet");
        contains.rhs("es");
        assert!(!contains.result().unwrap());
        contains.rhs("et");
        assert!(contains.result().unwrap());
    }
}
