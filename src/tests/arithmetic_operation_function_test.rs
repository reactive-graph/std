use crate::behaviour::entity::operation::function::*;

#[test]
fn arithmetic_operation_function_test() {
    let nv: f64 = 10.0;
    assert_eq!(11.0, FN_INCREMENT(nv));
    assert_eq!(9.0, FN_DECREMENT(nv));
}
