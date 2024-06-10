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

/// ManifestClass : High-level manifest class type:   * `General`: A general manifest that involves any amount of arbitrary components and packages where nothing more concrete can be said about the manifest and its nature.   * `Transfer`: A manifest of a 1-to-1 transfer to a one-to-many transfer of resources.   * `PoolContribution`: A manifest that contributed some amount of resources to a liquidity pool that can be a one-resource pool, two-resource pool, or a multi-resource pool.   * `PoolRedemption`: A manifest that redeemed resources from a liquidity pool. Similar to contributions, this can be any of the three pool blueprints available in the pool package.   * `ValidatorStake`: A manifest where XRD is staked to one or more validators.   * `ValidatorUnstake`: A manifest where XRD is unstaked from one or more validators.   * `ValidatorClaim`: A manifest where XRD is claimed from one or more validators.   * `AccountDepositSettingsUpdate`: A manifest that updated the deposit settings of the account. 
/// High-level manifest class type:   * `General`: A general manifest that involves any amount of arbitrary components and packages where nothing more concrete can be said about the manifest and its nature.   * `Transfer`: A manifest of a 1-to-1 transfer to a one-to-many transfer of resources.   * `PoolContribution`: A manifest that contributed some amount of resources to a liquidity pool that can be a one-resource pool, two-resource pool, or a multi-resource pool.   * `PoolRedemption`: A manifest that redeemed resources from a liquidity pool. Similar to contributions, this can be any of the three pool blueprints available in the pool package.   * `ValidatorStake`: A manifest where XRD is staked to one or more validators.   * `ValidatorUnstake`: A manifest where XRD is unstaked from one or more validators.   * `ValidatorClaim`: A manifest where XRD is claimed from one or more validators.   * `AccountDepositSettingsUpdate`: A manifest that updated the deposit settings of the account. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, candid::CandidType)]
pub enum ManifestClass {
    #[serde(rename = "General")]
    General,
    #[serde(rename = "Transfer")]
    Transfer,
    #[serde(rename = "PoolContribution")]
    PoolContribution,
    #[serde(rename = "PoolRedemption")]
    PoolRedemption,
    #[serde(rename = "ValidatorStake")]
    ValidatorStake,
    #[serde(rename = "ValidatorUnstake")]
    ValidatorUnstake,
    #[serde(rename = "ValidatorClaim")]
    ValidatorClaim,
    #[serde(rename = "AccountDepositSettingsUpdate")]
    AccountDepositSettingsUpdate,

}

impl ToString for ManifestClass {
    fn to_string(&self) -> String {
        match self {
            Self::General => String::from("General"),
            Self::Transfer => String::from("Transfer"),
            Self::PoolContribution => String::from("PoolContribution"),
            Self::PoolRedemption => String::from("PoolRedemption"),
            Self::ValidatorStake => String::from("ValidatorStake"),
            Self::ValidatorUnstake => String::from("ValidatorUnstake"),
            Self::ValidatorClaim => String::from("ValidatorClaim"),
            Self::AccountDepositSettingsUpdate => String::from("AccountDepositSettingsUpdate"),
        }
    }
}

impl Default for ManifestClass {
    fn default() -> ManifestClass {
        Self::General
    }
}

