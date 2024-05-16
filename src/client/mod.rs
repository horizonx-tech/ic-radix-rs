use anyhow::Result;
use async_trait::async_trait;
use ic_cdk::api::management_canister::http_request::{HttpHeader, HttpResponse};
use reqwest::{IntoUrl, Method, Response};

use crate::transports::CallOptions;
#[async_trait]
pub trait Client: Send + Sync {
    fn request<U: IntoUrl>(&self, method: Method, url: U) -> reqwest::RequestBuilder;
    async fn execute(
        &self,
        request: reqwest::Request,
        options: CallOptions,
    ) -> Result<HttpResponse>;
}
