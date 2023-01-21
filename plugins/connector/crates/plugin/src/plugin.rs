use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::behaviour::component::PropagationCounterFactory;
use crate::behaviour::relation::complex_connector::function::COMPLEX_CONNECTOR_BEHAVIOURS;
use crate::behaviour::relation::complex_connector::ComplexConnectorFactory;
use crate::behaviour::relation::connector::ConnectorFactory;
use crate::behaviour::relation::connector::CONNECTOR_BEHAVIOURS;
use crate::di::*;
use crate::model::ComponentBehaviourTypeId;
use crate::model::RelationBehaviourTypeId;
use crate::model_connector::BEHAVIOUR_PROPAGATION_COUNTER;
use crate::model_connector::COMPONENT_BEHAVIOUR_PROPAGATION_COUNTER;
use crate::plugins::component_provider;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::relation_type_provider;
use crate::plugins::ComponentProvider;
use crate::plugins::ComponentProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginActivationError;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::plugins::PluginDeactivationError;
use crate::plugins::RelationTypeProvider;
use crate::plugins::RelationTypeProviderError;
use crate::providers::ConnectorComponentProviderImpl;
use crate::providers::ConnectorRelationTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait ConnectorPlugin: Plugin + Send + Sync {}

#[module]
pub struct ConnectorPluginImpl {
    component_provider: Wrc<ConnectorComponentProviderImpl>,
    relation_type_provider: Wrc<ConnectorRelationTypeProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(ConnectorPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl ConnectorPlugin for ConnectorPluginImpl {}

impl Plugin for ConnectorPluginImpl {
    fn activate(&self) -> Result<(), PluginActivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let relation_component_behaviour_registry = context.get_relation_component_behaviour_registry();
            let relation_behaviour_registry = context.get_relation_behaviour_registry();
            // PropagationCounter
            let factory = Arc::new(PropagationCounterFactory::new(BEHAVIOUR_PROPAGATION_COUNTER.clone()));
            relation_component_behaviour_registry.register(COMPONENT_BEHAVIOUR_PROPAGATION_COUNTER.clone(), factory);
            // Connector
            for (behaviour_ty, f) in CONNECTOR_BEHAVIOURS.iter() {
                relation_behaviour_registry.register(RelationBehaviourTypeId::from(behaviour_ty), Arc::new(ConnectorFactory::new(behaviour_ty.clone(), *f)));
            }
            // Complex Connector
            for (behaviour_ty, f) in COMPLEX_CONNECTOR_BEHAVIOURS.iter() {
                relation_behaviour_registry
                    .register(RelationBehaviourTypeId::from(behaviour_ty), Arc::new(ComplexConnectorFactory::new(behaviour_ty.clone(), *f)));
            }
        }
        Ok(())
    }

    fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let relation_component_behaviour_registry = context.get_relation_component_behaviour_registry();
            // PropagationCounter
            relation_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_PROPAGATION_COUNTER);
            // Connector
            for behaviour_ty in CONNECTOR_BEHAVIOURS.keys() {
                relation_component_behaviour_registry.unregister(&ComponentBehaviourTypeId::from(behaviour_ty));
            }
            // Complex Connector
            for behaviour_ty in COMPLEX_CONNECTOR_BEHAVIOURS.keys() {
                relation_component_behaviour_registry.unregister(&ComponentBehaviourTypeId::from(behaviour_ty));
            }
        }
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context);
        Ok(())
    }

    fn remove_context(&self) -> Result<(), PluginContextDeinitializationError> {
        let mut writer = self.context.0.write().unwrap();
        *writer = None;
        Ok(())
    }

    fn get_component_provider(&self) -> Result<Option<Arc<dyn ComponentProvider>>, ComponentProviderError> {
        component_provider!(self.component_provider)
    }

    fn get_relation_type_provider(&self) -> Result<Option<Arc<dyn RelationTypeProvider>>, RelationTypeProviderError> {
        relation_type_provider!(self.relation_type_provider)
    }
}
