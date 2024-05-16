use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use ic_cdk::api::management_canister::http_request::{HttpHeader, HttpResponse};
use serde::Serialize;

use crate::{reqwest::Method, transports::CallOptions};
#[async_trait]
pub trait Client: Send + Sync {
    fn request(&self, method: Method, url: &str) -> RequestBuilder;
    async fn execute(&self, request: Request, options: CallOptions) -> Result<HttpResponse>;
}

#[derive(Debug, Clone)]
pub struct Request {
    pub headers: Vec<HttpHeader>,
    pub url: String,
    pub body: Option<Vec<u8>>,
    pub method: Method,
}

#[cfg(test)]
impl Into<outer_reqwest::Request> for Request {
    fn into(self) -> outer_reqwest::Request {
        let mut req = outer_reqwest::Request::new(
            self.method.into(),
            url::Url::from_str(self.url.as_str()).unwrap(),
        );
        for header in self.headers {
            req.headers_mut().insert(
                outer_reqwest::header::HeaderName::from_str(&header.clone().name).unwrap(),
                outer_reqwest::header::HeaderValue::from_str(&header.value).unwrap(),
            );
        }
        if let Some(body) = self.body {
            req.body_mut().replace(outer_reqwest::Body::from(body));
        }
        req
    }
}

pub struct RequestBuilder {
    pub headers: Vec<HttpHeader>,
    pub url: String,
    pub body: Option<Vec<u8>>,
    pub method: Method,
}
impl RequestBuilder {
    pub fn new(method: Method, url: &str) -> Self {
        RequestBuilder {
            headers: vec![],
            url: url.to_string(),
            body: None,
            method,
        }
    }
    pub fn build(self) -> Request {
        Request {
            headers: self.headers,
            url: self.url,
            body: self.body,
            method: self.method,
        }
    }
    pub fn header(mut self, name: String, value: String) -> Self {
        self.headers.push(HttpHeader { name, value });
        self
    }
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }
    pub fn json<T: Serialize>(self, body: &T) -> anyhow::Result<Self> {
        let body = serde_json::to_vec(body)?;
        Ok(self.body(body))
    }
}
