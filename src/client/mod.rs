use anyhow::Result;
use async_trait::async_trait;
use reqwest::{IntoUrl, Method, Request, Response};

use crate::transports::CallOptions;

#[async_trait]
pub trait Client: Send + Sync {
    fn request<U: IntoUrl>(&self, method: Method, url: U) -> reqwest::RequestBuilder;
    async fn execute(&self, request: Request, options: CallOptions) -> Result<Response>;
}
