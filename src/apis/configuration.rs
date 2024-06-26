/*
 * Radix Gateway API - Babylon
 *
 * This API is exposed by the Babylon Radix Gateway to enable clients to efficiently query current and historic state on the RadixDLT ledger, and intelligently handle transaction submission.  It is designed for use by wallets and explorers, and for light queries from front-end dApps. For exchange/asset integrations, back-end dApp integrations, or simple use cases, you should consider using the Core API on a Node. A Gateway is only needed for reading historic snapshots of ledger states or a more robust set-up.  The Gateway API is implemented by the [Network Gateway](https://github.com/radixdlt/babylon-gateway), which is configured to read from [full node(s)](https://github.com/radixdlt/babylon-node) to extract and index data from the network.  This document is an API reference documentation, visit [User Guide](https://docs.radixdlt.com/) to learn more about how to run a Gateway of your own.  ## Migration guide  Please see [the latest release notes](https://github.com/radixdlt/babylon-gateway/releases).  ## Integration and forward compatibility guarantees  All responses may have additional fields added at any release, so clients are advised to use JSON parsers which ignore unknown fields on JSON objects.  When the Radix protocol is updated, new functionality may be added, and so discriminated unions returned by the API may need to be updated to have new variants added, corresponding to the updated data. Clients may need to update in advance to be able to handle these new variants when a protocol update comes out.  On the very rare occasions we need to make breaking changes to the API, these will be warned in advance with deprecation notices on previous versions. These deprecation notices will include a safe migration path. Deprecation notes or breaking changes will be flagged clearly in release notes for new versions of the Gateway.  The Gateway DB schema is not subject to any compatibility guarantees, and may be changed at any release. DB changes will be flagged in the release notes so clients doing custom DB integrations can prepare.
 *
 * The version of the OpenAPI document: v1.5.2
 *
 * Generated by: https://openapi-generator.tech
 */

#[cfg(test)]
use crate::transports::HttpClient;
use crate::{
    client::Client,
    transports::{CallOptions, ICHttpClient},
};
const USER_AGENT: &str = "OpenAPI-Generator/v1.5.2/rust";
const MAINNET_BASE_PATH: &str = "https://mainnet.radixdlt.com";
#[derive(Debug, Clone)]
pub struct Configuration<T: Client> {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: T,
    pub call_options: CallOptions,
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

#[cfg(test)]
impl Configuration<HttpClient> {
    pub fn new() -> Configuration<HttpClient> {
        Configuration::default()
    }
}
#[cfg(not(test))]
impl Configuration<ICHttpClient> {
    pub fn new() -> Configuration<ICHttpClient> {
        Configuration::default()
    }
}

impl Default for Configuration<ICHttpClient> {
    fn default() -> Self {
        Configuration {
            base_path: MAINNET_BASE_PATH.to_owned(),
            user_agent: Some(USER_AGENT.to_owned()),
            client: ICHttpClient::new(None),
            call_options: CallOptions::default(),
        }
    }
}
#[cfg(test)]
impl Default for Configuration<HttpClient> {
    fn default() -> Self {
        Configuration {
            base_path: MAINNET_BASE_PATH.to_owned(),
            user_agent: Some(USER_AGENT.to_owned()),
            client: HttpClient::new(),
            call_options: CallOptions::default(),
        }
    }
}
