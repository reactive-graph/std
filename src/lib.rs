use crate::plugin::BasePlugin;
use crate::plugins::{Plugin, PluginError};
use inexor_rgf_core_model as model;
use inexor_rgf_core_plugins as plugins;
use std::sync::Arc;
use waiter_di::{profiles, Container, Provider};

pub mod plugin;
pub mod provider;

#[macro_use]
extern crate query_interface;

pub fn get<T>() -> Container<T> {
    Container::<T>::new()
}

pub fn construct_plugin() -> Result<Arc<dyn Plugin>, PluginError> {
    let mut container = get::<profiles::Default>();
    let container = &mut container;
    let plugin = Provider::<dyn BasePlugin>::create(container);
    let plugin = Arc::new(plugin);
    let plugin: Result<Arc<dyn Plugin>, _> =
        <dyn query_interface::Object>::query_arc(plugin.clone());
    if plugin.is_err() {
        return Err(PluginError::PluginCreationError);
    }
    Ok(plugin.unwrap())
}

plugins::export_plugin!(register);

extern "C" fn register(registrar: &mut dyn plugins::PluginRegistrar) {
    let plugin = construct_plugin();
    match plugin {
        Ok(plugin) => {
            registrar.register_plugin("base", Box::new(plugin));
        }
        Err(_) => {}
    }
}
