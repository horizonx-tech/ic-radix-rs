use anyhow::Result;
use async_trait::async_trait;
use reqwest::{IntoUrl, Method, Request, Response};

#[async_trait]
pub trait Client {
    fn request<U: IntoUrl>(&self, method: Method, url: U) -> reqwest::RequestBuilder;
    async fn execute(&self, request: Request) -> Result<Response>;
}
