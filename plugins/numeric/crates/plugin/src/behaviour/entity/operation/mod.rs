pub use function::NumericOperationFunction;

pub mod behaviour_f64;
pub mod behaviour_i64;
pub mod function;

#[cfg(test)]
pub mod tests {
    use inexor_rgf_graph::prelude::*;
    use inexor_rgf_reactive::ReactiveEntity;
    use inexor_rgf_reactive::ReactiveProperties;
    use serde_json::json;
    use uuid::Uuid;

    use inexor_rgf_model_numeric::NumericOperationProperties::LHS;
    use inexor_rgf_model_numeric::COMPONENT_NUMERIC_OPERATION;
    use inexor_rgf_model_result::ResultNumberF64Properties::RESULT;
    use inexor_rgf_model_result::COMPONENT_RESULT_NUMBER;

    pub fn numeric_operation(ty: &EntityTypeId) -> ReactiveEntity {
        let id = Uuid::new_v4();
        let properties = PropertyInstances::new().property(LHS, json!(0.0)).property(RESULT, json!(0.0));
        let components = ComponentTypeIds::new()
            .component(COMPONENT_NUMERIC_OPERATION.clone())
            .component(COMPONENT_RESULT_NUMBER.clone());

        ReactiveEntity::builder()
            .ty(ty)
            .id(id)
            .properties(ReactiveProperties::new_with_id_from_properties(id, properties))
            .components(components)
            .build()
    }
}
