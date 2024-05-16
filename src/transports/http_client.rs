#![allow(dead_code)]

use anyhow::{Ok, Result};
use async_trait::async_trait;
use ic_cdk::api::management_canister::http_request::{HttpHeader, HttpResponse};

use crate::{
    client::{Client, Request, RequestBuilder},
    reqwest::Method,
};

use super::CallOptions;
#[cfg(test)]
#[derive(Debug, Clone)]
pub struct HttpClient {
    inner: outer_reqwest::Client,
}

#[cfg(test)]
impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            inner: outer_reqwest::Client::new(),
        }
    }
}

#[cfg(test)]
#[async_trait]
impl Client for HttpClient {
    fn request(&self, method: Method, url: &str) -> RequestBuilder {
        RequestBuilder::new(method, url)
    }

    async fn execute(&self, request: Request, _: CallOptions) -> Result<HttpResponse> {
        let result = self.inner.execute(request.into()).await?;
        let headers = result
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or_default().to_string()))
            .map(|(k, v)| HttpHeader { name: k, value: v })
            .collect();
        let status = result.status().as_u16().into();
        let body = result.bytes().await?.to_vec();
        Ok(HttpResponse {
            body,
            headers,
            status,
        })
    }
}

#[cfg(test)]
mod test {
    use candid::Nat;

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
            .execute(request.build(), CallOptions::default())
            .await
            .unwrap();
        assert_eq!(response.status, Nat::from(200_u8));
    }
}
