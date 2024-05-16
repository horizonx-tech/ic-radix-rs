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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommittedTransactionInfo {
    #[serde(rename = "state_version")]
    pub state_version: i64,
    #[serde(rename = "epoch")]
    pub epoch: i64,
    #[serde(rename = "round")]
    pub round: i64,
    #[serde(rename = "round_timestamp")]
    pub round_timestamp: String,
    #[serde(rename = "transaction_status")]
    pub transaction_status: models::TransactionStatus,
    /// Bech32m-encoded hash.
    #[serde(rename = "payload_hash", skip_serializing_if = "Option::is_none")]
    pub payload_hash: Option<String>,
    /// Bech32m-encoded hash.
    #[serde(rename = "intent_hash", skip_serializing_if = "Option::is_none")]
    pub intent_hash: Option<String>,
    /// String-encoded decimal representing the amount of a related fungible resource.
    #[serde(rename = "fee_paid", skip_serializing_if = "Option::is_none")]
    pub fee_paid: Option<String>,
    #[serde(rename = "affected_global_entities", skip_serializing_if = "Option::is_none")]
    pub affected_global_entities: Option<Vec<String>>,
    #[serde(rename = "confirmed_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub confirmed_at: Option<Option<String>>,
    #[serde(rename = "error_message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<Option<String>>,
    /// Hex-encoded binary blob.
    #[serde(rename = "raw_hex", skip_serializing_if = "Option::is_none")]
    pub raw_hex: Option<String>,
    #[serde(rename = "receipt", skip_serializing_if = "Option::is_none")]
    pub receipt: Option<Box<models::TransactionReceipt>>,
    /// A text-representation of a transaction manifest. This field will be present only for user transactions and when explicitly opted-in using `manifest_instructions` flag. 
    #[serde(rename = "manifest_instructions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub manifest_instructions: Option<Option<String>>,
    /// A collection of zero or more manifest classes ordered from the most specific class to the least specific one. This field will be present only for user transactions. 
    #[serde(rename = "manifest_classes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub manifest_classes: Option<Option<Vec<models::ManifestClass>>>,
    /// The optional transaction message. This type is defined in the Core API as `TransactionMessage`. See the Core API documentation for more details. 
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<serde_json::Value>,
    #[serde(rename = "balance_changes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub balance_changes: Option<Option<Box<models::TransactionBalanceChanges>>>,
}

impl CommittedTransactionInfo {
    pub fn new(state_version: i64, epoch: i64, round: i64, round_timestamp: String, transaction_status: models::TransactionStatus) -> CommittedTransactionInfo {
        CommittedTransactionInfo {
            state_version,
            epoch,
            round,
            round_timestamp,
            transaction_status,
            payload_hash: None,
            intent_hash: None,
            fee_paid: None,
            affected_global_entities: None,
            confirmed_at: None,
            error_message: None,
            raw_hex: None,
            receipt: None,
            manifest_instructions: None,
            manifest_classes: None,
            message: None,
            balance_changes: None,
        }
    }
}
