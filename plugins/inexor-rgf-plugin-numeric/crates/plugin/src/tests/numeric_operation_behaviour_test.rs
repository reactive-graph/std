use std::sync::Arc;

use crate::behaviour::entity::operation::behaviour_f64::NumericOperationF64Factory;
use crate::behaviour::entity::operation::function::*;
use crate::model::BehaviourTypeId;
use crate::model::EntityTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::ReactiveEntityInstance;
use crate::model_numeric::NumericOperationF64;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourFsm;
use crate::tests::numeric_operation_test::numeric_operation;

#[test]
fn numeric_operation_behaviour_test() {
    let nv: f64 = -0.5;
    let nz: f64 = -0.0;
    let pz: f64 = 0.0;
    let pv: f64 = 0.5;

    for (behaviour_ty, f) in NUMERIC_OPERATIONS_F64.iter() {
        let entity_ty = EntityTypeId::new_from_type(behaviour_ty.namespace(), behaviour_ty.type_name());

        // negative
        let expected = f(nv).as_f64();
        let result = test_numeric_operation_behaviour(behaviour_ty, &entity_ty, nv);
        println!("{}({})  |  {:?}  |  {:?}", entity_ty, nv, expected, result);
        assert_eq!(expected, result);

        // negative zero
        let expected = f(nz).as_f64();
        let result = test_numeric_operation_behaviour(behaviour_ty, &entity_ty, nz);
        println!("{}({})  |  {:?}  |  {:?}", entity_ty, nz, expected, result);
        assert_eq!(expected, result);

        // positive zero
        let expected = f(pz).as_f64();
        let result = test_numeric_operation_behaviour(behaviour_ty, &entity_ty, pz);
        println!("{}({})  |  {:?}  |  {:?}", entity_ty, pz, expected, result);
        assert_eq!(expected, result);

        // positive
        let expected = f(pv).as_f64();
        let result = test_numeric_operation_behaviour(behaviour_ty, &entity_ty, pv);
        println!("{}({})  |  {:?}  |  {:?}", entity_ty, pv, expected, result);
        assert_eq!(expected, result);
    }
}

fn test_numeric_operation_behaviour(behaviour_ty: &BehaviourTypeId, entity_ty: &EntityTypeId, v: f64) -> Option<f64> {
    let behaviour = create_numeric_operation_behaviour(behaviour_ty, entity_ty);
    let reactive_instance = behaviour.get_reactive_instance();
    let numeric_operation = NumericOperationF64::from(reactive_instance.clone());
    numeric_operation.lhs(v);
    numeric_operation.result()
}

fn create_numeric_operation_behaviour(behaviour_ty: &BehaviourTypeId, entity_ty: &EntityTypeId) -> Arc<dyn BehaviourFsm<ReactiveEntityInstance> + Send + Sync> {
    let reactive_instance = numeric_operation(entity_ty);
    let not_function = NUMERIC_OPERATIONS_F64.get(&behaviour_ty).expect("Failed to get function");
    let not_factory = NumericOperationF64Factory::new(behaviour_ty.clone(), not_function.clone());
    not_factory.create(reactive_instance.clone()).expect("Failed to create behaviour")
}
