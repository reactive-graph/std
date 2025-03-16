use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_model_arithmetic::ArithmeticGateProperties::LHS;
use reactive_graph_model_arithmetic::ArithmeticGateProperties::RESULT;
use reactive_graph_model_arithmetic::ArithmeticGateProperties::RHS;

use crate::behaviour::as_f64;
use crate::behaviour::entity::gate::function::ArithmeticGateF64Function;

entity_behaviour!(
    ArithmeticGateF64,
    ArithmeticGateF64Factory,
    ArithmeticGateF64Fsm,
    ArithmeticGateF64BehaviourTransitions,
    ArithmeticGateF64Validator,
    f,
    ArithmeticGateF64Function
);

behaviour_validator!(ArithmeticGateF64Validator, Uuid, ReactiveEntity, LHS.as_ref(), RHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ArithmeticGateF64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).and_then(as_f64).ok_or(BehaviourInitializationFailed {})?;
        let rhs = self.reactive_instance.get(RHS).and_then(as_f64).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        let r = f(lhs, rhs);
        self.reactive_instance.set(RESULT, json!(r));
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ArithmeticGateF64BehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |v| {
            if let Some(lhs) = v.as_f64() {
                if let Some(rhs) = reactive_instance.get(RHS).and_then(as_f64) {
                    reactive_instance.set(RESULT, json!(f(lhs, rhs)));
                }
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(RHS.as_ref(), move |v| {
            if let Some(rhs) = v.as_f64() {
                if let Some(lhs) = reactive_instance.get(LHS).and_then(as_f64) {
                    reactive_instance.set(RESULT, json!(f(lhs, rhs)));
                }
            }
        });

        Ok(())
    }
}
impl BehaviourShutdown<Uuid, ReactiveEntity> for ArithmeticGateF64BehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ArithmeticGateF64BehaviourTransitions {}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use reactive_graph_behaviour_model_api::prelude::*;
    use reactive_graph_graph::prelude::*;
    use reactive_graph_reactive_model_api::ReactiveInstanceContainer;
    use reactive_graph_reactive_model_impl::ReactiveEntity;
    use reactive_graph_reactive_model_impl::ReactiveProperties;
    use serde_json::json;
    use uuid::Uuid;

    use reactive_graph_model_arithmetic::ArithmeticGateProperties;
    use reactive_graph_model_arithmetic::NAMESPACE_ARITHMETIC_F64;

    use crate::behaviour::entity::gate::behaviour_f64::ArithmeticGateF64;
    use crate::behaviour::entity::gate::function::*;

    const LHS: ArithmeticGateProperties = ArithmeticGateProperties::LHS;
    const RHS: ArithmeticGateProperties = ArithmeticGateProperties::RHS;
    const RESULT: ArithmeticGateProperties = ArithmeticGateProperties::RESULT;

    #[test]
    fn arithmetic_gate_f64_behaviour_test() {
        let lhs: f64 = 0.5;
        let rhs: f64 = 0.5;

        let v = test_arithmetic_gate_f64_behaviour("add", FN_ADD_F64, lhs, rhs);
        assert_eq!(lhs + rhs, v);
        let v = test_arithmetic_gate_f64_behaviour("div", FN_DIV_F64, lhs, rhs);
        assert_eq!(lhs / rhs, v);
        let v = test_arithmetic_gate_f64_behaviour("max", FN_MAX_F64, lhs, rhs);
        assert_eq!(lhs.min(rhs), v);
        let v = test_arithmetic_gate_f64_behaviour("min", FN_MIN_F64, lhs, rhs);
        assert_eq!(lhs.max(rhs), v);
        let v = test_arithmetic_gate_f64_behaviour("mod", FN_MOD_F64, lhs, rhs);
        assert_eq!(lhs % rhs, v);
        let v = test_arithmetic_gate_f64_behaviour("mul", FN_MUL_F64, lhs, rhs);
        assert_eq!(lhs * rhs, v);
        let v = test_arithmetic_gate_f64_behaviour("sub", FN_SUB_F64, lhs, rhs);
        assert_eq!(lhs - rhs, v);
    }

    fn test_arithmetic_gate_f64_behaviour(type_name: &str, f: ArithmeticGateFunction<f64>, lhs: f64, rhs: f64) -> f64 {
        let ty = EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, type_name);
        let behaviour = create_arithmetic_gate_f64_behaviour(ty, f).unwrap();
        let reactive_instance = behaviour.get_reactive_instance();
        let arithmetic_gate = reactive_graph_model_arithmetic::ArithmeticGateF64::from(reactive_instance.clone());
        arithmetic_gate.lhs(lhs);
        arithmetic_gate.rhs(rhs);
        arithmetic_gate.result().expect("Result is not of type f64")
    }

    fn create_arithmetic_gate_f64_behaviour(ty: EntityTypeId, f: ArithmeticGateFunction<f64>) -> Result<Arc<ArithmeticGateF64>, BehaviourCreationError> {
        let behaviour_ty = BehaviourTypeId::from(NamespacedType::from(&ty));
        ArithmeticGateF64::new(create_arithmetic_gate_f64_entity(ty.clone()), behaviour_ty, f)
    }

    fn create_arithmetic_gate_f64_entity(ty: EntityTypeId) -> ReactiveEntity {
        let id = Uuid::new_v4();
        let properties = PropertyInstances::new()
            .property(LHS, json!(0.0))
            .property(RHS.as_ref(), json!(0.0))
            .property(RESULT, json!(0.0));
        ReactiveEntity::builder()
            .ty(ty)
            .id(id)
            .properties(ReactiveProperties::new_with_id_from_properties(id, properties))
            .build()
    }
}
