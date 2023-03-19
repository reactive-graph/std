use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::behaviour::entity::counter::CounterFactory;
use crate::behaviour::entity::gate::behaviour_f64::ArithmeticGateF64Factory;
use crate::behaviour::entity::gate::behaviour_i64::ArithmeticGateI64Factory;
use crate::behaviour::entity::gate::behaviour_u64::ArithmeticGateU64Factory;
use crate::behaviour::entity::gate::function::ARITHMETIC_GATES_F64;
use crate::behaviour::entity::gate::function::ARITHMETIC_GATES_I64;
use crate::behaviour::entity::gate::function::ARITHMETIC_GATES_U64;
use crate::behaviour::entity::operation::behaviour_f64::ArithmeticOperationF64Factory;
use crate::behaviour::entity::operation::behaviour_i64::ArithmeticOperationI64Factory;
use crate::behaviour::entity::operation::behaviour_u64::ArithmeticOperationU64Factory;
use crate::behaviour::entity::operation::function::ARITHMETIC_OPERATIONS_F64;
use crate::behaviour::entity::operation::function::ARITHMETIC_OPERATIONS_I64;
use crate::behaviour::entity::operation::function::ARITHMETIC_OPERATIONS_U64;
use crate::di::*;
use crate::model::EntityBehaviourTypeId;
use crate::model_arithmetic::BEHAVIOUR_COUNTER;
use crate::model_arithmetic::ENTITY_BEHAVIOUR_COUNTER;
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
use crate::providers::ArithmeticComponentProviderImpl;
use crate::providers::ArithmeticEntityTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait ArithmeticPlugin: Plugin + Send + Sync {}

#[module]
pub struct ArithmeticPluginImpl {
    component_provider: Wrc<ArithmeticComponentProviderImpl>,
    entity_type_provider: Wrc<ArithmeticEntityTypeProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(ArithmeticPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl ArithmeticPlugin for ArithmeticPluginImpl {}

#[async_trait]
impl Plugin for ArithmeticPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();

            // Counter
            let factory = Arc::new(CounterFactory::new(BEHAVIOUR_COUNTER.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_COUNTER.clone(), factory);

            // Arithmetic operations (f64)
            for (behaviour_ty, f) in ARITHMETIC_OPERATIONS_F64.iter() {
                entity_behaviour_registry.register(
                    EntityBehaviourTypeId::from(behaviour_ty),
                    Arc::new(ArithmeticOperationF64Factory::new(behaviour_ty.clone(), *f)),
                );
            }

            // Arithmetic operations (i64)
            for (behaviour_ty, f) in ARITHMETIC_OPERATIONS_I64.iter() {
                entity_behaviour_registry.register(
                    EntityBehaviourTypeId::from(behaviour_ty),
                    Arc::new(ArithmeticOperationI64Factory::new(behaviour_ty.clone(), *f)),
                );
            }

            // Arithmetic operations (u64)
            for (behaviour_ty, f) in ARITHMETIC_OPERATIONS_U64.iter() {
                entity_behaviour_registry.register(
                    EntityBehaviourTypeId::from(behaviour_ty),
                    Arc::new(ArithmeticOperationU64Factory::new(behaviour_ty.clone(), *f)),
                );
            }

            // Arithmetic gates (f64)
            for (behaviour_ty, f) in ARITHMETIC_GATES_F64.iter() {
                entity_behaviour_registry
                    .register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(ArithmeticGateF64Factory::new(behaviour_ty.clone(), *f)));
            }

            // Arithmetic gates (i64)
            for (behaviour_ty, f) in ARITHMETIC_GATES_I64.iter() {
                entity_behaviour_registry
                    .register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(ArithmeticGateI64Factory::new(behaviour_ty.clone(), *f)));
            }

            // Arithmetic gates (u64)
            for (behaviour_ty, f) in ARITHMETIC_GATES_U64.iter() {
                entity_behaviour_registry
                    .register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(ArithmeticGateU64Factory::new(behaviour_ty.clone(), *f)));
            }
        }
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_COUNTER);
            for behaviour_ty in ARITHMETIC_OPERATIONS_F64.keys() {
                entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
            }
            for behaviour_ty in ARITHMETIC_OPERATIONS_I64.keys() {
                entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
            }
            for behaviour_ty in ARITHMETIC_OPERATIONS_U64.keys() {
                entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
            }
            for behaviour_ty in ARITHMETIC_GATES_F64.keys() {
                entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
            }
            for behaviour_ty in ARITHMETIC_GATES_I64.keys() {
                entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
            }
            for behaviour_ty in ARITHMETIC_GATES_U64.keys() {
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
}
