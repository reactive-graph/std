use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::relation_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use serde_json::Value;

pub use function::COMPLEX_CONNECTOR_BEHAVIOURS;
pub use function::ComplexConnectorFunction;
use reactive_graph_model_connector::ConnectorProperties::INBOUND_PROPERTY_NAME;
use reactive_graph_model_connector::ConnectorProperties::OUTBOUND_PROPERTY_NAME;

use crate::behaviour::relation::complex_connector::validator::ComplexConnectorValidator;

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

impl BehaviourInit<RelationInstanceId, ReactiveRelation> for ComplexConnectorBehaviourTransitions {}

impl BehaviourShutdown<RelationInstanceId, ReactiveRelation> for ComplexConnectorBehaviourTransitions {}

impl BehaviourConnect<RelationInstanceId, ReactiveRelation> for ComplexConnectorBehaviourTransitions {
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

impl BehaviourTransitions<RelationInstanceId, ReactiveRelation> for ComplexConnectorBehaviourTransitions {}
