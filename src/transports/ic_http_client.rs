//! IC http client

use anyhow::Result;
use async_trait::async_trait;
use candid::CandidType;
use candid::{candid_method, Principal};
use derive_builder::Builder;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
    TransformContext, TransformFunc,
};
use reqwest::{IntoUrl, Method, Request, Response};
use serde::{self, Deserialize, Serialize};

use crate::client::Client;

#[derive(Clone, Debug)]
pub struct ICHttpClient {
    pub max_response_bytes: u64,
}

#[derive(Builder, Default, Clone, Debug, PartialEq, Eq)]
pub struct CallOptions {
    max_resp: Option<u64>,
    cycles: Option<u64>,
    transform: Option<TransformContext>,
}

impl ICHttpClient {
    pub fn new(max_resp: Option<u64>) -> Self {
        ICHttpClient {
            max_response_bytes: if let Some(v) = max_resp { v } else { 500_000 },
        }
    }

    pub fn set_max_response_bytes(&mut self, v: u64) {
        self.max_response_bytes = v;
    }

    async fn request(
        &self,
        url: String,
        req_type: HttpMethod,
        payload: &Request,
        options: CallOptions,
    ) -> Result<HttpResponse> {
        let body = payload
            .body()
            .map(|b| b.as_bytes().map(|b| b.to_vec()))
            .flatten();
        let headers: Vec<HttpHeader> = payload
            .headers()
            .iter()
            .map(|(k, v)| HttpHeader {
                name: k.to_string(),
                value: v.to_str().unwrap_or_default().to_string(),
            })
            .collect();
        let request = CanisterHttpRequestArgument {
            url: url.clone(),
            max_response_bytes: if let Some(v) = options.max_resp {
                Some(v)
            } else {
                Some(self.max_response_bytes)
            },
            method: req_type,
            headers,
            body,
            transform: match options.transform {
                Some(t) => Some(t),
                None => Some(TransformContext {
                    function: TransformFunc(candid::Func {
                        principal: ic_cdk::api::id(),
                        method: "transform".to_string(),
                    }),
                    context: vec![],
                }),
            },
        };

        let cycles = http_request_required_cycles(&request);
        match http_request(request.clone(), cycles).await {
            Ok((result,)) => Ok(result),
            Err((_, m)) => {
                anyhow::bail!("Failed to make http request: {}", m);
            }
        }
    }

    pub async fn get(
        &self,
        url: String,
        payload: &Request,
        options: CallOptions,
    ) -> Result<HttpResponse> {
        self.request(url, HttpMethod::GET, &payload, options).await
    }

    pub async fn post(
        &self,
        url: String,
        payload: &Request,
        options: CallOptions,
    ) -> Result<HttpResponse> {
        self.request(url, HttpMethod::POST, payload, options).await
    }
}

#[async_trait]
impl Client for ICHttpClient {
    fn request<U: IntoUrl>(&self, method: Method, url: U) -> reqwest::RequestBuilder {
        reqwest::Client::new().request(method, url)
    }

    async fn execute(&self, request: Request, options: CallOptions) -> Result<HttpResponse> {
        let method = request.method().clone();
        match method {
            Method::GET => self.get(request.url().to_string(), &request, options).await,
            Method::POST => {
                self.post(request.url().to_string(), &request, options)
                    .await
            }
            _ => anyhow::bail!("Unsupported method: {:?}", method),
        }
    }
}

// Calcurate cycles for http_request
// NOTE:
//   v0.11: https://github.com/dfinity/cdk-rs/blob/0b14facb80e161de79264c8f88b1a0c8e18ffcb6/examples/management_canister/src/caller/lib.rs#L7-L19
//   v0.8: https://github.com/dfinity/cdk-rs/blob/a8454cb37420c200c7b224befd6f68326a01442e/src/ic-cdk/src/api/management_canister/http_request.rs#L290-L299
fn http_request_required_cycles(arg: &CanisterHttpRequestArgument) -> u128 {
    let max_response_bytes = match arg.max_response_bytes {
        Some(ref n) => *n as u128,
        None => 2 * 1024 * 1024u128, // default 2MiB
    };
    let arg_raw = candid::utils::encode_args((arg,)).expect("Failed to encode arguments.");
    // The fee is for a 13-node subnet to demonstrate a typical usage.
    (3_000_000u128
        + 60_000u128 * 13
        + (arg_raw.len() as u128 + "http_request".len() as u128) * 400
        + max_response_bytes * 800)
        * 13
}
