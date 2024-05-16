/*
 * Radix Gateway API - Babylon
 *
 * This API is exposed by the Babylon Radix Gateway to enable clients to efficiently query current and historic state on the RadixDLT ledger, and intelligently handle transaction submission.  It is designed for use by wallets and explorers, and for light queries from front-end dApps. For exchange/asset integrations, back-end dApp integrations, or simple use cases, you should consider using the Core API on a Node. A Gateway is only needed for reading historic snapshots of ledger states or a more robust set-up.  The Gateway API is implemented by the [Network Gateway](https://github.com/radixdlt/babylon-gateway), which is configured to read from [full node(s)](https://github.com/radixdlt/babylon-node) to extract and index data from the network.  This document is an API reference documentation, visit [User Guide](https://docs.radixdlt.com/) to learn more about how to run a Gateway of your own.  ## Migration guide  Please see [the latest release notes](https://github.com/radixdlt/babylon-gateway/releases).  ## Integration and forward compatibility guarantees  All responses may have additional fields added at any release, so clients are advised to use JSON parsers which ignore unknown fields on JSON objects.  When the Radix protocol is updated, new functionality may be added, and so discriminated unions returned by the API may need to be updated to have new variants added, corresponding to the updated data. Clients may need to update in advance to be able to handle these new variants when a protocol update comes out.  On the very rare occasions we need to make breaking changes to the API, these will be warned in advance with deprecation notices on previous versions. These deprecation notices will include a safe migration path. Deprecation notes or breaking changes will be flagged clearly in release notes for new versions of the Gateway.  The Gateway DB schema is not subject to any compatibility guarantees, and may be changed at any release. DB changes will be flagged in the release notes so clients doing custom DB integrations can prepare.
 *
 * The version of the OpenAPI document: v1.5.2
 *
 * Generated by: https://openapi-generator.tech
 */

use std::str::FromStr;

use anyhow::Result;

use super::{configuration, Error, StatusCode};
use crate::{apis::ResponseContent, client::Client, invoke, invoke_without_request, models};

/// struct for typed errors of method [`account_deposit_pre_validation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountDepositPreValidationError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transaction_committed_details`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionCommittedDetailsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transaction_construction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionConstructionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transaction_preview`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionPreviewError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transaction_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionStatusError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transaction_submit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionSubmitError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Helper endpoint that allows pre-validation if a deposit of certain resources to a given account can succeed or not. It is only meant for pre-validation usage, it does not guarantee that execution will succeed.
pub async fn account_deposit_pre_validation<T: Client>(
    configuration: &configuration::Configuration<T>,
    account_deposit_pre_validation_request: models::AccountDepositPreValidationRequest,
) -> Result<models::AccountDepositPreValidationResponse> {
    invoke!(
        configuration,
        account_deposit_pre_validation_request,
        "/transaction/account-deposit-pre-validation"
    )
}

/// Returns the committed details and receipt of the transaction for a given transaction identifier. Transaction identifiers which don't correspond to a committed transaction will return a `TransactionNotFoundError`.
pub async fn transaction_committed_details<T: Client>(
    configuration: &configuration::Configuration<T>,
    transaction_committed_details_request: models::TransactionCommittedDetailsRequest,
) -> Result<models::TransactionCommittedDetailsResponse> {
    invoke!(
        configuration,
        transaction_committed_details_request,
        "/transaction/committed-details"
    )
}

/// Returns information needed to construct a new transaction including current `epoch` number.
pub async fn transaction_construction<T: Client>(
    configuration: &configuration::Configuration<T>,
) -> Result<models::TransactionConstructionResponse> {
    invoke_without_request!(configuration, "/transaction/construction")
}

/// Previews transaction against the network. This endpoint is effectively a proxy towards the Core API `/v0/transaction/preview` endpoint. See the Core API documentation for more details.
pub async fn transaction_preview<T: Client>(
    configuration: &configuration::Configuration<T>,
    transaction_preview_request: models::TransactionPreviewRequest,
) -> Result<models::TransactionPreviewResponse> {
    invoke!(
        configuration,
        transaction_preview_request,
        "/transaction/preview"
    )
}

/// Returns overall transaction status and all of its known payloads based on supplied intent hash.
pub async fn transaction_status<T: Client>(
    configuration: &configuration::Configuration<T>,
    transaction_status_request: models::TransactionStatusRequest,
) -> Result<models::TransactionStatusResponse> {
    invoke!(
        configuration,
        transaction_status_request,
        "/transaction/status"
    )
}

/// Submits a signed transaction payload to the network.
pub async fn transaction_submit<T: Client>(
    configuration: &configuration::Configuration<T>,
    transaction_submit_request: models::TransactionSubmitRequest,
) -> Result<models::TransactionSubmitResponse> {
    invoke!(
        configuration,
        transaction_submit_request,
        "/transaction/submit"
    )
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_construction_metadata() {
        let cfg = configuration::Configuration::new();
        let res = transaction_construction(&cfg).await;
        assert!(res.is_ok());
    }
}
