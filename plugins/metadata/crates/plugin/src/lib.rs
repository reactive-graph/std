#![feature(associated_type_bounds)]
#![feature(lazy_cell)]
#![feature(register_tool)]
#![register_tool(tarpaulin)]

pub mod plugin;
pub mod providers;

// #![feature(register_tool)]
// #![register_tool(tarpaulin)]
// #![allow(clippy::map_entry)]
// 
// #[macro_use]
// extern crate query_interface;
// 
// use std::any::Any;
// use std::any::TypeId;
// use std::collections::HashMap;
// use std::sync::Arc;
// 
// use inexor_rgf_core_di as di;
// use inexor_rgf_core_di::inject;
// use inexor_rgf_core_di::RcAny;
// use inexor_rgf_core_di::Wrc;
// use inexor_rgf_graph as model;
// use inexor_rgf_plugin_api as plugins;
// use log::error;
// 
// use crate::di::profiles;
// use crate::di::Container;
// use crate::di::Provider;
// use crate::plugin::MetaDataPlugin;
// use crate::plugin::PluginContextContainer;
// use crate::plugins::Plugin;
// use crate::plugins::PluginContext;
// use crate::plugins::PluginDependency;
// use crate::plugins::PluginLoadingError;
// 
// pub mod plugin;
// pub mod providers;
// 
// pub static PLUGIN_NAME: &str = env!("CARGO_PKG_NAME");
// pub static PLUGIN_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
// pub static PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
// 
// pub fn get<T>(context: Arc<dyn PluginContext>) -> Container<T> {
//     Container::<T>::new_with_context(Arc::new(context))
// }
// 
// // pub fn get<T>(context: Arc<dyn PluginContext>) -> Container<T> {
// //     let context_container = PluginContextContainer::new(context);
// //     let mut components: HashMap<TypeId, RcAny> = HashMap::new();
// //     let ty = TypeId::of::<Arc<dyn PluginContext>>();
// //     let component = inexor_rgf_core_di::Wrc::new(context_container);
// //     components.insert(ty, component);
// //     Container::<T>::new_with_components(components)
// //
// //     // pub type RcAny = Wrc<dyn Any + Send + Sync>;
// //     // let context: Arc<dyn PluginContext> = Arc::from(context);
// //     // let x = context.downcast::
// //     // let context: RcAny = context_container;
// // }
// 
// impl<T> Provider<PluginContextContainer> for Container<T> {
//     type Impl = PluginContextContainer;
// 
//     fn get(&mut self) -> Wrc<Self::Impl> {
//         let ty = TypeId::of::<Arc<dyn PluginContext>>();
//         let any = self.components.get(&ty).unwrap();
//         return any.clone().downcast::<PluginContextContainer>().unwrap();
//     }
// 
//     fn create(&mut self) -> Self::Impl {
//         panic!("X");
//     }
// }
// 
// pub fn construct_plugin(context: Arc<dyn PluginContext>) -> Result<Arc<dyn Plugin>, PluginLoadingError> {
//     let mut container = get::<profiles::Default>(context);
//     let container = &mut container;
//     let plugin = Provider::<dyn MetaDataPlugin>::create(container);
//     let plugin = Arc::new(plugin);
//     let plugin: Result<Arc<dyn Plugin>, _> = <dyn query_interface::Object>::query_arc(plugin);
//     if plugin.is_err() {
//         error!("Failed to construct plugin");
//         return Err(PluginLoadingError::PluginContainerInitializationError);
//     }
//     Ok(plugin.unwrap())
// }
// 
// plugins::export_plugin!(register, get_dependencies, PLUGIN_NAME, PLUGIN_DESCRIPTION, PLUGIN_VERSION);
// 
// #[allow(improper_ctypes_definitions)]
// extern "C" fn register(registrar: &mut dyn plugins::PluginRegistrar) {
//     if let Err(error) = log4rs::init_file("config/logging.toml", Default::default()) {
//         println!("Failed to configure logger in {}: {}", PLUGIN_NAME, error);
//     }
//     if let Ok(plugin) = construct_plugin(registrar.context()) {
//         registrar.register_plugin(Box::new(plugin));
//     }
// }
// 
// #[allow(improper_ctypes_definitions)]
// extern "C" fn get_dependencies() -> Vec<PluginDependency> {
//     vec![PluginDependency::new("inexor-rgf-plugin-taxonomy", ">=0.9.0, <0.10.0")]
// }
// 
// // #[cfg(test)]
// // use inexor_rgf_graph as model;
// // use inexor_rgf_plugin_api::PluginContext;
// //
// // #[cfg(test)]
// // use inexor_rgf_model_metadata as model_metadata;
// //
// // #[cfg(test)]
// // #[tarpaulin::ignore]
// // pub mod tests;
