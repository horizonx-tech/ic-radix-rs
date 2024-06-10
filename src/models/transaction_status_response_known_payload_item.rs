/*
 * Radix Gateway API - Babylon
 *
 * This API is exposed by the Babylon Radix Gateway to enable clients to efficiently query current and historic state on the RadixDLT ledger, and intelligently handle transaction submission.  It is designed for use by wallets and explorers, and for light queries from front-end dApps. For exchange/asset integrations, back-end dApp integrations, or simple use cases, you should consider using the Core API on a Node. A Gateway is only needed for reading historic snapshots of ledger states or a more robust set-up.  The Gateway API is implemented by the [Network Gateway](https://github.com/radixdlt/babylon-gateway), which is configured to read from [full node(s)](https://github.com/radixdlt/babylon-node) to extract and index data from the network.  This document is an API reference documentation, visit [User Guide](https://docs.radixdlt.com/) to learn more about how to run a Gateway of your own.  ## Migration guide  Please see [the latest release notes](https://github.com/radixdlt/babylon-gateway/releases).  ## Integration and forward compatibility guarantees  All responses may have additional fields added at any release, so clients are advised to use JSON parsers which ignore unknown fields on JSON objects.  When the Radix protocol is updated, new functionality may be added, and so discriminated unions returned by the API may need to be updated to have new variants added, corresponding to the updated data. Clients may need to update in advance to be able to handle these new variants when a protocol update comes out.  On the very rare occasions we need to make breaking changes to the API, these will be warned in advance with deprecation notices on previous versions. These deprecation notices will include a safe migration path. Deprecation notes or breaking changes will be flagged clearly in release notes for new versions of the Gateway.  The Gateway DB schema is not subject to any compatibility guarantees, and may be changed at any release. DB changes will be flagged in the release notes so clients doing custom DB integrations can prepare. 
 *
 * The version of the OpenAPI document: v1.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, candid::CandidType)]
pub struct TransactionStatusResponseKnownPayloadItem {
    /// Bech32m-encoded hash.
    #[serde(rename = "payload_hash")]
    pub payload_hash: String,
    #[serde(rename = "status")]
    pub status: models::TransactionStatus,
    #[serde(rename = "payload_status", skip_serializing_if = "Option::is_none")]
    pub payload_status: Option<models::TransactionPayloadStatus>,
    /// An additional description to clarify the payload status. 
    #[serde(rename = "payload_status_description", skip_serializing_if = "Option::is_none")]
    pub payload_status_description: Option<String>,
    /// The initial error message received for a rejection or failure during transaction execution. This will typically be the useful error message, explaining the root cause of the issue. Please note that presence of an error message doesn't imply that this payload will definitely reject or fail. This could represent an error during a temporary rejection (such as out of fees) which then gets resolved (e.g. by depositing money to pay the fee), allowing the transaction to be committed. 
    #[serde(rename = "error_message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<Option<String>>,
    /// The latest error message received for a rejection or failure during transaction execution, this is only returned if it is different from the initial error message. This is more current than the initial error message, but may be less useful, as it could be a message regarding the expiry of the transaction at the end of its epoch validity window. Please note that presence of an error message doesn't imply that this payload will definitely reject or fail. This could represent an error during a temporary rejection (such as out of fees) which then gets resolved (e.g. by depositing money to pay the fee), allowing the transaction to be committed. 
    #[serde(rename = "latest_error_message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub latest_error_message: Option<Option<String>>,
    #[serde(rename = "handling_status", skip_serializing_if = "Option::is_none")]
    pub handling_status: Option<models::TransactionPayloadGatewayHandlingStatus>,
    /// Additional reason for why the Gateway has its current handling status. 
    #[serde(rename = "handling_status_reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub handling_status_reason: Option<Option<String>>,
    /// The most recent error message received when submitting this transaction to the network. Please note that the presence of an error message doesn't imply that this transaction payload will definitely reject or fail. This could be a transient error. 
    #[serde(rename = "submission_error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub submission_error: Option<Option<String>>,
}

impl TransactionStatusResponseKnownPayloadItem {
    pub fn new(payload_hash: String, status: models::TransactionStatus) -> TransactionStatusResponseKnownPayloadItem {
        TransactionStatusResponseKnownPayloadItem {
            payload_hash,
            status,
            payload_status: None,
            payload_status_description: None,
            error_message: None,
            latest_error_message: None,
            handling_status: None,
            handling_status_reason: None,
            submission_error: None,
        }
    }
}

