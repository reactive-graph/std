use std::sync::Arc;
use std::sync::LazyLock;

use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFactoryCreator;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctions;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctionsStorage;

use reactive_graph_std_arithmetic_model::NAMESPACE_ARITHMETIC_F64;
use reactive_graph_std_arithmetic_model::NAMESPACE_ARITHMETIC_I64;
use reactive_graph_std_arithmetic_model::NAMESPACE_ARITHMETIC_U64;

use crate::behaviour::entity::gate::behaviour_f64::ArithmeticGateF64Factory;
use crate::behaviour::entity::gate::behaviour_i64::ArithmeticGateI64Factory;
use crate::behaviour::entity::gate::behaviour_u64::ArithmeticGateU64Factory;

pub type ArithmeticGateFunction<T> = fn(T, T) -> T;
pub type ArithmeticGateF64Function = ArithmeticGateFunction<f64>;
pub type ArithmeticGateI64Function = ArithmeticGateFunction<i64>;
pub type ArithmeticGateU64Function = ArithmeticGateFunction<u64>;

pub const FN_ADD_F64: ArithmeticGateF64Function = |lhs, rhs| lhs + rhs;
pub const FN_DIV_F64: ArithmeticGateF64Function = |lhs, rhs| if rhs != 0.0 { lhs / rhs } else { 0.0 };
pub const FN_MAX_F64: ArithmeticGateF64Function = |lhs, rhs| lhs.max(rhs);
pub const FN_MIN_F64: ArithmeticGateF64Function = |lhs, rhs| lhs.min(rhs);
pub const FN_MOD_F64: ArithmeticGateF64Function = |lhs, rhs| if rhs != 0.0 { lhs % rhs } else { 0.0 };
pub const FN_MUL_F64: ArithmeticGateF64Function = |lhs, rhs| lhs * rhs;
pub const FN_SUB_F64: ArithmeticGateF64Function = |lhs, rhs| lhs - rhs;

pub const FN_ADD_I64: ArithmeticGateI64Function = |lhs, rhs| lhs + rhs;
pub const FN_DIV_I64: ArithmeticGateI64Function = |lhs, rhs| if rhs != 0 { lhs / rhs } else { 0 };
pub const FN_MAX_I64: ArithmeticGateI64Function = |lhs, rhs| lhs.max(rhs);
pub const FN_MIN_I64: ArithmeticGateI64Function = |lhs, rhs| lhs.min(rhs);
pub const FN_MOD_I64: ArithmeticGateI64Function = |lhs, rhs| if rhs != 0 { lhs % rhs } else { 0 };
pub const FN_MUL_I64: ArithmeticGateI64Function = |lhs, rhs| lhs * rhs;
pub const FN_SUB_I64: ArithmeticGateI64Function = |lhs, rhs| lhs - rhs;

pub const FN_ADD_U64: ArithmeticGateU64Function = |lhs, rhs| lhs + rhs;
pub const FN_DIV_U64: ArithmeticGateU64Function = |lhs, rhs| if rhs != 0 { lhs / rhs } else { 0 };
pub const FN_MAX_U64: ArithmeticGateU64Function = |lhs, rhs| lhs.max(rhs);
pub const FN_MIN_U64: ArithmeticGateU64Function = |lhs, rhs| lhs.min(rhs);
pub const FN_MOD_U64: ArithmeticGateU64Function = |lhs, rhs| if rhs != 0 { lhs % rhs } else { 0 };
pub const FN_MUL_U64: ArithmeticGateU64Function = |lhs, rhs| lhs * rhs;
pub const FN_SUB_U64: ArithmeticGateU64Function = |lhs, rhs| lhs - rhs;

const FACTORY_CREATOR_F64: EntityBehaviourFactoryCreator<ArithmeticGateF64Function> = |ty, f| Arc::new(ArithmeticGateF64Factory::new(ty.clone(), f));
const FACTORY_CREATOR_I64: EntityBehaviourFactoryCreator<ArithmeticGateI64Function> = |ty, f| Arc::new(ArithmeticGateI64Factory::new(ty.clone(), f));
const FACTORY_CREATOR_U64: EntityBehaviourFactoryCreator<ArithmeticGateU64Function> = |ty, f| Arc::new(ArithmeticGateU64Factory::new(ty.clone(), f));

pub static ARITHMETIC_GATES_F64: EntityBehaviourFunctionsStorage<ArithmeticGateF64Function> = LazyLock::new(|| {
    EntityBehaviourFunctions::<ArithmeticGateF64Function>::with_namespace(NAMESPACE_ARITHMETIC_F64, FACTORY_CREATOR_F64)
        .behaviour("add", FN_ADD_F64)
        .behaviour("div", FN_DIV_F64)
        .behaviour("max", FN_MAX_F64)
        .behaviour("min", FN_MIN_F64)
        .behaviour("mod", FN_MOD_F64)
        .behaviour("mul", FN_MUL_F64)
        .behaviour("sub", FN_SUB_F64)
        .get()
});

pub static ARITHMETIC_GATES_I64: EntityBehaviourFunctionsStorage<ArithmeticGateI64Function> = LazyLock::new(|| {
    EntityBehaviourFunctions::<ArithmeticGateI64Function>::with_namespace(NAMESPACE_ARITHMETIC_I64, FACTORY_CREATOR_I64)
        .behaviour("add", FN_ADD_I64)
        .behaviour("div", FN_DIV_I64)
        .behaviour("max", FN_MAX_I64)
        .behaviour("min", FN_MIN_I64)
        .behaviour("mod", FN_MOD_I64)
        .behaviour("mul", FN_MUL_I64)
        .behaviour("sub", FN_SUB_I64)
        .get()
});

pub static ARITHMETIC_GATES_U64: EntityBehaviourFunctionsStorage<ArithmeticGateU64Function> = LazyLock::new(|| {
    EntityBehaviourFunctions::<ArithmeticGateU64Function>::with_namespace(NAMESPACE_ARITHMETIC_U64, FACTORY_CREATOR_U64)
        .behaviour("add", FN_ADD_U64)
        .behaviour("div", FN_DIV_U64)
        .behaviour("max", FN_MAX_U64)
        .behaviour("min", FN_MIN_U64)
        .behaviour("mod", FN_MOD_U64)
        .behaviour("mul", FN_MUL_U64)
        .behaviour("sub", FN_SUB_U64)
        .get()
});

#[cfg(test)]
mod tests {

    use super::*;

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
}
