#[macro_use]
extern crate query_interface;

use std::sync::Arc;

use crate::di::{profiles, Container, Provider};
use inexor_rgf_core_di as di;
use inexor_rgf_core_frp as frp;
use inexor_rgf_core_model as model;
use inexor_rgf_core_plugins as plugins;
use inexor_rgf_core_reactive as reactive;
use log::error;

use crate::plugin::JsonPlugin;
use crate::plugins::{Plugin, PluginError};

pub mod behaviour;
pub mod plugin;
pub mod provider;

pub fn get<T>() -> Container<T> {
    Container::<T>::new()
}

pub fn construct_plugin() -> Result<Arc<dyn Plugin>, PluginError> {
    let mut container = get::<profiles::Default>();
    let container = &mut container;
    let plugin = Provider::<dyn JsonPlugin>::create(container);
    let plugin = Arc::new(plugin);
    let plugin: Result<Arc<dyn Plugin>, _> = <dyn query_interface::Object>::query_arc(plugin.clone());
    if plugin.is_err() {
        error!("Failed to construct plugin");
        return Err(PluginError::PluginCreationError);
    }
    Ok(plugin.unwrap())
}

plugins::export_plugin!(register);

#[allow(improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn plugins::PluginRegistrar) {
    let logger_result = log4rs::init_file("config/logging.yml", Default::default());
    match logger_result {
        Err(error) => {
            println!("Failed to configure logger: {}", error);
        }
        _ => {}
    }

    let plugin = construct_plugin();
    match plugin {
        Ok(plugin) => {
            const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
            registrar.register_plugin(PKG_NAME, Box::new(plugin));
        }
        Err(_) => {}
    }
}
