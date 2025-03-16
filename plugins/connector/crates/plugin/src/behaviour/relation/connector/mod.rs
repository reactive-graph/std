use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::relation_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use serde_json::Value;

pub use function::CONNECTOR_BEHAVIOURS;
pub use function::ConnectorFunction;
use reactive_graph_model_connector::ConnectorProperties::INBOUND_PROPERTY_NAME;
use reactive_graph_model_connector::ConnectorProperties::OUTBOUND_PROPERTY_NAME;

use crate::behaviour::relation::connector::validator::ConnectorValidator;

pub mod function;
pub mod validator;

relation_behaviour!(
    Connector,
    ConnectorFactory,
    ConnectorFsm,
    ConnectorBehaviourTransitions,
    ConnectorValidator,
    f,
    ConnectorFunction
);

impl ConnectorBehaviourTransitions {
    fn get_outbound_property_name(&self) -> Option<String> {
        self.reactive_instance.as_string(OUTBOUND_PROPERTY_NAME.as_ref())
    }

    fn get_inbound_property_name(&self) -> Option<String> {
        self.reactive_instance.as_string(INBOUND_PROPERTY_NAME.as_ref())
    }
}

impl BehaviourInit<RelationInstanceId, ReactiveRelation> for ConnectorBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let outbound_property_name = self.get_outbound_property_name().ok_or(BehaviourInitializationFailed {})?;
        let inbound_property_name = self.get_inbound_property_name().ok_or(BehaviourInitializationFailed {})?;
        let value = self
            .reactive_instance
            .outbound
            .get(outbound_property_name)
            .ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        self.reactive_instance.inbound.set(inbound_property_name.clone(), f(&value));
        Ok(())
    }
}

impl BehaviourShutdown<RelationInstanceId, ReactiveRelation> for ConnectorBehaviourTransitions {}

impl BehaviourConnect<RelationInstanceId, ReactiveRelation> for ConnectorBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let outbound_property_name = self.get_outbound_property_name().ok_or(BehaviourConnectFailed {})?;
        let inbound_property_name = self.get_inbound_property_name().ok_or(BehaviourConnectFailed {})?;
        let f = self.f;
        let inbound = self.reactive_instance.inbound.clone();
        self.outbound_property_observers
            .observe_with_handle(&outbound_property_name, move |value: &Value| {
                inbound.set(inbound_property_name.clone(), f(value));
            });
        Ok(())
    }
}

impl BehaviourTransitions<RelationInstanceId, ReactiveRelation> for ConnectorBehaviourTransitions {}
