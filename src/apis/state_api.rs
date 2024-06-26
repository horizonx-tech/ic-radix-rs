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
use crate::{
    apis::ResponseContent,
    client::Client,
    invoke,
    invoker::{self, Invoker},
    models,
};

/// struct for typed errors of method [`account_authorized_depositors_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountAuthorizedDepositorsPageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`account_resource_preferences_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountResourcePreferencesPageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`entity_fungible_resource_vault_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EntityFungibleResourceVaultPageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`entity_fungibles_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EntityFungiblesPageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`entity_metadata_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EntityMetadataPageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`entity_non_fungible_ids_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EntityNonFungibleIdsPageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`entity_non_fungible_resource_vault_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EntityNonFungibleResourceVaultPageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`entity_non_fungibles_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EntityNonFungiblesPageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`entity_schema_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EntitySchemaPageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`key_value_store_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeyValueStoreDataError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`key_value_store_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeyValueStoreKeysError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`non_fungible_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NonFungibleDataError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`non_fungible_ids`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NonFungibleIdsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`non_fungible_location`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NonFungibleLocationError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`package_blueprint_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PackageBlueprintPageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`package_code_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PackageCodePageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`state_entity_details`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StateEntityDetailsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`state_validators_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StateValidatorsListError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Returns paginable collection of authorized depositors for given account.
pub async fn account_authorized_depositors_page<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_account_authorized_depositors_page_request: models::StateAccountAuthorizedDepositorsPageRequest,
) -> Result<models::StateAccountAuthorizedDepositorsPageResponse> {
    invoke!(
        configuration,
        state_account_authorized_depositors_page_request,
        "/state/account/page/authorized-depositors"
    )
}

/// Returns paginable collection of resource preference rules for given account.
pub async fn account_resource_preferences_page<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_account_resource_preferences_page_request: models::StateAccountResourcePreferencesPageRequest,
) -> Result<models::StateAccountResourcePreferencesPageResponse> {
    invoke!(
        configuration,
        state_account_resource_preferences_page_request,
        "/state/account/page/resource-preferences"
    )
}

/// Returns vaults for fungible resource owned by a given global entity. The returned response is in a paginated format, ordered by the resource's first appearance on the ledger.
pub async fn entity_fungible_resource_vault_page<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_entity_fungible_resource_vaults_page_request: models::StateEntityFungibleResourceVaultsPageRequest,
) -> Result<models::StateEntityFungibleResourceVaultsPageResponse> {
    invoke!(
        configuration,
        state_entity_fungible_resource_vaults_page_request,
        "/state/entity/page/fungible-vaults/"
    )
}

/// Returns the total amount of each fungible resource owned by a given global entity. Result can be aggregated globally or per vault. The returned response is in a paginated format, ordered by the resource's first appearance on the ledger.
pub async fn entity_fungibles_page<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_entity_fungibles_page_request: models::StateEntityFungiblesPageRequest,
) -> Result<models::StateEntityFungiblesPageResponse> {
    invoke!(
        configuration,
        state_entity_fungibles_page_request,
        "/state/entity/page/fungibles/"
    )
}

/// Returns all the metadata properties associated with a given global entity. The returned response is in a paginated format, ordered by first appearance on the ledger.
pub async fn entity_metadata_page<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_entity_metadata_page_request: models::StateEntityMetadataPageRequest,
) -> Result<models::StateEntityMetadataPageResponse> {
    invoke!(
        configuration,
        state_entity_metadata_page_request,
        "/state/entity/page/metadata"
    )
}

/// Returns all non-fungible IDs of a given non-fungible resource owned by a given entity. The returned response is in a paginated format, ordered by the resource's first appearence on the ledger.
pub async fn entity_non_fungible_ids_page<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_entity_non_fungible_ids_page_request: models::StateEntityNonFungibleIdsPageRequest,
) -> Result<models::StateEntityNonFungibleIdsPageResponse> {
    invoke!(
        configuration,
        state_entity_non_fungible_ids_page_request,
        "/state/entity/page/non-fungible-vault/ids"
    )
}

/// Returns vaults for non fungible resource owned by a given global entity. The returned response is in a paginated format, ordered by the resource's first appearance on the ledger.
pub async fn entity_non_fungible_resource_vault_page<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_entity_non_fungible_resource_vaults_page_request: models::StateEntityNonFungibleResourceVaultsPageRequest,
) -> Result<models::StateEntityNonFungibleResourceVaultsPageResponse> {
    invoke!(
        configuration,
        state_entity_non_fungible_resource_vaults_page_request,
        "/state/entity/page/non-fungible-vaults/"
    )
}

