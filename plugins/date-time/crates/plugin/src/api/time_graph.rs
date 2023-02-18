use std::sync::Arc;

use async_trait::async_trait;

use crate::plugins::PluginContext;

#[async_trait]
pub trait TimeGraph: Send + Sync {
    fn init(&self);

    fn shutdown(&self);

    fn set_context(&self, context: Arc<dyn PluginContext>);
}
