use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use reactive_graph_model_numeric::NumericOperationProperties::LHS;
use reactive_graph_model_result::ResultNumberF64Properties::RESULT;

use crate::behaviour::as_f64;
use crate::behaviour::entity::operation::function::NumericOperationF64Function;

entity_behaviour!(
    NumericOperationF64,
    NumericOperationF64Factory,
    NumericOperationF64Fsm,
    NumericOperationF64BehaviourTransitions,
    NumericOperationF64Validator,
    f,
    NumericOperationF64Function
);

behaviour_validator!(NumericOperationF64Validator, Uuid, ReactiveEntity, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for NumericOperationF64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).and_then(as_f64).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        let initial_value = f(lhs);
        self.reactive_instance.set(RESULT, json!(initial_value));
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for NumericOperationF64BehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |v: &Value| {
            if let Some(v) = as_f64(v.clone()) {
                reactive_instance.set(RESULT, json!(f(v)));
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for NumericOperationF64BehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for NumericOperationF64BehaviourTransitions {}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use reactive_graph_behaviour_model_api::BehaviourFactory;
    use reactive_graph_behaviour_model_api::BehaviourFsm;
    use reactive_graph_behaviour_model_api::BehaviourTypeId;
    use reactive_graph_graph::prelude::*;
    use reactive_graph_reactive_model_impl::ReactiveEntity;
    use uuid::Uuid;

    use reactive_graph_model_numeric::NumericOperationF64;

    use crate::behaviour::entity::operation::behaviour_f64::NumericOperationF64Factory;
    use crate::behaviour::entity::operation::function::*;
    use crate::behaviour::entity::operation::tests::numeric_operation;

    #[test]
    fn numeric_operation_behaviour_test() {
        let nv: f64 = -0.5;
        let nz: f64 = -0.0;
        let pz: f64 = 0.0;
        let pv: f64 = 0.5;

        for (behaviour_ty, f) in NUMERIC_OPERATIONS_F64.iter() {
            let entity_ty = EntityTypeId::new_from_type(behaviour_ty.namespace(), behaviour_ty.type_name());

            // negative
            let expected = f(nv);
            let result = test_numeric_operation_behaviour(behaviour_ty, &entity_ty, nv);
            println!("{}({})  |  {:?}  |  {:?}", entity_ty, nv, expected, result);
            if !expected.is_nan() {
                assert_eq!(expected, result.unwrap());
            } else {
                assert!(result.is_none());
            }

            // negative zero
            let expected = f(nz);
            let result = test_numeric_operation_behaviour(behaviour_ty, &entity_ty, nz);
            println!("{}({})  |  {:?}  |  {:?}", entity_ty, nz, expected, result);
            if !expected.is_infinite() {
                assert_eq!(expected, result.unwrap());
            } else {
                assert!(result.is_none());
            }

            // positive zero
            let expected = f(pz);
            let result = test_numeric_operation_behaviour(behaviour_ty, &entity_ty, pz);
            println!("{}({})  |  {:?}  |  {:?}", entity_ty, pz, expected, result);
            if !expected.is_infinite() {
                assert_eq!(expected, result.unwrap());
            } else {
                assert!(result.is_none());
            }

            // positive
            let expected = f(pv);
            let result = test_numeric_operation_behaviour(behaviour_ty, &entity_ty, pv);
            println!("{}({})  |  {:?}  |  {:?}", entity_ty, pv, expected, result);
            assert_eq!(expected, result.unwrap());
        }
    }

    fn test_numeric_operation_behaviour(behaviour_ty: &BehaviourTypeId, entity_ty: &EntityTypeId, v: f64) -> Option<f64> {
        let behaviour = create_numeric_operation_behaviour(behaviour_ty, entity_ty);
        let reactive_instance = behaviour.get_reactive_instance();
        let numeric_operation = NumericOperationF64::from(reactive_instance.clone());
        numeric_operation.lhs(v);
        numeric_operation.result()
    }

    fn create_numeric_operation_behaviour(
        behaviour_ty: &BehaviourTypeId,
        entity_ty: &EntityTypeId,
    ) -> Arc<dyn BehaviourFsm<Uuid, ReactiveEntity> + Send + Sync> {
        let reactive_instance = numeric_operation(entity_ty);
        let not_function = NUMERIC_OPERATIONS_F64.get(&behaviour_ty).expect("Failed to get function");
        let not_factory = NumericOperationF64Factory::new(behaviour_ty.clone(), not_function.clone());
        not_factory.create(reactive_instance.clone()).expect("Failed to create behaviour")
    }
}
