#![allow(clippy::map_entry)]

#[macro_use]
extern crate query_interface;

use std::sync::Arc;

use inexor_rgf_core_di as di;
use inexor_rgf_core_model as model;
use inexor_rgf_core_plugins as plugins;
use inexor_rgf_core_reactive as reactive;
use log::error;

use crate::di::profiles;
use crate::di::Container;
use crate::di::Provider;
use crate::plugin::FilePlugin;
use crate::plugins::Plugin;
use crate::plugins::PluginError;

pub mod api;
pub mod behaviour;
pub mod implementation;
pub mod plugin;
pub mod provider;

pub fn get<T>() -> Container<T> {
    Container::<T>::new()
}

pub fn construct_plugin() -> Result<Arc<dyn Plugin>, PluginError> {
    let mut container = get::<profiles::Default>();
    let container = &mut container;
    let plugin = Provider::<dyn FilePlugin>::create(container);
    let plugin = Arc::new(plugin);
    let plugin: Result<Arc<dyn Plugin>, _> = <dyn query_interface::Object>::query_arc(plugin);
    if plugin.is_err() {
        error!("Failed to construct plugin");
        return Err(PluginError::PluginCreationError);
    }
    Ok(plugin.unwrap())
}

plugins::export_plugin!(register);

#[allow(improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn plugins::PluginRegistrar) {
    const PKG_NAME: &str = env!("CARGO_PKG_NAME");
    if let Err(error) = log4rs::init_file("config/logging.toml", Default::default()) {
        println!("Failed to configure logger in {}: {}", PKG_NAME, error);
    }
    if let Ok(plugin) = construct_plugin() {
        registrar.register_plugin(PKG_NAME, Box::new(plugin));
    }
}
