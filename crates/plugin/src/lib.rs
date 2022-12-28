#![allow(clippy::map_entry)]

#[macro_use]
extern crate query_interface;

use std::sync::Arc;

use inexor_rgf_core_di as di;
use inexor_rgf_core_model as model;
use inexor_rgf_core_plugins as plugins;
use inexor_rgf_core_reactive as reactive;
use inexor_rgf_model_config as model_config;

use log::error;

use crate::di::profiles;
use crate::di::Container;
use crate::di::Provider;
use crate::plugin::ConfigPlugin;
use crate::plugins::Plugin;
use crate::plugins::PluginDependency;
use crate::plugins::PluginLoadingError;

pub mod behaviour;
pub mod plugin;
pub mod providers;

pub static PLUGIN_NAME: &str = env!("CARGO_PKG_NAME");
pub static PLUGIN_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub static PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn get<T>() -> Container<T> {
    Container::<T>::new()
}

pub fn construct_plugin() -> Result<Arc<dyn Plugin>, PluginLoadingError> {
    let mut container = get::<profiles::Default>();
    let container = &mut container;
    let plugin = Provider::<dyn ConfigPlugin>::create(container);
    let plugin = Arc::new(plugin);
    let plugin: Result<Arc<dyn Plugin>, _> = <dyn query_interface::Object>::query_arc(plugin);
    match plugin {
        Ok(plugin) => Ok(plugin),
        Err(_) => {
            error!("Failed to construct plugin {}", PLUGIN_NAME);
            Err(PluginLoadingError::PluginContainerInitializationError)
        }
    }
}

plugins::export_plugin!(register, get_dependencies, PLUGIN_NAME, PLUGIN_DESCRIPTION, PLUGIN_VERSION);

#[allow(improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn plugins::PluginRegistrar) {
    if let Err(error) = log4rs::init_file("config/logging.toml", Default::default()) {
        println!("Failed to configure logger in {}: {}", PLUGIN_NAME, error);
    }
    if let Ok(plugin) = construct_plugin() {
        registrar.register_plugin(Box::new(plugin));
    }
}

#[allow(improper_ctypes_definitions)]
extern "C" fn get_dependencies() -> Vec<PluginDependency> {
    vec![
        PluginDependency::new("inexor-rgf-plugin-base", ">=0.8.0, <0.9.0"),
        PluginDependency::new("inexor-rgf-plugin-logical", ">=0.8.0, <0.9.0"),
        // PluginDependency::new("inexor-rgf-plugin-file", ">=0.8.0, <0.9.0")
    ]
}
