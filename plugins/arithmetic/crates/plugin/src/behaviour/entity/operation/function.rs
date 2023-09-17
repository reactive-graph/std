use inexor_rgf_model_arithmetic::NAMESPACE_ARITHMETIC_F64;
use inexor_rgf_model_arithmetic::NAMESPACE_ARITHMETIC_I64;
use inexor_rgf_model_arithmetic::NAMESPACE_ARITHMETIC_U64;

use std::sync::Arc;
use std::sync::LazyLock;

use crate::behaviour::entity::operation::behaviour_f64::ArithmeticOperationF64Factory;
use crate::behaviour::entity::operation::behaviour_i64::ArithmeticOperationI64Factory;
use crate::behaviour::entity::operation::behaviour_u64::ArithmeticOperationU64Factory;
use inexor_rgf_behaviour::entity::EntityBehaviourFactoryCreator;
use inexor_rgf_behaviour::entity::EntityBehaviourFunctions;
use inexor_rgf_behaviour::entity::EntityBehaviourFunctionsStorage;

pub type ArithmeticOperationFunction<T> = fn(T) -> T;
pub type ArithmeticOperationF64Function = ArithmeticOperationFunction<f64>;
pub type ArithmeticOperationI64Function = ArithmeticOperationFunction<i64>;
pub type ArithmeticOperationU64Function = ArithmeticOperationFunction<u64>;

pub const FN_INCREMENT_F64: ArithmeticOperationF64Function = |lhs| lhs + 1.0;
pub const FN_DECREMENT_F64: ArithmeticOperationF64Function = |lhs| lhs - 1.0;

pub const FN_INCREMENT_I64: ArithmeticOperationI64Function = |lhs| lhs + 1;
pub const FN_DECREMENT_I64: ArithmeticOperationI64Function = |lhs| lhs - 1;

pub const FN_INCREMENT_U64: ArithmeticOperationU64Function = |lhs| lhs + 1;
pub const FN_DECREMENT_U64: ArithmeticOperationU64Function = |lhs| lhs - 1;

const FACTORY_CREATOR_F64: EntityBehaviourFactoryCreator<ArithmeticOperationF64Function> = |ty, f| Arc::new(ArithmeticOperationF64Factory::new(ty.clone(), f));
const FACTORY_CREATOR_I64: EntityBehaviourFactoryCreator<ArithmeticOperationI64Function> = |ty, f| Arc::new(ArithmeticOperationI64Factory::new(ty.clone(), f));
const FACTORY_CREATOR_U64: EntityBehaviourFactoryCreator<ArithmeticOperationU64Function> = |ty, f| Arc::new(ArithmeticOperationU64Factory::new(ty.clone(), f));

pub static ARITHMETIC_OPERATIONS_F64: EntityBehaviourFunctionsStorage<ArithmeticOperationF64Function> = LazyLock::new(|| {
    EntityBehaviourFunctions::<ArithmeticOperationF64Function>::with_namespace(NAMESPACE_ARITHMETIC_F64, FACTORY_CREATOR_F64)
        .behaviour("increment", FN_INCREMENT_F64)
        .behaviour("decrement", FN_DECREMENT_F64)
        .get()
});

pub static ARITHMETIC_OPERATIONS_I64: EntityBehaviourFunctionsStorage<ArithmeticOperationI64Function> = LazyLock::new(|| {
    EntityBehaviourFunctions::<ArithmeticOperationI64Function>::with_namespace(NAMESPACE_ARITHMETIC_I64, FACTORY_CREATOR_I64)
        .behaviour("increment", FN_INCREMENT_I64)
        .behaviour("decrement", FN_DECREMENT_I64)
        .get()
});

pub static ARITHMETIC_OPERATIONS_U64: EntityBehaviourFunctionsStorage<ArithmeticOperationU64Function> = LazyLock::new(|| {
    EntityBehaviourFunctions::<ArithmeticOperationU64Function>::with_namespace(NAMESPACE_ARITHMETIC_U64, FACTORY_CREATOR_U64)
        .behaviour("increment", FN_INCREMENT_U64)
        .behaviour("decrement", FN_DECREMENT_U64)
        .get()
});

#[cfg(test)]
mod tests {

    use super::*;

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
}
