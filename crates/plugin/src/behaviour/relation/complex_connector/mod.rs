use serde_json::Value;

pub use function::ComplexConnectorFunction;
pub use function::COMPLEX_CONNECTOR_BEHAVIOURS;

use crate::behaviour::relation::complex_connector::validator::ComplexConnectorValidator;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveBehaviourContainer;
use crate::model::ReactiveRelationInstance;
use crate::model_connector::ConnectorProperties::INBOUND_PROPERTY_NAME;
use crate::model_connector::ConnectorProperties::OUTBOUND_PROPERTY_NAME;
use crate::reactive::*;

pub mod function;
pub mod validator;

relation_behaviour!(
    ComplexConnector,
    ComplexConnectorFactory,
    ComplexConnectorFsm,
    ComplexConnectorBehaviourTransitions,
    ComplexConnectorValidator,
    f,
    ComplexConnectorFunction
);

impl ComplexConnectorBehaviourTransitions {
    fn get_outbound_property_name(&self) -> Option<String> {
        self.reactive_instance.as_string(OUTBOUND_PROPERTY_NAME.as_ref())
    }

    fn get_inbound_property_name(&self) -> Option<String> {
        self.reactive_instance.as_string(INBOUND_PROPERTY_NAME.as_ref())
    }
}

impl BehaviourInit<ReactiveRelationInstance> for ComplexConnectorBehaviourTransitions {}

impl BehaviourShutdown<ReactiveRelationInstance> for ComplexConnectorBehaviourTransitions {}

impl BehaviourConnect<ReactiveRelationInstance> for ComplexConnectorBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let outbound_property_name = self.get_outbound_property_name().ok_or(BehaviourConnectFailed {})?;
        let inbound_property_name = self.get_inbound_property_name().ok_or(BehaviourConnectFailed {})?;
        let f = self.f;
        let relation_instance = self.reactive_instance.clone();
        self.outbound_property_observers
            .observe_with_handle(&outbound_property_name, move |value: &Value| {
                // The ComplexConnectorFunction have to propagate
                f(value.clone(), inbound_property_name.clone(), relation_instance.clone());
            });
        Ok(())
    }
}

impl BehaviourTransitions<ReactiveRelationInstance> for ComplexConnectorBehaviourTransitions {}
