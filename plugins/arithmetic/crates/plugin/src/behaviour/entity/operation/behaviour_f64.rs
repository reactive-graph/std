use inexor_rgf_behaviour::entity_behaviour;
use inexor_rgf_behaviour::PropertyObserverContainer;
use inexor_rgf_behaviour_api::behaviour_validator;
use inexor_rgf_behaviour_api::prelude::*;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive::ReactiveEntity;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_arithmetic::ArithmeticOperationProperties::LHS;
use inexor_rgf_model_arithmetic::ArithmeticOperationProperties::RESULT;

use crate::behaviour::as_f64;
use crate::behaviour::entity::operation::function::ArithmeticOperationF64Function;

entity_behaviour!(
    ArithmeticOperationF64,
    ArithmeticOperationF64Factory,
    ArithmeticOperationF64Fsm,
    ArithmeticOperationF64BehaviourTransitions,
    ArithmeticOperationF64Validator,
    f,
    ArithmeticOperationF64Function
);

behaviour_validator!(ArithmeticOperationF64Validator, Uuid, ReactiveEntity, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ArithmeticOperationF64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self.reactive_instance.get(LHS).and_then(as_f64).ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        self.reactive_instance.set(RESULT, json!(f(lhs)));
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ArithmeticOperationF64BehaviourTransitions {
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

impl BehaviourShutdown<Uuid, ReactiveEntity> for ArithmeticOperationF64BehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ArithmeticOperationF64BehaviourTransitions {}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use inexor_rgf_behaviour_api::prelude::*;
    use inexor_rgf_graph::prelude::*;
    use inexor_rgf_reactive::ReactiveEntity;
    use inexor_rgf_reactive::ReactiveProperties;
    use inexor_rgf_reactive_api::prelude::*;
    use serde_json::json;
    use uuid::Uuid;

    use inexor_rgf_model_arithmetic::ArithmeticOperationProperties;
    use inexor_rgf_model_arithmetic::NAMESPACE_ARITHMETIC_F64;

    use crate::behaviour::entity::operation::behaviour_f64::ArithmeticOperationF64;
    use crate::behaviour::entity::operation::function::*;

    const LHS: ArithmeticOperationProperties = ArithmeticOperationProperties::LHS;
    const RESULT: ArithmeticOperationProperties = ArithmeticOperationProperties::RESULT;

    #[test]
    fn arithmetic_operation_behaviour_test() {
        let lhs: f64 = -10.0;
        let incremented_value = test_arithmetic_operation_behaviour("increment", FN_INCREMENT_F64, lhs);
        assert_eq!(-9.0, incremented_value);
        let decremented_value = test_arithmetic_operation_behaviour("decrement", FN_DECREMENT_F64, lhs);
        assert_eq!(-11.0, decremented_value);
    }

    fn test_arithmetic_operation_behaviour(type_name: &str, f: ArithmeticOperationFunction<f64>, v: f64) -> f64 {
        let ty = EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, type_name);
        let behaviour = create_arithmetic_operation_behaviour(ty, f);
        let reactive_instance = behaviour.get_reactive_instance();
        let arithmetic_operation = inexor_rgf_model_arithmetic::ArithmeticOperationF64::from(reactive_instance.clone());
        arithmetic_operation.lhs(v);
        arithmetic_operation.result().expect("Result is not of type f64")
    }

    fn create_arithmetic_operation_behaviour(ty: EntityTypeId, f: ArithmeticOperationFunction<f64>) -> Arc<ArithmeticOperationF64> {
        let behaviour_ty = BehaviourTypeId::from(NamespacedType::from(&ty));
        let reactive_instance = create_arithmetic_operation_entity(ty.clone());
        ArithmeticOperationF64::new(reactive_instance, behaviour_ty, f).expect("Failed to create ArithmeticOperationF64")
    }

    fn create_arithmetic_operation_entity(ty: EntityTypeId) -> ReactiveEntity {
        let id = Uuid::new_v4();
        let properties = PropertyInstances::new().property(LHS, json!(0.0)).property(RESULT, json!(0.0));
        ReactiveEntity::builder()
            .ty(ty)
            .id(id)
            .properties(ReactiveProperties::new_with_id_from_properties(id, properties))
            .build()
    }
}
