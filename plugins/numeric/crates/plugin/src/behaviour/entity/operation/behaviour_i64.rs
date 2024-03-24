use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_numeric::NumericOperationProperties::LHS;
use inexor_rgf_model_result::ResultNumberI64Properties::RESULT;

use crate::behaviour::as_i64;
use crate::behaviour::entity::operation::function::NumericOperationI64Function;

entity_behaviour!(
    NumericOperationI64,
    NumericOperationI64Factory,
    NumericOperationI64Fsm,
    NumericOperationI64BehaviourTransitions,
    NumericOperationI64Validator,
    f,
    NumericOperationI64Function
);

behaviour_validator!(NumericOperationI64Validator, Uuid, ReactiveEntity, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for NumericOperationI64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).and_then(as_i64).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        let initial_value = f(lhs);
        self.reactive_instance.set(RESULT, json!(initial_value));
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for NumericOperationI64BehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |v: &Value| {
            if let Some(v) = as_i64(v.clone()) {
                reactive_instance.set(RESULT, json!(f(v)));
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for NumericOperationI64BehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for NumericOperationI64BehaviourTransitions {}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use inexor_rgf_model_numeric::NumericOperationI64;

    use crate::behaviour::entity::operation::behaviour_i64::NumericOperationI64Factory;
    use crate::behaviour::entity::operation::function::*;
    use crate::behaviour::entity::operation::tests::numeric_operation;

    use super::*;

    #[test]
    fn numeric_operation_behaviour_test() {
        let nv: i64 = -1;
        let nz: i64 = -0;
        let pz: i64 = 0;
        let pv: i64 = 1;

        for (behaviour_ty, f) in NUMERIC_OPERATIONS_I64.iter() {
            let entity_ty = EntityTypeId::new_from_type(behaviour_ty.namespace(), behaviour_ty.type_name());

            // negative
            let expected = f(nv);
            let result = test_numeric_operation_behaviour(behaviour_ty, &entity_ty, nv);
            println!("{}({})  |  {:?}  |  {:?}", entity_ty, nv, expected, result);
            assert_eq!(expected, result.unwrap());

            // negative zero
            let expected = f(nz);
            let result = test_numeric_operation_behaviour(behaviour_ty, &entity_ty, nz);
            println!("{}({})  |  {:?}  |  {:?}", entity_ty, nz, expected, result);
            assert_eq!(expected, result.unwrap());

            // positive zero
            let expected = f(pz);
            let result = test_numeric_operation_behaviour(behaviour_ty, &entity_ty, pz);
            println!("{}({})  |  {:?}  |  {:?}", entity_ty, pz, expected, result);
            assert_eq!(expected, result.unwrap());

            // positive
            let expected = f(pv);
            let result = test_numeric_operation_behaviour(behaviour_ty, &entity_ty, pv);
            println!("{}({})  |  {:?}  |  {:?}", entity_ty, pv, expected, result);
            assert_eq!(expected, result.unwrap());
        }
    }

    fn test_numeric_operation_behaviour(behaviour_ty: &BehaviourTypeId, entity_ty: &EntityTypeId, v: i64) -> Option<i64> {
        let behaviour = create_numeric_operation_behaviour(behaviour_ty, entity_ty);
        let reactive_instance = behaviour.get_reactive_instance();
        let numeric_operation = NumericOperationI64::from(reactive_instance.clone());
        numeric_operation.lhs(v);
        numeric_operation.result()
    }

    fn create_numeric_operation_behaviour(
        behaviour_ty: &BehaviourTypeId,
        entity_ty: &EntityTypeId,
    ) -> Arc<dyn BehaviourFsm<Uuid, ReactiveEntity> + Send + Sync> {
        let reactive_instance = numeric_operation(entity_ty);
        let not_function = NUMERIC_OPERATIONS_I64.get(&behaviour_ty).expect("Failed to get function");
        let not_factory = NumericOperationI64Factory::new(behaviour_ty.clone(), not_function.clone());
        not_factory.create(reactive_instance.clone()).expect("Failed to create behaviour")
    }
}
