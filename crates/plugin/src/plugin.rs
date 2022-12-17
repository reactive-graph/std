use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::behaviour::entity::gate::LogicalGateFactory;
use crate::behaviour::entity::gate::LOGICAL_GATES;
use crate::behaviour::entity::if_then_else::IfThenElseFactory;
use crate::behaviour::entity::operation::LogicalOperationFactory;
use crate::behaviour::entity::operation::LOGICAL_OPERATIONS;
use crate::behaviour::entity::toggle::ToggleFactory;
use crate::behaviour::entity::trigger::TriggerFactory;
use crate::di::*;
use crate::model::EntityBehaviourTypeId;
use crate::model_logical::BEHAVIOUR_IF_THEN_ELSE;
use crate::model_logical::BEHAVIOUR_TOGGLE;
use crate::model_logical::BEHAVIOUR_TRIGGER;
use crate::model_logical::ENTITY_BEHAVIOUR_IF_THEN_ELSE;
use crate::model_logical::ENTITY_BEHAVIOUR_TOGGLE;
use crate::model_logical::ENTITY_BEHAVIOUR_TRIGGER;
use crate::plugins::component_provider;
use crate::plugins::entity_type_provider;
use crate::plugins::flow_type_provider;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::ComponentProvider;
use crate::plugins::ComponentProviderError;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::FlowTypeProvider;
use crate::plugins::FlowTypeProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginActivationError;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::plugins::PluginDeactivationError;
use crate::provider::LogicalComponentProviderImpl;
use crate::provider::LogicalEntityTypeProviderImpl;
use crate::provider::LogicalFlowTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait LogicalPlugin: Plugin + Send + Sync {}

#[module]
pub struct LogicalPluginImpl {
    component_provider: Wrc<LogicalComponentProviderImpl>,
    entity_type_provider: Wrc<LogicalEntityTypeProviderImpl>,
    flow_type_provider: Wrc<LogicalFlowTypeProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(LogicalPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl LogicalPlugin for LogicalPluginImpl {}

impl Plugin for LogicalPluginImpl {
    fn activate(&self) -> Result<(), PluginActivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();

            // If Then Else
            let factory = Arc::new(IfThenElseFactory::new(BEHAVIOUR_IF_THEN_ELSE.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_IF_THEN_ELSE.clone(), factory);

            // Toggle
            let factory = Arc::new(ToggleFactory::new(BEHAVIOUR_TOGGLE.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_TOGGLE.clone(), factory);

            // Trigger
            let factory = Arc::new(TriggerFactory::new(BEHAVIOUR_TRIGGER.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_TRIGGER.clone(), factory);

            // Logical operations
            for (behaviour_ty, f) in LOGICAL_OPERATIONS.iter() {
                entity_behaviour_registry.register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(LogicalOperationFactory::new(behaviour_ty.clone(), *f)));
            }

            // Logical gates
            for (behaviour_ty, f) in LOGICAL_GATES.iter() {
                entity_behaviour_registry.register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(LogicalGateFactory::new(behaviour_ty.clone(), *f)));
            }
        }
        Ok(())
    }

    fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_IF_THEN_ELSE);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_TOGGLE);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_TRIGGER);
            for behaviour_ty in LOGICAL_OPERATIONS.keys() {
                entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
            }
            for behaviour_ty in LOGICAL_GATES.keys() {
                entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
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

    fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
        entity_type_provider!(self.entity_type_provider)
    }

    fn get_flow_type_provider(&self) -> Result<Option<Arc<dyn FlowTypeProvider>>, FlowTypeProviderError> {
        flow_type_provider!(self.flow_type_provider)
    }
}
