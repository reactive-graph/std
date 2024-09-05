pub use function::NumericGateFunction;

pub mod behaviour_f64;
pub mod function;

#[cfg(test)]
pub mod tests {
    use reactive_graph_graph::prelude::*;
    use reactive_graph_reactive_model_impl::ReactiveEntity;
    use reactive_graph_reactive_model_impl::ReactiveProperties;
    use serde_json::json;
    use uuid::Uuid;

    use reactive_graph_model_numeric::NumericGateProperties::LHS;
    use reactive_graph_model_numeric::NumericGateProperties::RHS;
    use reactive_graph_model_numeric::COMPONENT_NUMERIC_GATE;
    use reactive_graph_model_result::ResultNumberF64Properties::RESULT;
    use reactive_graph_model_result::COMPONENT_RESULT_NUMBER;

    pub fn numeric_gate(ty: &EntityTypeId) -> ReactiveEntity {
        let id = Uuid::new_v4();
        let properties = PropertyInstances::new()
            .property(LHS, json!(0.0))
            .property(RHS, json!(0.0))
            .property(RESULT, json!(0.0));
        let components = ComponentTypeIds::new()
            .component(COMPONENT_NUMERIC_GATE.clone())
            .component(COMPONENT_RESULT_NUMBER.clone());

        ReactiveEntity::builder()
            .ty(ty)
            .id(id)
            .properties(ReactiveProperties::new_with_id_from_properties(id, properties))
            .components(components)
            .build()
    }
}
