use reactive_graph_model_string::BEHAVIOUR_TEMPLATING;
use reactive_graph_model_string::ENTITY_BEHAVIOUR_TEMPLATING;
use reactive_graph_plugin_api::EntityBehaviourRegistry;
use reactive_graph_plugin_api::prelude::plugin::*;
use reactive_graph_plugin_api::prelude::providers::*;

use crate::behaviour::entity::string_bool_operation::function::STRING_BOOL_OPERATIONS;
use crate::behaviour::entity::string_comparison::function::STRING_COMPARISONS;
use crate::behaviour::entity::string_gate::STRING_GATES;
use crate::behaviour::entity::string_number_operation::STRING_NUMBER_OPERATIONS;
use crate::behaviour::entity::string_string_number_gate::STRING_STRING_NUMBER_GATES;
use crate::behaviour::entity::templating::TemplatingFactory;

export_plugin!({
    "dependencies": [
        { "name": "reactive-graph-plugin-base", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-plugin-result", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait StringPlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct StringPluginImpl {
    component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,

    #[component(default = "component_provider_registry")]
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,

    entity_types_provider: Arc<dyn TypeProvider<EntityTypes> + Send + Sync>,

    #[component(default = "entity_types_provider_registry")]
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,

    #[component(default = "entity_behaviour_registry")]
    entity_behaviour_registry: Arc<dyn EntityBehaviourRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for StringPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;
        self.entity_behaviour_registry.register_all(&STRING_BOOL_OPERATIONS.get_factories()).await;
        self.entity_behaviour_registry.register_all(&STRING_COMPARISONS.get_factories()).await;
        self.entity_behaviour_registry.register_all(&STRING_GATES.get_factories()).await;
        self.entity_behaviour_registry.register_all(&STRING_NUMBER_OPERATIONS.get_factories()).await;
        self.entity_behaviour_registry.register_all(&STRING_STRING_NUMBER_GATES.get_factories()).await;
        self.entity_behaviour_registry
            .register(ENTITY_BEHAVIOUR_TEMPLATING.clone(), Arc::new(TemplatingFactory::new(BEHAVIOUR_TEMPLATING.clone())))
            .await;
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_TEMPLATING).await;
        self.entity_behaviour_registry
            .unregister_all(&STRING_STRING_NUMBER_GATES.to_entity_behaviour_tys())
            .await;
        self.entity_behaviour_registry
            .unregister_all(&STRING_NUMBER_OPERATIONS.to_entity_behaviour_tys())
            .await;
        self.entity_behaviour_registry.unregister_all(&STRING_GATES.to_entity_behaviour_tys()).await;
        self.entity_behaviour_registry
            .unregister_all(&STRING_COMPARISONS.to_entity_behaviour_tys())
            .await;
        self.entity_behaviour_registry
            .unregister_all(&STRING_BOOL_OPERATIONS.to_entity_behaviour_tys())
            .await;
        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}

// use std::sync::Arc;
// use std::sync::RwLock;
//
// use async_trait::async_trait;
//
// use crate::behaviour::entity::string_bool_operation::StringBoolOperationFactory;
// use crate::behaviour::entity::string_bool_operation::STRING_BOOL_OPERATIONS;
// use crate::behaviour::entity::string_comparison::function::STRING_COMPARISONS;
// use crate::behaviour::entity::string_comparison::StringComparisonFactory;
// use crate::behaviour::entity::string_gate::StringGateFactory;
// use crate::behaviour::entity::string_gate::STRING_GATES;
// use crate::behaviour::entity::string_number_operation::StringNumberOperationFactory;
// use crate::behaviour::entity::string_number_operation::STRING_NUMBER_OPERATIONS;
// use crate::behaviour::entity::string_operation::StringOperationFactory;
// use crate::behaviour::entity::string_operation::STRING_OPERATIONS;
// use crate::behaviour::entity::templating::TemplatingFactory;
// use crate::di::*;
// use crate::model_string::BEHAVIOUR_TEMPLATING;
// use crate::model_string::ENTITY_BEHAVIOUR_TEMPLATING;
// use crate::plugins::component_provider;
// use crate::plugins::entity_type_provider;
// use crate::plugins::plugin_context::PluginContext;
// use crate::plugins::ComponentProvider;
// use crate::plugins::ComponentProviderError;
// use crate::plugins::EntityTypeProvider;
// use crate::plugins::EntityTypeProviderError;
// use crate::plugins::Plugin;
// use crate::plugins::PluginActivationError;
// use crate::plugins::PluginContextDeinitializationError;
// use crate::plugins::PluginContextInitializationError;
// use crate::plugins::PluginDeactivationError;
// use crate::providers::StringComponentProviderImpl;
// use crate::providers::StringEntityTypeProviderImpl;
// use reactive_graph_graph::EntityBehaviourTypeId;
//
// #[wrapper]
// pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);
//
// #[provides]
// fn create_empty_plugin_context_container() -> PluginContextContainer {
//     PluginContextContainer(RwLock::new(None))
// }
//
// pub trait StringPlugin: Plugin + Send + Sync {}
//
// #[component]
// pub struct StringPluginImpl {
//     component_provider: Wrc<StringComponentProviderImpl>,
//     entity_type_provider: Wrc<StringEntityTypeProviderImpl>,
//
//     context: PluginContextContainer,
// }
//
// impl StringPluginImpl {}
//
// interfaces!(StringPluginImpl: dyn Plugin);
//
// #[provides]
// impl StringPlugin for StringPluginImpl {}
//
// #[async_trait]
// impl Plugin for StringPluginImpl {
//     async fn activate(&self) -> Result<(), PluginActivationError> {
//         let guard = self.context.0.read().unwrap();
//         if let Some(context) = guard.clone() {
//             let entity_behaviour_registry = context.get_entity_behaviour_registry();
//             // String Bool Operations
//             // fn(String) -> bool
//             for (behaviour_ty, f) in STRING_BOOL_OPERATIONS.iter() {
//                 entity_behaviour_registry
//                     .register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(StringBoolOperationFactory::new(behaviour_ty.clone(), *f)));
//             }
//             // String Comparisons
//             // fn(String, String) -> bool
//             for (behaviour_ty, f) in STRING_COMPARISONS.iter() {
//                 entity_behaviour_registry.register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(StringComparisonFactory::new(behaviour_ty.clone(), *f)));
//             }
//             // String Gates
//             // fn(String, String) -> String
//             for (behaviour_ty, f) in STRING_GATES.iter() {
//                 entity_behaviour_registry.register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(StringGateFactory::new(behaviour_ty.clone(), *f)));
//             }
//             // String Number Operations
//             // fn(String) -> Number
//             for (behaviour_ty, f) in STRING_NUMBER_OPERATIONS.iter() {
//                 entity_behaviour_registry.register(
//                     EntityBehaviourTypeId::from(behaviour_ty),
//                     Arc::new(StringNumberOperationFactory::new(behaviour_ty.clone(), *f)),
//                 );
//             }
//             // String Operations
//             // fn(String) -> String
//             for (behaviour_ty, f) in STRING_OPERATIONS.iter() {
//                 entity_behaviour_registry.register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(StringOperationFactory::new(behaviour_ty.clone(), *f)));
//             }
//             // String String Number Gates
//             // fn(String, String) -> Number
//             for (behaviour_ty, f) in STRING_OPERATIONS.iter() {
//                 entity_behaviour_registry.register(EntityBehaviourTypeId::from(behaviour_ty), Arc::new(StringOperationFactory::new(behaviour_ty.clone(), *f)));
//             }
//
//             // Templating
//             let factory = Arc::new(TemplatingFactory::new(BEHAVIOUR_TEMPLATING.clone()));
//             entity_behaviour_registry.register(ENTITY_BEHAVIOUR_TEMPLATING.clone(), factory);
//         }
//         Ok(())
//     }
//
//     async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
//         let guard = self.context.0.read().unwrap();
//         if let Some(context) = guard.clone() {
//             let entity_behaviour_registry = context.get_entity_behaviour_registry();
//             for behaviour_ty in STRING_BOOL_OPERATIONS.keys() {
//                 entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
//             }
//             for behaviour_ty in STRING_COMPARISONS.keys() {
//                 entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
//             }
//             for behaviour_ty in STRING_GATES.keys() {
//                 entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
//             }
//             for behaviour_ty in STRING_NUMBER_OPERATIONS.keys() {
//                 entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
//             }
//             for behaviour_ty in STRING_OPERATIONS.keys() {
//                 entity_behaviour_registry.unregister(&EntityBehaviourTypeId::from(behaviour_ty));
//             }
//             entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_TEMPLATING);
//         }
//         Ok(())
//     }
//
//     fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
//         self.context.0.write().unwrap().replace(context.clone());
//         Ok(())
//     }
//
//     fn remove_context(&self) -> Result<(), PluginContextDeinitializationError> {
//         let mut writer = self.context.0.write().unwrap();
//         *writer = None;
//         Ok(())
//     }
//
//     fn get_component_provider(&self) -> Result<Option<Arc<dyn ComponentProvider>>, ComponentProviderError> {
//         component_provider!(self.component_provider)
//     }
//
//     fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
//         entity_type_provider!(self.entity_type_provider)
//     }
// }
