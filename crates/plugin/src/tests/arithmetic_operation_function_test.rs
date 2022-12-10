use crate::behaviour::entity::operation::function::*;

#[test]
fn arithmetic_operation_f64_function_test() {
    let nv: f64 = 10.0;
    assert_eq!(11.0, FN_INCREMENT_F64(nv));
    assert_eq!(9.0, FN_DECREMENT_F64(nv));
}

#[test]
fn arithmetic_operation_i64_function_test() {
    let nv: i64 = -10;
    assert_eq!(-9, FN_INCREMENT_I64(nv));
    assert_eq!(-11, FN_DECREMENT_I64(nv));
}

#[test]
fn arithmetic_operation_u64_function_test() {
    let nv: u64 = 10;
    assert_eq!(11, FN_INCREMENT_U64(nv));
    assert_eq!(9, FN_DECREMENT_U64(nv));
}
