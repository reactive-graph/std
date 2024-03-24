use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use uuid::Uuid;

use inexor_rgf_model_numeric::NumericGateProperties::LHS;
use inexor_rgf_model_numeric::NumericGateProperties::RHS;
use inexor_rgf_model_result::ResultNumberF64Properties::RESULT;

use crate::behaviour::as_f64;
use crate::behaviour::entity::gate::function::NumericGateF64Function;

entity_behaviour!(
    NumericGateF64,
    NumericGateF64Factory,
    NumericGateF64Fsm,
    NumericGateF64BehaviourTransitions,
    NumericGateF64Validator,
    f,
    NumericGateF64Function
);

behaviour_validator!(NumericGateF64Validator, Uuid, ReactiveEntity, LHS.as_ref(), RHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for NumericGateF64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).and_then(as_f64).ok_or(BehaviourInitializationFailed {})?;
        let rhs = self.reactive_instance.get(RHS).and_then(as_f64).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        let initial_value = f(lhs, rhs);
        self.reactive_instance.set(RESULT, initial_value);
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for NumericGateF64BehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |v| {
            if let Some(lhs) = v.as_f64() {
                if let Some(rhs) = reactive_instance.get(RHS).and_then(as_f64) {
                    reactive_instance.set(RESULT, f(lhs, rhs));
                }
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(RHS.as_ref(), move |v| {
            if let Some(rhs) = v.as_f64() {
                if let Some(lhs) = reactive_instance.get(LHS).and_then(as_f64) {
                    reactive_instance.set(RESULT, f(lhs, rhs));
                }
            }
        });

        Ok(())
    }
}
impl BehaviourShutdown<Uuid, ReactiveEntity> for NumericGateF64BehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for NumericGateF64BehaviourTransitions {}

// #[cfg(test)]
// mod tests {
//     use inexor_rgf_behaviour_api::BehaviourCreationError;
//     use serde_json::json;
//
//     use crate::behaviour::entity::gate::function::*;
//     use inexor_rgf_graph::prelude::*;
//     use inexor_rgf_reactive_model_impl::ReactiveEntity;
//     use inexor_rgf_model_numeric::NumericGateF64;
//     use inexor_rgf_model_numeric::NumericGateProperties;
//     use inexor_rgf_reactive_api::prelude::*;
//     use crate::behaviour::entity::gate::tests::numeric_gate;
//
//     #[test]
//     fn numeric_operation_behaviour_test() {
//         let lhs: f64 = 0.5;
//         let rhs: f64 = 0.5;
//
//         assert_eq!(lhs.atan2(rhs), test_numeric_gate_behaviour(FN_ATAN2_F64, lhs, rhs).unwrap());
//         assert_eq!(lhs.hypot(rhs), test_numeric_gate_behaviour(FN_HYPOT_F64, lhs, rhs).unwrap());
//         assert_eq!(lhs.log(rhs), test_numeric_gate_behaviour(FN_LOG_F64, lhs, rhs).unwrap());
//         assert_eq!(lhs.powf(rhs), test_numeric_gate_behaviour(FN_POW_F64, lhs, rhs).unwrap());
//     }
//
//     fn test_numeric_gate_behaviour(f: NumericGateFunction<f64>, lhs: f64, rhs: f64) -> Option<f64> {
//         let b = create_numeric_gate_behaviour(f).unwrap();
//         b.lhs(lhs);
//         b.rhs(rhs);
//         b.result()
//     }
//
//     fn create_numeric_gate_behaviour(f: NumericGateFunction<f64>) -> Result<NumericGateF64, BehaviourCreationError> {
//
//         let numeric_gate = create_numeric_gate_entity();
//         NumericGateF64::from(numeric_gate("numeric", "abs"), f)
//     }
//
//     fn create_numeric_gate_entity() -> ReactiveEntity {
//         ReactiveEntityInstanceBuilder::new("numeric", "abs")
//             .property(NumericGateProperties::LHS.as_ref(), json!(NumericGateProperties::LHS.default_value()))
//             .property(NumericGateProperties::RHS.as_ref(), json!(NumericGateProperties::RHS.default_value()))
//             .property(NumericGateProperties::RESULT.as_ref(), json!(NumericGateProperties::RESULT.default_value()))
//             .build()
//     }
// }
