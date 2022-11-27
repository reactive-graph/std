use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::di::*;
use crate::plugins::component_provider;
use crate::plugins::entity_type_provider;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::relation_type_provider;
use crate::plugins::ComponentProvider;
use crate::plugins::ComponentProviderError;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::plugins::RelationTypeProvider;
use crate::plugins::RelationTypeProviderError;
use crate::providers::TaxonomyComponentProviderImpl;
use crate::providers::TaxonomyEntityTypeProviderImpl;
use crate::providers::TaxonomyRelationTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[async_trait]
pub trait TaxonomyPlugin: Plugin + Send + Sync {}

#[module]
pub struct TaxonomyPluginImpl {
    component_provider: Wrc<TaxonomyComponentProviderImpl>,
    entity_type_provider: Wrc<TaxonomyEntityTypeProviderImpl>,
    relation_type_provider: Wrc<TaxonomyRelationTypeProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(TaxonomyPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl TaxonomyPlugin for TaxonomyPluginImpl {}

impl Plugin for TaxonomyPluginImpl {
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

    fn get_relation_type_provider(&self) -> Result<Option<Arc<dyn RelationTypeProvider>>, RelationTypeProviderError> {
        relation_type_provider!(self.relation_type_provider)
    }
}
