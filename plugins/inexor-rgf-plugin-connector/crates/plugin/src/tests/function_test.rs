use crate::behaviour::relation::connector::CONNECTOR_BEHAVIOURS;
use crate::model::NamespacedTypeGetter;

#[test]
fn propagation_function_test() {
    let expected_propagation_functions = vec![
        "debug_connector",
        "default_connector",
        "parse_float_connector",
        "parse_int_connector",
        "to_string_connector",
        "trace_connector",
    ];
    assert_eq!(
        expected_propagation_functions.len(),
        CONNECTOR_BEHAVIOURS
            .keys()
            .into_iter()
            .map(|ty| ty.type_name())
            .filter(|function_name| expected_propagation_functions.contains(&function_name.as_str()))
            .count()
    );
}
