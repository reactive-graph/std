use std::sync::Arc;
use std::sync::RwLock;

use crate::behaviour::entity::string_bool_operation::StringBoolOperationFactory;
use crate::behaviour::entity::string_bool_operation::STRING_BOOL_OPERATIONS;
use crate::behaviour::entity::string_comparison::function::STRING_COMPARISONS;
use crate::behaviour::entity::string_comparison::StringComparisonFactory;
use crate::behaviour::entity::string_gate::StringGateFactory;
use crate::behaviour::entity::string_gate::STRING_GATES;
use crate::behaviour::entity::string_number_operation::StringNumberOperationFactory;
use crate::behaviour::entity::string_number_operation::STRING_NUMBER_OPERATIONS;
use crate::behaviour::entity::string_operation::StringOperationFactory;
use crate::behaviour::entity::string_operation::STRING_OPERATIONS;
use crate::behaviour::entity::templating::TemplatingFactory;
use crate::di::*;
use crate::model::EntityBehaviourTypeId;
use crate::model_string::BEHAVIOUR_TEMPLATING;
use crate::model_string::ENTITY_BEHAVIOUR_TEMPLATING;
use crate::plugins::component_provider;
use crate::plugins::entity_type_provider;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::ComponentProvider;
use crate::plugins::ComponentProviderError;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginActivationError;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::plugins::PluginDeactivationError;
use crate::providers::StringComponentProviderImpl;
use crate::providers::StringEntityTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

pub trait StringPlugin: Plugin + Send + Sync {}

#[module]
pub struct StringPluginImpl {
    component_provider: Wrc<StringComponentProviderImpl>,
    entity_type_provider: Wrc<StringEntityTypeProviderImpl>,

    context: PluginContextContainer,
}

impl StringPluginImpl {}

interfaces!(StringPluginImpl: dyn Plugin);

#[provides]
impl StringPlugin for StringPluginImpl {}

impl Plugin for StringPluginImpl {
    fn activate(&self) -> Result<(), PluginActivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();
            // String Bool Operations
            // fn(String) -> bool
            for (behaviour_ty, f) in STRING_BOOL_OPERATIONS.iter() {
                entity_behaviour_registry
                    .register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(StringBoolOperationFactory::new(behaviour_ty.clone(), *f)));
            }
            // String Comparisons
            // fn(String, String) -> bool
            for (behaviour_ty, f) in STRING_COMPARISONS.iter() {
                entity_behaviour_registry.register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(StringComparisonFactory::new(behaviour_ty.clone(), *f)));
            }
            // String Gates
            // fn(String, String) -> String
            for (behaviour_ty, f) in STRING_GATES.iter() {
                entity_behaviour_registry.register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(StringGateFactory::new(behaviour_ty.clone(), *f)));
            }
            // String Number Operations
            // fn(String) -> Number
            for (behaviour_ty, f) in STRING_NUMBER_OPERATIONS.iter() {
                entity_behaviour_registry.register(
                    EntityBehaviourTypeId::from(behaviour_ty),
                    Arc::new(StringNumberOperationFactory::new(behaviour_ty.clone(), *f)),
                );
            }
            // String Operations
            // fn(String) -> String
            for (behaviour_ty, f) in STRING_OPERATIONS.iter() {
                entity_behaviour_registry.register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(StringOperationFactory::new(behaviour_ty.clone(), *f)));
            }
            // String String Number Gates
            // fn(String, String) -> Number
            for (behaviour_ty, f) in STRING_OPERATIONS.iter() {
                entity_behaviour_registry.register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(StringOperationFactory::new(behaviour_ty.clone(), *f)));
            }

            // Templating
            let factory = Arc::new(TemplatingFactory::new(BEHAVIOUR_TEMPLATING.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_TEMPLATING.clone(), factory);
        }
        Ok(())
    }

    fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();
            for behaviour_ty in STRING_BOOL_OPERATIONS.keys() {
                entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
            }
            for behaviour_ty in STRING_COMPARISONS.keys() {
                entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
            }
            for behaviour_ty in STRING_GATES.keys() {
                entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
            }
            for behaviour_ty in STRING_NUMBER_OPERATIONS.keys() {
                entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
            }
            for behaviour_ty in STRING_OPERATIONS.keys() {
                entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
            }
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_TEMPLATING);
        }
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context.clone());
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
}
