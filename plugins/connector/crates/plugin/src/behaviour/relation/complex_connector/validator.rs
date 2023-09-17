use inexor_rgf_behaviour::RelationPropertyValidator;
use inexor_rgf_behaviour_api::behaviour_validator;
use inexor_rgf_behaviour_api::prelude::*;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_model_connector::ConnectorProperties::INBOUND_PROPERTY_NAME;
use inexor_rgf_model_connector::ConnectorProperties::OUTBOUND_PROPERTY_NAME;
use inexor_rgf_reactive::ReactiveRelation;

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
