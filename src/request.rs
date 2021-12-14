use crate::client::Client;
use crate::error::Result;
use async_trait::async_trait;
use jsonrpc_core::types::params::Params;

#[async_trait]
pub trait HttpRequest {
    async fn get_api(&self, request_url: &str, params: &Params) -> Result<serde_json::Value>;

    async fn post_api(&self, request_url: &str, params: &Params) -> Result<serde_json::Value>;

    async fn delete_api(&self, request_url: &str, params: &Params) -> Result<serde_json::Value>;
}

#[async_trait]
impl HttpRequest for Client {
    async fn get_api(&self, request_url: &str, params: &Params) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        self.get(request_url, params).await
    }

    async fn post_api(&self, request_url: &str, params: &Params) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        self.post(request_url, params).await
    }

    async fn delete_api(&self, request_url: &str, params: &Params) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        self.delete(request_url, params).await
    }
}
