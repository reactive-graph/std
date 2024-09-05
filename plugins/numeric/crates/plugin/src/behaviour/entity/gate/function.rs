use std::sync::Arc;
use std::sync::LazyLock;

use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFactoryCreator;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctions;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctionsStorage;

use reactive_graph_model_numeric::NAMESPACE_NUMERIC_F64;

use crate::behaviour::entity::gate::behaviour_f64::NumericGateF64Factory;

use serde_json::json;
use serde_json::Value;

pub type NumericGateFunction<T> = fn(T, T) -> Value;
pub type NumericGateF64Function = NumericGateFunction<f64>;

pub const FN_ATAN2_F64: NumericGateF64Function = |lhs, rhs| json!(lhs.atan2(rhs));
pub const FN_HYPOT_F64: NumericGateF64Function = |lhs, rhs| json!(lhs.hypot(rhs));
pub const FN_LOG_F64: NumericGateF64Function = |lhs, rhs| json!(lhs.log(rhs));
pub const FN_POW_F64: NumericGateF64Function = |lhs, rhs| json!(lhs.powf(rhs));

const FACTORY_CREATOR_F64: EntityBehaviourFactoryCreator<NumericGateF64Function> = |ty, f| Arc::new(NumericGateF64Factory::new(ty.clone(), f));

pub static NUMERIC_GATES_F64: EntityBehaviourFunctionsStorage<NumericGateF64Function> = LazyLock::new(|| {
    EntityBehaviourFunctions::<NumericGateF64Function>::with_namespace(NAMESPACE_NUMERIC_F64, FACTORY_CREATOR_F64)
        .behaviour("atan2", FN_ATAN2_F64)
        .behaviour("hypot", FN_HYPOT_F64)
        .behaviour("log", FN_POW_F64)
        .behaviour("pow", FN_POW_F64)
        .get()
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_gate_function_test() {
        let lhs: f64 = 0.5;
        let rhs: f64 = 0.5;
        assert_eq!(lhs.atan2(rhs), FN_ATAN2_F64(lhs, rhs));
        assert_eq!(lhs.hypot(rhs), FN_HYPOT_F64(lhs, rhs));
        assert_eq!(lhs.log(rhs), FN_LOG_F64(lhs, rhs));
        assert_eq!(lhs.powf(rhs), FN_POW_F64(lhs, rhs));
    }
}
