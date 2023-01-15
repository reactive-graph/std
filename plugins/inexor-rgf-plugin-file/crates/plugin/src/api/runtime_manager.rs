use std::sync::Arc;

use async_trait::async_trait;
use tokio::runtime::Handle;

use crate::plugins::PluginContext;

#[async_trait]
pub trait RuntimeManager: Send + Sync {
    fn init(&self);

    fn shutdown(&self);

    fn set_context(&self, context: Arc<dyn PluginContext>);

    fn get_handle(&self) -> &Handle;
}
