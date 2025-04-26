use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::RelationPropertyValidator;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use reactive_graph_std_connector_model::ConnectorProperties::INBOUND_PROPERTY_NAME;
use reactive_graph_std_connector_model::ConnectorProperties::OUTBOUND_PROPERTY_NAME;

behaviour_validator!(ComplexConnectorValidator, RelationInstanceId, ReactiveRelation);

impl BehaviourPropertyValidator<RelationInstanceId, ReactiveRelation> for ComplexConnectorValidator {
    fn validate_properties(&self) -> Result<(), BehaviourPropertyInvalid> {
        self.validate_property(OUTBOUND_PROPERTY_NAME.as_ref())?;
        self.validate_property(INBOUND_PROPERTY_NAME.as_ref())?;
        // Dynamically validate if the properties of the outbound and inbound instances.
        let outbound_property_name = self
            .reactive_instance
            .as_string(OUTBOUND_PROPERTY_NAME.as_ref())
            .ok_or(BehaviourPropertyInvalid::PropertyMissing(OUTBOUND_PROPERTY_NAME.to_string()))?;
        self.validate_outbound_property(&outbound_property_name)?;
        let inbound_property_name = self
            .reactive_instance
            .as_string(OUTBOUND_PROPERTY_NAME.as_ref())
            .ok_or(BehaviourPropertyInvalid::PropertyMissing(OUTBOUND_PROPERTY_NAME.to_string()))?;
        self.validate_inbound_property(&inbound_property_name)?;
        Ok(())
    }
}

impl RelationPropertyValidator for ComplexConnectorValidator {}
