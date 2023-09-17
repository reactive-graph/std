use async_trait::async_trait;
use inexor_rgf_plugin_api::injectable;

#[injectable]
#[async_trait]
pub trait TimeGraph: Send + Sync {
    async fn init(&self);

    async fn shutdown(&self);
}
