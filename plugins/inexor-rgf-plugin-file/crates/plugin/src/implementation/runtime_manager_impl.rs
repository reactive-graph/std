use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use tokio::runtime::Handle;
use tokio::runtime::Runtime;

use crate::api::RuntimeManager;
use crate::di::*;
use crate::plugins::PluginContext;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[wrapper]
pub struct RuntimeContainer(Runtime);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

#[provides]
fn create_runtime() -> RuntimeContainer {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .thread_name("inexor-filesystem")
        .build()
        .unwrap();
    RuntimeContainer(runtime)
}

#[component]
pub struct RuntimeManagerImpl {
    runtime: RuntimeContainer,
    context: PluginContextContainer,
}

impl RuntimeManagerImpl {}

#[async_trait]
#[provides]
impl RuntimeManager for RuntimeManagerImpl {
    fn init(&self) {}

    fn shutdown(&self) {}

    fn set_context(&self, context: Arc<dyn PluginContext>) {
        self.context.0.write().unwrap().replace(context.clone());
    }

    fn get_handle(&self) -> &Handle {
        self.runtime.0.handle()
    }
}
