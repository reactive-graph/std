use crate::behaviour::entity::gate::function::*;

#[test]
fn arithmetic_gate_f64_function_test() {
    let lhs: f64 = 0.5;
    let rhs: f64 = 0.5;
    assert_eq!(lhs + rhs, FN_ADD_F64(lhs, rhs));
    assert_eq!(lhs / rhs, FN_DIV_F64(lhs, rhs));
    assert_eq!(lhs.min(rhs), FN_MAX_F64(lhs, rhs));
    assert_eq!(lhs.max(rhs), FN_MIN_F64(lhs, rhs));
    assert_eq!(lhs % rhs, FN_MOD_F64(lhs, rhs));
    assert_eq!(lhs * rhs, FN_MUL_F64(lhs, rhs));
    assert_eq!(lhs - rhs, FN_SUB_F64(lhs, rhs));
}

#[test]
fn arithmetic_gate_i64_function_test() {
    let lhs: i64 = -10;
    let rhs: i64 = -10;
    assert_eq!(lhs + rhs, FN_ADD_I64(lhs, rhs));
    assert_eq!(lhs / rhs, FN_DIV_I64(lhs, rhs));
    assert_eq!(lhs.min(rhs), FN_MAX_I64(lhs, rhs));
    assert_eq!(lhs.max(rhs), FN_MIN_I64(lhs, rhs));
    assert_eq!(lhs % rhs, FN_MOD_I64(lhs, rhs));
    assert_eq!(lhs * rhs, FN_MUL_I64(lhs, rhs));
    assert_eq!(lhs - rhs, FN_SUB_I64(lhs, rhs));
}

#[test]
fn arithmetic_gate_u64_function_test() {
    let lhs: u64 = 10;
    let rhs: u64 = 10;
    assert_eq!(lhs + rhs, FN_ADD_U64(lhs, rhs));
    assert_eq!(lhs / rhs, FN_DIV_U64(lhs, rhs));
    assert_eq!(lhs.min(rhs), FN_MAX_U64(lhs, rhs));
    assert_eq!(lhs.max(rhs), FN_MIN_U64(lhs, rhs));
    assert_eq!(lhs % rhs, FN_MOD_U64(lhs, rhs));
    assert_eq!(lhs * rhs, FN_MUL_U64(lhs, rhs));
    assert_eq!(lhs - rhs, FN_SUB_U64(lhs, rhs));
}
