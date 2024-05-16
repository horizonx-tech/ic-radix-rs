use candid::{export_service, CandidType, Deserialize};
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_radix_rs::apis::transaction_api::transaction_construction;
use serde::Serialize;

#[ic_cdk::update]
async fn example_transaction_construction() -> Response {
    let cfg = ic_radix_rs::apis::configuration::Configuration::new();
    let result = transaction_construction(&cfg).await.unwrap();
    Response {
        network: result.ledger_state.network,
        state_version: result.ledger_state.state_version,
        proposer_round_timestamp: result.ledger_state.proposer_round_timestamp,
        epoch: result.ledger_state.epoch,
        round: result.ledger_state.round,
    }
}

#[ic_cdk::query]
fn transform(args: TransformArgs) -> HttpResponse {
    args.response
}

#[derive(CandidType, Deserialize, Serialize)]
struct Response {
    /// The logical name of the network
    pub network: String,
    /// The state version of the ledger. Each transaction increments the state version by 1.
    pub state_version: i64,
    /// The proposer round timestamp of the consensus round when this transaction was committed to ledger. This is not guaranteed to be strictly increasing, as it is computed as an average across the validator set. If this is significantly behind the current timestamp, the Network Gateway is likely reporting out-dated information, or the network has stalled.
    pub proposer_round_timestamp: String,
    /// The epoch number of the ledger at this state version.
    pub epoch: i64,
    /// The consensus round in the epoch that this state version was committed in.
    pub round: i64,
}
export_service!();

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

mod test {

    use crate::__export_service;

    #[test]
    fn test_export() {
        std::fs::write("examples_backend.did", __export_service()).unwrap();
    }
}
