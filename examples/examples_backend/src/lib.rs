use candid::CandidType;
use ic_radix_rs::apis::transaction_api::transaction_construction;

#[ic_cdk::query]
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
#[derive(CandidType)]
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