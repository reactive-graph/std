use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::api::TimeGraph;
use crate::behaviour::UtcNowFactory;
use crate::behaviour::UtcTimestampFactory;
use crate::di::*;
use crate::model_date_time::BEHAVIOUR_UTC_NOW;
use crate::model_date_time::BEHAVIOUR_UTC_TIMESTAMP;
use crate::model_date_time::ENTITY_BEHAVIOUR_UTC_NOW;
use crate::model_date_time::ENTITY_BEHAVIOUR_UTC_TIMESTAMP;
use crate::plugins::entity_type_provider;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::relation_type_provider;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginActivationError;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::plugins::PluginDeactivationError;
use crate::plugins::RelationTypeProvider;
use crate::plugins::RelationTypeProviderError;
use crate::providers::DateTimeEntityTypeProviderImpl;
use crate::providers::DateTimeRelationTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait DateTimePlugin: Plugin + Send + Sync {}

#[module]
pub struct DateTimePluginImpl {
    entity_type_provider: Wrc<DateTimeEntityTypeProviderImpl>,
    relation_type_provider: Wrc<DateTimeRelationTypeProviderImpl>,
    time_graph: Wrc<dyn TimeGraph>,
    context: PluginContextContainer,
}

impl DateTimePluginImpl {}

interfaces!(DateTimePluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl DateTimePlugin for DateTimePluginImpl {}

#[async_trait]
impl Plugin for DateTimePluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        {
            let guard = self.context.0.read().unwrap();
            if let Some(context) = guard.clone() {
                let entity_behaviour_registry = context.get_entity_behaviour_registry();
                // Utc Timestamp
                let factory = Arc::new(UtcTimestampFactory::new(BEHAVIOUR_UTC_TIMESTAMP.clone()));
                entity_behaviour_registry.register(ENTITY_BEHAVIOUR_UTC_TIMESTAMP.clone(), factory);
                // Utc Now
                let factory = Arc::new(UtcNowFactory::new(BEHAVIOUR_UTC_NOW.clone()));
                entity_behaviour_registry.register(ENTITY_BEHAVIOUR_UTC_NOW.clone(), factory);
            }
        }
        self.time_graph.init().await;
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.time_graph.shutdown().await;
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_UTC_TIMESTAMP);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_UTC_NOW);
        }
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context.clone());
        self.time_graph.set_context(context.clone());
        Ok(())
    }

    fn remove_context(&self) -> Result<(), PluginContextDeinitializationError> {
        let mut writer = self.context.0.write().unwrap();
        *writer = None;
        Ok(())
    }

    fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
        entity_type_provider!(self.entity_type_provider)
    }

    fn get_relation_type_provider(&self) -> Result<Option<Arc<dyn RelationTypeProvider>>, RelationTypeProviderError> {
        relation_type_provider!(self.relation_type_provider)
    }
}
