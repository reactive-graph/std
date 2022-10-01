use crate::behaviour::entity::gate::function::*;

#[test]
fn arithmetic_gate_function_test() {
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
