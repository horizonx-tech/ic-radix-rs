#![allow(dead_code)]

use anyhow::Result;
use async_trait::async_trait;
use reqwest::{IntoUrl, Method, Request, Response};

use crate::client::Client;

use super::CallOptions;

#[derive(Debug, Clone)]
pub struct HttpClient {
    inner: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            inner: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Client for HttpClient {
    fn request<U: IntoUrl>(&self, method: Method, url: U) -> reqwest::RequestBuilder {
        self.inner.request(method, url)
    }

    async fn execute(&self, request: Request, _: CallOptions) -> Result<Response> {
        self.inner.execute(request).await.map_err(Into::into)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new() {
        HttpClient::new();
    }

    #[tokio::test]
    async fn test_execute() {
        let client = HttpClient::new();
        let request = client.request(Method::GET, "https://example.com");
        let response = client
            .execute(request.build().unwrap(), CallOptions::default())
            .await
            .unwrap();
        assert!(response.status().is_success());
    }
}
