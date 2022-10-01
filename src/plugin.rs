use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::behaviour::component::component_behaviour_provider::ConnectorComponentBehaviourProviderImpl;
use crate::behaviour::relation::relation_behaviour_provider::ConnectorRelationBehaviourProviderImpl;
use crate::di::*;
use crate::plugins::component_behaviour_provider;
use crate::plugins::component_provider;
use crate::plugins::plugin::PluginMetadata;
use crate::plugins::plugin::PluginMetadataError;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::plugin_metadata;
use crate::plugins::relation_behaviour_provider;
use crate::plugins::relation_type_provider;
use crate::plugins::ComponentBehaviourProvider;
use crate::plugins::ComponentBehaviourProviderError;
use crate::plugins::ComponentProvider;
use crate::plugins::ComponentProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginContextInitializationError;
use crate::plugins::RelationBehaviourProvider;
use crate::plugins::RelationBehaviourProviderError;
use crate::plugins::RelationTypeProvider;
use crate::plugins::RelationTypeProviderError;
use crate::provider::ConnectorComponentProviderImpl;
use crate::provider::ConnectorRelationTypeProviderImpl;

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
    component_behaviour_provider: Wrc<ConnectorComponentBehaviourProviderImpl>,
    relation_behaviour_provider: Wrc<ConnectorRelationBehaviourProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(ConnectorPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl ConnectorPlugin for ConnectorPluginImpl {}

impl Plugin for ConnectorPluginImpl {
    fn metadata(&self) -> Result<PluginMetadata, PluginMetadataError> {
        plugin_metadata!("inexor-rgf-plugin-base")
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context);
        Ok(())
    }

    fn get_component_provider(&self) -> Result<Option<Arc<dyn ComponentProvider>>, ComponentProviderError> {
        component_provider!(self.component_provider)
    }

    fn get_relation_type_provider(&self) -> Result<Option<Arc<dyn RelationTypeProvider>>, RelationTypeProviderError> {
        relation_type_provider!(self.relation_type_provider)
    }

    fn get_component_behaviour_provider(&self) -> Result<Option<Arc<dyn ComponentBehaviourProvider>>, ComponentBehaviourProviderError> {
        component_behaviour_provider!(self.component_behaviour_provider)
    }

    fn get_relation_behaviour_provider(&self) -> Result<Option<Arc<dyn RelationBehaviourProvider>>, RelationBehaviourProviderError> {
        relation_behaviour_provider!(self.relation_behaviour_provider)
    }
}
