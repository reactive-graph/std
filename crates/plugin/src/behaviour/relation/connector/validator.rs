use std::sync::Arc;

use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveRelationInstance;
use crate::model_connector::ConnectorProperties::INBOUND_PROPERTY_NAME;
use crate::model_connector::ConnectorProperties::OUTBOUND_PROPERTY_NAME;
use crate::reactive::behaviour_validator;
use crate::reactive::BehaviourPropertyInvalid;
use crate::reactive::BehaviourPropertyValidator;
use crate::reactive::BehaviourReactiveInstanceContainer;
use crate::reactive::BehaviourValidator;
use crate::reactive::RelationPropertyValidator;

behaviour_validator!(ConnectorValidator, ReactiveRelationInstance);

impl BehaviourPropertyValidator<ReactiveRelationInstance> for ConnectorValidator {
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

impl RelationPropertyValidator for ConnectorValidator {}