/// Returns the total amount of each non-fungible resource owned by a given global entity. Result can be aggregated globally or per vault. The returned response is in a paginated format, ordered by the resource's first appearance on the ledger.
pub async fn entity_non_fungibles_page<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_entity_non_fungibles_page_request: models::StateEntityNonFungiblesPageRequest,
) -> Result<models::StateEntityNonFungiblesPageResponse> {
    invoke!(
        configuration,
        state_entity_non_fungibles_page_request,
        "/state/entity/page/non-fungibles/"
    )
}

/// Returns all the schemas associated with a given global entity. The returned response is in a paginated format, ordered by first appearance on the ledger.
pub async fn entity_schema_page<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_entity_schema_page_request: models::StateEntitySchemaPageRequest,
) -> Result<models::StateEntitySchemaPageResponse> {
    invoke!(
        configuration,
        state_entity_schema_page_request,
        "/state/entity/page/schemas"
    )
}

/// Returns data (value) associated with a given key of a given key-value store. [Check detailed documentation for explanation](#section/How-to-query-the-content-of-a-key-value-store-inside-a-component)
pub async fn key_value_store_data<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_key_value_store_data_request: models::StateKeyValueStoreDataRequest,
) -> Result<models::StateKeyValueStoreDataResponse> {
    invoke!(
        configuration,
        state_key_value_store_data_request,
        "/state/key-value-store/data"
    )
}

/// Allows to iterate over key value store keys.
pub async fn key_value_store_keys<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_key_value_store_keys_request: models::StateKeyValueStoreKeysRequest,
) -> Result<models::StateKeyValueStoreKeysResponse> {
    invoke!(
        configuration,
        state_key_value_store_keys_request,
        "/state/key-value-store/keys"
    )
}

/// Returns data associated with a given non-fungible ID of a given non-fungible resource.
pub async fn non_fungible_data<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_non_fungible_data_request: models::StateNonFungibleDataRequest,
) -> Result<models::StateNonFungibleDataResponse> {
    invoke!(
        configuration,
        state_non_fungible_data_request,
        "/state/non-fungible/data"
    )
}

/// Returns the non-fungible IDs of a given non-fungible resource. Returned response is in a paginated format, ordered by their first appearance on the ledger.
pub async fn non_fungible_ids<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_non_fungible_ids_request: models::StateNonFungibleIdsRequest,
) -> Result<models::StateNonFungibleIdsResponse> {
    invoke!(
        configuration,
        state_non_fungible_ids_request,
        "/state/non-fungible/ids"
    )
}

/// Returns location of a given non-fungible ID.
pub async fn non_fungible_location<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_non_fungible_location_request: models::StateNonFungibleLocationRequest,
) -> Result<models::StateNonFungibleLocationResponse> {
    invoke!(
        configuration,
        state_non_fungible_location_request,
        "/state/non-fungible/location"
    )
}

/// Returns all the blueprints associated with a given package entity. The returned response is in a paginated format, ordered by first appearance on the ledger.
pub async fn package_blueprint_page<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_package_blueprint_page_request: models::StatePackageBlueprintPageRequest,
) -> Result<models::StatePackageBlueprintPageResponse> {
    invoke!(
        configuration,
        state_package_blueprint_page_request,
        "/state/package/page/blueprints"
    )
}

/// Returns all the codes associated with a given package entity. The returned response is in a paginated format, ordered by first appearance on the ledger.
pub async fn package_code_page<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_package_code_page_request: models::StatePackageCodePageRequest,
) -> Result<models::StatePackageCodePageResponse> {
    invoke!(
        configuration,
        state_package_code_page_request,
        "/state/package/page/codes"
    )
}

/// Returns detailed information for collection of entities. Aggregate resources globally by default.
pub async fn state_entity_details<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_entity_details_request: models::StateEntityDetailsRequest,
) -> Result<models::StateEntityDetailsResponse> {
    invoke!(
        configuration,
        state_entity_details_request,
        "/state/entity/details"
    )
}

pub async fn state_validators_list<T: Client>(
    configuration: &configuration::Configuration<T>,
    state_validators_list_request: models::StateValidatorsListRequest,
) -> Result<models::StateValidatorsListResponse> {
    invoke!(
        configuration,
        state_validators_list_request,
        "/state/validators/list"
    )
}
