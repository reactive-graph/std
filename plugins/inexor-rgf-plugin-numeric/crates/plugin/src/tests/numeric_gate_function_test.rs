use crate::behaviour::entity::gate::function::*;

#[test]
fn numeric_gate_function_test() {
    let lhs: f64 = 0.5;
    let rhs: f64 = 0.5;
    assert_eq!(lhs.atan2(rhs), FN_ATAN2_F64(lhs, rhs));
    assert_eq!(lhs.hypot(rhs), FN_HYPOT_F64(lhs, rhs));
    assert_eq!(lhs.log(rhs), FN_LOG_F64(lhs, rhs));
    assert_eq!(lhs.powf(rhs), FN_POW_F64(lhs, rhs));
}
